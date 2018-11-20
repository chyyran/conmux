extern crate bytes;
extern crate crossbeam;
extern crate lazy_static;
extern crate widestring;
extern crate winapi;
extern crate dunce;
extern crate terminal_size;

use std::io::{stdout, Read, Write};
use std::sync::mpsc::sync_channel;
use std::thread;
use std::path::PathBuf;


mod conpty;
mod pipes;
mod surface;

use self::conpty::*;
use self::surface::Surface;
#[allow(dead_code)]
#[allow(unused)]
fn main() {

    let term = Surface::new();

    let mut pty = ConPty::new(&term.dimensions, "powershell", Some(&PathBuf::from("C:\\"))).unwrap();
    pty.start_shell().unwrap();

    pty.pipes.1.write(b"ping localhost");
    pty.pipes.1.write(b"\r");

    let (tx, rx) = sync_channel(1);

    thread::spawn(move || {
        let mut buffer = vec![0; 1];
        loop {
            buffer.clear();
            buffer.resize(1, 0);
            let readbytes = pty.pipes.0.read(&mut buffer).unwrap();
            if readbytes > 0 {
                tx.send(buffer.clone());
            }

            // let send_str = String::from_utf8(buffer.clone()).unwrap();
        }
        println!("listen thread quit");
    });

    let mut stdout = stdout();
    loop {
        let j = rx.recv().unwrap();
        stdout.flush();
        stdout.write_all(&j);
    }
}
