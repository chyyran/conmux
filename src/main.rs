extern crate bytes;
extern crate crossbeam;
extern crate dunce;
extern crate lazy_static;
extern crate terminal_size;
extern crate widestring;
extern crate winapi;

use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread;

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

    enable_console();

    let mut pty =
        ConPty::new(&term.dimensions, "powershell", Some(&PathBuf::from("C:\\"))).unwrap();
    pty.start_shell().unwrap();

    // pty.pipes.1.write(b"ping localhost");
    // pty.pipes.1.write(b"\r");

    let (tx, rx) = channel();
    let mut readpipe = pty.pipes.0.clone();
    thread::spawn(move || {
        let mut buffer = vec![0; 1];
        loop {
            buffer.clear();
            buffer.resize(1, 0);
            let readbytes = readpipe.read(&mut buffer).unwrap();
            tx.send(buffer.clone());
        }
        println!("listen thread quit");
    });

    thread::spawn(move || {
        let mut stdin = stdin();
        for c in stdin.lock().bytes() {
            pty.pipes.1.write(&[c.unwrap()]);
            pty.pipes.1.flush();
        }
    });

    let mut stdout = stdout();
    stdout.lock().flush();
    loop {
        let j = rx.recv().unwrap();
        stdout.lock().write_all(&j);
        stdout.lock().flush();
    }
   

    // while let Ok(_) = stdin.read(&mut input) {
    //   //  if (&input[0] == &b"\r"[0]) { continue; }
    //     pty.pipes.1.write_all(&input);
    //     pty.pipes.1.flush();

    //     input.clear();
    //     input.resize(1, 0);
    // }
}
