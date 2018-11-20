use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::wincon::{ENABLE_VIRTUAL_TERMINAL_PROCESSING, DISABLE_NEWLINE_AUTO_RETURN};
use winapi::um::processenv::GetStdHandle;
use winapi::um::winbase::{STD_OUTPUT_HANDLE, STD_INPUT_HANDLE};
use winapi::shared::minwindef::DWORD;

use std::io::{Write, Read};

use std::os::windows::io::FromRawHandle;
use std::fs::File;

pub fn enable_console() {

    let console_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    let mut console_mode: DWORD = 0;
    let _result = unsafe { GetConsoleMode(console_handle, &mut console_mode) };

    console_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;
    let _result = unsafe { SetConsoleMode(console_handle, console_mode) };
}

pub fn os_stdout() -> impl Write {
    let console_handle = unsafe { winapi::um::processenv::GetStdHandle(STD_OUTPUT_HANDLE) };
    unsafe { File::from_raw_handle(console_handle) } 
}


pub fn os_stdin() -> impl Read {
    let console_handle = unsafe { winapi::um::processenv::GetStdHandle(STD_INPUT_HANDLE) };
    unsafe { File::from_raw_handle(console_handle) } 
}