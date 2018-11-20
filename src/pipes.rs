use std::fs::File;
use std::io::{Error, Read, Result, Write};

use std::os::windows::io::{FromRawHandle, IntoRawHandle};
use std::ptr::null_mut;

use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::namedpipeapi::CreatePipe;
use winapi::um::winnt::HANDLE;

pub trait IntoHandle {
    fn into_handle(self) -> PipeHandle;
}

pub struct SyncPipeIn {
    f_in: File,
}

pub struct SyncPipeOut {
    f_out: File,
}

pub struct PipeHandle(pub HANDLE);

impl IntoHandle for SyncPipeIn {
    fn into_handle(self) -> PipeHandle {
        PipeHandle(self.f_in.into_raw_handle())
    }
}

impl IntoHandle for SyncPipeOut {
    fn into_handle(self) -> PipeHandle {
        PipeHandle(self.f_out.into_raw_handle())
    }
}

impl Drop for PipeHandle {
    fn drop(&mut self) {
        unsafe {
            if INVALID_HANDLE_VALUE != self.0 {
                CloseHandle(self.0);
            }
        }
    }
}

impl Read for SyncPipeIn {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>
    {
        self.f_in.read(buf)
    }
}

impl Write for SyncPipeOut {
    fn write(&mut self, buf: &[u8]) -> Result<usize>
    {
        self.f_out.write(buf)
    }

    fn flush(&mut self) -> Result<()>
    {
        self.f_out.flush()
    }
}

pub fn create_sync_pipe() -> Result<(SyncPipeIn, SyncPipeOut)> {
    let mut pipe_in: HANDLE = null_mut();
    let mut pipe_out: HANDLE = null_mut();

    let result = unsafe { CreatePipe(&mut pipe_in, &mut pipe_out, null_mut(), 0) };

    // As per MSDN docs, CreatePipe returns a non-zero on success.
    if result == 0 {
        Err(Error::last_os_error())
    } else {
        // immediately consume the pipes as rust File handles.
        let f_in = unsafe { File::from_raw_handle(pipe_in) };
        let f_out = unsafe { File::from_raw_handle(pipe_out) };

        Ok((SyncPipeIn { f_in }, SyncPipeOut { f_out }))
    }
}
