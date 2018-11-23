extern crate bytes;
extern crate crossbeam;
extern crate ctrlc;
extern crate dunce;
extern crate lazy_static;
extern crate terminal_size;
extern crate unicode_reader;
extern crate widestring;
extern crate winapi;

use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;
use std::thread;

mod conpty;
mod event_loop;
mod pipes;
mod pty;
mod surface;
mod wincon;

use self::conpty::*;
use self::event_loop::*;
use self::pty::*;
use self::surface::Surface;
use self::wincon::*;

#[allow(dead_code)]
#[allow(unused)]

fn main() {
    let term = Surface::new();

    let token = enable_console().unwrap();

    let mut context = Context::new(token);

    let mut pty =
        ConPty::new(&term.dimensions, "powershell", Some(&PathBuf::from("C:\\"))).unwrap();

    context.add_console(pty);
    context.set_active_console(0);
    context.active_console().start_shell();


    let rx = context.active_console().reader().clone();
    thread::spawn(move || {
        let mut stdout = stdout();
        let mut lock = stdout.lock();
        let mut rx = rx.bytes();
        lock.flush();

        while let Some(Ok(c)) = rx.next() { 
            lock.write(&[c]);
            lock.flush();
        }
    });

    run(&mut context);
}
