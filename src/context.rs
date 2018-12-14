use crate::ansitypes::*;
use crate::event::{Action, Column, EventContext, Line};
use crate::pty::*;

use crossbeam::channel::Sender;
use ansi_escapes;
use std::io::{stdin, Read, Write};
use vte::Perform;

struct InputPerformer {
    tx: Sender<Action>,
}

impl Perform for InputPerformer {
    fn print(&mut self, c: char) {
        if c == C0::DEL.into() {
            self.tx.send(Action::HostBackspace).unwrap();
        } else {
            self.tx.send(Action::HostInput(c)).unwrap();
        }
    }

    fn execute(&mut self, byte: u8) {
        let action = match byte {
            C0::HT => Action::HostPutTabs(1),
            C0::BS => Action::HostBackspace,
            C0::CR => Action::HostCarriageReturn,
            C0::LF | C0::VT | C0::FF => Action::HostLineFeed,
            C0::SUB => Action::HostSubtitute,
            C1::NEL => Action::HostNewline,
            _ => { println!("{} [unhandled] execute", byte); Action::HostControlInput(byte) },
        };

        self.tx.send(action).unwrap();
    }

    fn hook(&mut self, params: &[i64], intermediates: &[u8], ignore: bool) {}

    fn put(&mut self, byte: u8) {}

    fn unhook(&mut self) {}

    fn osc_dispatch(&mut self, params: &[&[u8]]) {}

    fn csi_dispatch(&mut self, params: &[i64], intermediates: &[u8], ignore: bool, c: char) {
        macro_rules! arg_or_default {
            (idx: $idx:expr, default: $default:expr) => {
                params
                    .get($idx)
                    .and_then(|v| if *v == 0 { None } else { Some(*v) })
                    .unwrap_or($default)
            };
        }

        let action = match c {
            'C' | 'a' => {
                Action::HostCursorMoveForward(Column(arg_or_default!(idx: 0, default: 1) as usize))
            }
            'D' => {
                Action::HostCursorMoveBackward(Column(arg_or_default!(idx: 0, default: 1) as usize))
            }
            'A' => {
                Action::HostCursorMoveUp(Line(arg_or_default!(idx: 0, default: 1) as usize))
            }
            'B' => {
                Action::HostCursorMoveUp(Line(arg_or_default!(idx: 0, default: 1) as usize))
            }
            '~' => {
                match params[0] {
                    3 => Action::HostDeleteKey,
                    _ => Action::Noop
                }
            }
            _ => { println!("{}: [unhandled] csi", c); Action::Noop },
        };

        self.tx.send(action).unwrap();
    }

    fn esc_dispatch(&mut self, params: &[i64], intermediates: &[u8], ignore: bool, byte: u8) {}
}

/// Will lock stdin.
pub fn listen_input<T>(ectx: &mut EventContext<T>)
where
    T: PseudoConsole<T>,
{
    ectx.sender(|tx| {
        let stdin = stdin();
        let mut lock = stdin.lock().bytes();
        let mut sender = InputPerformer { tx: tx.clone() };
        let mut parser = vte::Parser::new();
        while let Some(Ok(c)) = lock.next() {
            // tx.send(Action::KeyInputReceived(c)).unwrap();
            parser.advance(&mut sender, c);
        }
        Ok(())
    });
}

pub fn register_console_handler<'a, T>(ectx: &mut EventContext<T>)
where
    T: PseudoConsole<T>,
{
    ectx.handler(|ctx, action| {
        let mut buf = vec![0; 4];
        let writer = ctx.active_console_mut().writer();

        match action {
            Action::HostInput(c) => {
                buf.clear();
                buf.resize(c.len_utf8(), 0);
                c.encode_utf8(&mut buf);
                writer.write(&buf).unwrap();
            }

            Action::HostControlInput(c) => {
                writer.write(&[c]).unwrap();
            }

            Action::HostCarriageReturn => {
                writer.write(&[C0::CR]).unwrap();
            }

            Action::HostLineFeed => {
                writer.write(&[C0::LF]).unwrap();
            }

            Action::HostBackspace => {
                writer.write(&[C0::BS]).unwrap();
            }

            Action::HostCursorMoveForward(cols) => {
                let sequence = format!("{}", ansi_escapes::CursorForward(cols.0 as u16)).into_bytes();
                writer.write(&sequence).unwrap();
            }

            Action::HostCursorMoveBackward(cols) => {
                let sequence = format!("{}", ansi_escapes::CursorBackward(cols.0 as u16)).into_bytes();
                writer.write(&sequence).unwrap();
            }

            Action::HostCursorMoveUp(rows) => {
                let sequence = format!("{}", ansi_escapes::CursorUp(rows.0 as u16)).into_bytes();
                writer.write(&sequence).unwrap();
            }

            Action::HostCursorMoveDown(rows) => {
                let sequence = format!("{}", ansi_escapes::CursorDown(rows.0 as u16)).into_bytes();
                writer.write(&sequence).unwrap();
            }
            Action::HostDeleteKey => {
                let sequence = String::from("\x1b[3~").into_bytes();
                writer.write(&sequence).unwrap();
            }
            _ => (),
        }

        writer.flush().unwrap();
        None
    })
}
