use winapi::shared::minwindef::DWORD;
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use winapi::um::wincon::*;

use std::io::{Error, Result};

pub struct ConsoleEnabledToken;

pub fn enable_console() -> Result<ConsoleEnabledToken> {
    let console_out_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    let console_in_handle =  unsafe { GetStdHandle(STD_INPUT_HANDLE) };

    let mut console_out_mode: DWORD = 0;
    let mut console_in_mode: DWORD = 0;

    let result = unsafe { GetConsoleMode(console_out_handle, &mut console_out_mode) };

    if result == 0 {
        eprintln!("get console mode error");
        return Err(Error::last_os_error());
    }

    let result = unsafe {
        SetConsoleMode(
            console_out_handle,
            console_out_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN,
        )
    };
    if result == 0 {
        eprintln!("set console mode error when enable VT");
        return Err(Error::last_os_error());
    }

    let result = unsafe { GetConsoleMode(console_in_handle, &mut console_in_mode) };

    if result == 0 {
        eprintln!("get console mode error for input");
        return Err(Error::last_os_error());
    }

    let result = unsafe {
        SetConsoleMode(
            console_in_handle,
            (console_in_mode & !ENABLE_ECHO_INPUT & !ENABLE_LINE_INPUT) | ENABLE_VIRTUAL_TERMINAL_INPUT
        )
    };

    if result == 0 {
        eprintln!("set console mode error for echo input disable");
        return Err(Error::last_os_error());
    }

    Ok(ConsoleEnabledToken)
}
