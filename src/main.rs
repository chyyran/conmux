extern crate bytes;
extern crate crossbeam;
extern crate dunce;
extern crate lazy_static;
extern crate terminal_size;
extern crate unicode_reader;
extern crate widestring;
extern crate winapi;
extern crate ctrlc;

use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread;

use bytes::BytesMut;
use unicode_reader::CodePoints;

mod conpty;
mod pipes;
mod surface;
mod wincon;

use self::conpty::*;
use self::surface::Surface;
use self::wincon::*;

#[allow(dead_code)]
#[allow(unused)]

fn main() {
    let term = Surface::new();

    let token = enable_console().unwrap();

    let mut pty =
        ConPty::new(&term.dimensions, "powershell", Some(&PathBuf::from("C:\\")), token).unwrap();

    pty.start_shell().unwrap();

    let (tx, rx) = channel();
    let mut readpipe = pty.pipes.0.clone();
    thread::spawn(move || {
        let mut lock = readpipe.bytes();
        for c in lock {
            if let Ok(c) = c {
                tx.send(c);
            }
        }
        println!("listen thread quit");
    });

    thread::spawn(move || {
        let mut stdout = stdout();
        let mut lock = stdout.lock();
        lock.flush();
        loop {
            let j = rx.recv().unwrap();

            lock.write(&[j]);
            lock.flush();
        }
    });

    let mut stdin = stdin();
    let mut lock = CodePoints::from(stdin.lock().bytes());
    let mut buf = BytesMut::new();

    while let Some(Ok(c)) = lock.next() {
        //todo: handle cursor sequences.
        let utf8_len = c.len_utf8();
        buf.resize(utf8_len, 0);
        c.encode_utf8(&mut buf[..]);
        pty.pipes.1.write(&buf);
        pty.pipes.1.flush();
    }
}
