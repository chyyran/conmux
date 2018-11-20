use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use std::mem::size_of;
use std::ptr::{null, null_mut};

use winapi::shared::basetsd::{PSIZE_T, SIZE_T};
use winapi::shared::minwindef::BYTE;
use winapi::shared::ntdef::{LPCWSTR, LPWSTR};
use winapi::shared::winerror::S_OK;
use winapi::um::consoleapi;
use winapi::um::processthreadsapi::{
    CreateProcessW, InitializeProcThreadAttributeList, UpdateProcThreadAttribute,
    PROCESS_INFORMATION, STARTUPINFOW,
};
use winapi::um::winbase::{EXTENDED_STARTUPINFO_PRESENT, STARTUPINFOEXW};
use winapi::um::wincon::{COORD, HPCON};

use dunce::canonicalize;
use widestring::U16CString;

use crate::pipes::*;

#[derive(Clone)]
pub struct Coord {
    x: i16,
    y: i16,
}

impl From<(i16, i16)> for Coord {
    fn from(tuple: (i16, i16)) -> Coord {
        Coord {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl Into<COORD> for Coord {
    fn into(self) -> COORD {
        COORD {
            X: self.x as _,
            Y: self.y as _,
        }
    }
}

pub struct PtyHandle(HPCON);
unsafe impl Send for PtyHandle {}
unsafe impl Sync for PtyHandle {}

pub struct ConPty {
    pty_handle: PtyHandle,
    pub size: Coord,
    shell: String,
    pwd: Option<PathBuf>,
    pub pipes: (SyncPipeIn, SyncPipeOut),
}

impl ConPty {
    pub fn new(
        coord: impl Into<Coord>,
        shell: impl Into<String>,
        pwd: Option<&Path>,
    ) -> Result<ConPty> {
        let coord = coord.into();
        let (hpipe_in, ph_pipe_out) = create_sync_pipe().unwrap();
        let (ph_pipe_in, hpipe_out) = create_sync_pipe().unwrap();
        match ConPty::create_pseudo_console(coord.clone(), (hpipe_in, hpipe_out)) {
            Err(err) => Err(err),
            Ok(pty_handle) => Ok(ConPty {
                pty_handle: PtyHandle(pty_handle),
                shell: shell.into(),
                size: coord,
                pwd: pwd.map(|p| p.to_owned()),
                pipes: (ph_pipe_in, ph_pipe_out),
            }),
        }
    }

    #[must_use]
    pub fn start_shell(&self) -> Result<()> {

        // Most of this code was ripped from
        // https://github.com/davidhewitt/alacritty/blob/consoleapi/src/tty/windows/conpty.rs
        // Essentially this hooks up the shell and points it at the pseudo pty started by create_pseudo_console.

        let mut size: SIZE_T = 0;

        let mut startup_info_ex: STARTUPINFOEXW = Default::default();
        startup_info_ex.StartupInfo.cb = size_of::<STARTUPINFOEXW>() as u32;

        // Create the appropriately sized thread attribute list.
        unsafe {
            let failure =
                InitializeProcThreadAttributeList(null_mut(), 1, 0, &mut size as PSIZE_T) > 0;

            // This call was expected to return false.
            if failure { return Err(Error::last_os_error()); } 
        }

        let mut attr_list: Box<[BYTE]> = vec![0; size].into_boxed_slice();

        // Set startup info's attribute list & initialize it
        startup_info_ex.lpAttributeList = attr_list.as_mut_ptr() as _;
        unsafe {
            let success = InitializeProcThreadAttributeList(
                startup_info_ex.lpAttributeList,
                1,
                0,
                &mut size as PSIZE_T,
            ) > 0;

            if !success { return Err(Error::last_os_error()); } 
        }

        // Set thread attribute list's Pseudo Console to the specified ConPTY
        unsafe {
            let success = UpdateProcThreadAttribute(
                startup_info_ex.lpAttributeList,
                0,
                22 | 0x0002_0000, // PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE
                self.pty_handle.0,
                size_of::<HPCON>(),
                null_mut(),
                null_mut(),
            ) > 0;

            if !success { return Err(Error::last_os_error()); } 
        }

        let mut proc_info: PROCESS_INFORMATION = Default::default();
        let command = U16CString::from_str(&self.shell)
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Path to shell was invalid"))?
            .into_raw();

        let cwd_ptr = match &self.pwd {
            None => null(),
            Some(pwd) => match canonicalize(&pwd) {
                Ok(pwd) => {
                    let pwd_str = &pwd.to_str();
                    if pwd_str.is_none() {
                        null()
                    } else {
                        U16CString::from_str(pwd_str.unwrap())
                            .map_err(|_| {
                                Error::new(ErrorKind::InvalidInput, "Path to shell was invalid")
                            })?
                            .into_raw() as LPCWSTR
                    }
                }
                Err(_) => null(),
            },
        };

        unsafe {
            let success = CreateProcessW(
                null(),
                command as LPWSTR,
                null_mut(),
                null_mut(),
                false as i32,
                EXTENDED_STARTUPINFO_PRESENT,
                null_mut(),
                cwd_ptr,
                &mut startup_info_ex.StartupInfo as *mut STARTUPINFOW,
                &mut proc_info as *mut PROCESS_INFORMATION,
            ) > 0;

            if !success { return Err(Error::last_os_error()); }
        }

        Ok(())
    }

    fn create_pseudo_console(
        coord: impl Into<Coord>,
        pipes: (SyncPipeIn, SyncPipeOut),
    ) -> Result<HPCON> {
        let mut pty_handle = 0 as HPCON;
        let result = unsafe {
            consoleapi::CreatePseudoConsole(
                coord.into().into(),
                pipes.0.into_handle().0,
                pipes.1.into_handle().0,
                0,
                &mut pty_handle as *mut HPCON,
            )
        };

        if result != S_OK {
            Err(Error::last_os_error())
        } else {
            Ok(pty_handle)
        }
    }
}

impl Drop for ConPty {
    // todo: check for pseudoconsole already closed.
    fn drop(&mut self) {
        unsafe {
            consoleapi::ClosePseudoConsole(self.pty_handle.0);
        }
    }
}
