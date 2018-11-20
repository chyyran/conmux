use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::wincon::{ENABLE_VIRTUAL_TERMINAL_PROCESSING, DISABLE_NEWLINE_AUTO_RETURN};
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::winnt::HANDLE;
use winapi::shared::minwindef::DWORD;

use std::ptr::null_mut;

pub fn enable_console() {

    let console_handle = unsafe { winapi::um::processenv::GetStdHandle(STD_OUTPUT_HANDLE) };
    let mut console_mode: DWORD = 0;
    let result = unsafe { winapi::um::consoleapi::GetConsoleMode(console_handle, &mut console_mode) };

    console_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING | DISABLE_NEWLINE_AUTO_RETURN;
    let result = unsafe { SetConsoleMode(console_handle, console_mode) };
}


