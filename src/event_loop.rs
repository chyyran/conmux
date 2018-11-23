use crate::pty::*;
use crate::wincon::ConsoleEnabledToken;

use std::io::{stdin, Read, Write};
use std::io::{Error, ErrorKind, Result};

use std::thread;

use crossbeam::channel::{select, unbounded, bounded, Receiver};

pub struct Context<T>
where
    T: PseudoConsole<T>,
{
    consoles: Vec<BufferedPseudoConsole<T>>,
    active_console: usize,
}

pub enum Action {
    InputCharacter(char),
    ControlCode(char),
}

impl<T> Context<T>
where
    T: PseudoConsole<T>,
{
    pub fn new(_: ConsoleEnabledToken) -> Context<T> {
        Context {
            consoles: Vec::new(),
            active_console: 0,
        }
    }

    pub fn add_console(&mut self, console: T) {
        self.consoles.push(BufferedPseudoConsole::new(console));
    }


    pub fn set_active_console(&mut self, idx: usize) -> Result<()> {
        if let Some(_) = self.consoles.get(idx) {
            self.active_console = idx;
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::NotFound,
                "Console in index not found",
            ))
        }
    }

    pub fn console(&self, i: usize) -> Option<&T> {
        self.consoles.get(i).and_then(|c| Some(c.as_ref()))
    }

     pub fn console_mut(&mut self, i: usize) -> Option<&mut T> {
        self.consoles.get_mut(i).and_then(|c| Some(c.as_mut()))
    }

    pub fn active_console_mut(&mut self) -> &mut T {
        self.consoles[self.active_console].as_mut()
    }

    pub fn active_console(&self) -> &T {
        self.consoles[self.active_console].as_ref()
    }
}

/// Will lock stdin.
pub fn listen_input(quit: Receiver<()>) -> Receiver<u8> {
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        let stdin = stdin();
        let mut lock = stdin.lock().bytes();
        loop {
            select! {
               recv(quit) -> _ => break,
               default => {
                   if let Some(Ok(c)) = lock.next() {
                       tx.send(c).unwrap();
                   }
               }
            }
        }
    });
    rx
}

pub fn run<T>(ctx: &mut Context<T>)
where
    T: PseudoConsole<T>,
{
    let (quit_tx, quit) = bounded(1);
    let input_rx = listen_input(quit);
    loop {
        if let Ok(next_key) = input_rx.recv() {
            let writer = ctx.active_console_mut().writer();
            writer.write(&[next_key]).unwrap();
            writer.flush().unwrap();
        } else {
            quit_tx.send(()).unwrap();
            break
        }
    }

}
