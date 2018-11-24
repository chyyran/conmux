use crate::event::{Action, EventContext};
use crate::pty::*;

use std::io::{stdin, Read, Write};



/// Will lock stdin.
pub fn listen_input<T>(ectx: &mut EventContext<T>)
where
    T: PseudoConsole<T>,
{
    ectx.sender(|tx| {
        let stdin = stdin();
        let mut lock = stdin.lock().bytes();
        while let Some(Ok(c)) = lock.next() {
            tx.send(Action::KeyInputReceived(c)).unwrap();
        }
        Ok(())
    });
}

pub fn register_console_handler<'a, T>(ectx: &mut EventContext<T>)
where
    T: PseudoConsole<T>,
{
    ectx.handler(|ctx, action| {
        if let Action::KeyInputReceived(key) = action {
            let writer = ctx.active_console_mut().writer();
            writer.write(&[key]).unwrap();
            writer.flush().unwrap();
        }
    })
}
