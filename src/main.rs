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
mod context;
mod pipes;
mod pty;
mod surface;
mod wincon;
mod event;

use self::conpty::*;
use self::context::*;
use self::pty::*;
use self::surface::Surface;
use self::wincon::*;
use self::event::*;

#[allow(dead_code)]
#[allow(unused)]

fn main() {
    let term = Surface::new();

    let token = enable_console().unwrap();

    let mut ectx = EventContext::new(token);

    // ectx.sender(||{
        
    // })
    let mut pty =
        ConPty::new(&term.dimensions, "powershell", Some(&PathBuf::from("C:\\"))).unwrap();

    ectx.context.add_console(pty);
    ectx.context.set_active_console(0);
    ectx.context.active_console().start_shell();

    let rx = ectx.context.active_console().reader().clone();
    ectx.sender(|tx| {
        let mut rx = rx.bytes();
        while let Some(Ok(c)) = rx.next() { 
            tx.send(Action::PtyOutReceived(0, c));
        }
        Ok(())
    });

    ectx.handler(|ctx, action| {
        let mut stdout = stdout();
        let mut lock = stdout.lock();
        if let Action::PtyOutReceived(_, c) = action {
           lock.write(&[c]);
           lock.flush();
        }
    });

    listen_input(&mut ectx);
    register_console_handler(&mut ectx);
    ectx.start_event_loop();
}
