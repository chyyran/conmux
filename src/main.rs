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
mod event;
mod pipes;
mod pty;
mod surface;
mod wincon;

use self::conpty::*;
use self::context::*;
use self::event::*;
use self::pty::*;
use self::surface::Surface;
use self::wincon::*;

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
    ectx.add_console(pty);

    ectx.handler(|ctx, action| {
        if let Action::Startup = action {
            ctx.set_active_console(0);
            ctx.active_console().start_shell();
        }
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
