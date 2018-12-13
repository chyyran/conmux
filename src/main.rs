// extern crate bytes;
// extern crate crossbeam;
// extern crate ctrlc;
// extern crate dunce;
// extern crate lazy_static;
// extern crate terminal_size;
// extern crate unicode_reader;
// extern crate widestring;
// extern crate winapi;

use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::exit;
use std::fs::OpenOptions;


mod buffer;
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
use self::surface::{Coord, Surface};
use self::wincon::*;

use terminal_size::{terminal_size, Height, Width};

#[allow(dead_code)]
#[allow(unused)]

fn main() {
    let term = Surface::new();

    let token = enable_console().unwrap();
    let mut ectx = EventContext::new(token);

    let mut pty =
        ConPty::new(&term.dimensions, "powershell", Some(&PathBuf::from("C:\\"))).unwrap();
    ectx.add_console(pty);
    ectx.sender(|tx| {
        let (Width(mut w), Height(mut h)) = terminal_size().unwrap();
        loop {
            let (Width(new_w), Height(new_h)) = terminal_size().unwrap();
            if (new_h != h) || (new_w != w) {
                w = new_w;
                h = new_h;
                tx.send(Action::PtyResize(
                    0,
                    Coord {
                        x: new_w as usize,
                        y: new_h as usize,
                    },
                ));
            }
        }
        Ok(())
    });

    ectx.handler(|ctx, action| {
        if let Action::Startup = action {
            ctx.set_active_console(0);
        }
        None
    });

    ectx.handler(|ctx, action| {
        if let Action::PtyDead(_) = action {
            restore_console();
            exit(0);
        }
        None
    });

    ectx.handler(|ctx, action| {
        let mut stdout = stdout();
        let mut lock = stdout.lock();
        if let Action::PtyOutReceived(_, c) = action {
            lock.write(&[c]);
            lock.flush();
        }
        None
    });

    ectx.handler(|ctx, action| {
        let mut file = OpenOptions::new().append(true).create(true).truncate(false).open("foo.txt").unwrap();
        if let Action::PtyOutReceived(_, c) = action {
            file.write(&[c]);
            file.flush();
        }
        None
    }); 
    listen_input(&mut ectx);
    register_console_handler(&mut ectx);
    ectx.start_event_loop();
}
