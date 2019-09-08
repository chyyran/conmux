use crate::pty::*;
use crate::wincon::ConsoleEnabledToken;
use crate::surface::Coord;
use crate::ansitypes::*;

use crossbeam::channel::*;

use std::io::{stdin, Read, Write};
use std::io::{Error, ErrorKind, Result};
use std::marker::PhantomData;

use std::sync::{Arc, Weak};
use std::process::exit;
use std::ops::FnOnce;
use std::thread::{self as thread, JoinHandle};

#[derive(Debug, Copy, Clone)]
pub struct PtyIndex(pub usize);
#[derive(Debug, Copy, Clone)]
pub struct Line(pub usize);
#[derive(Debug, Copy, Clone)]
pub struct Column(pub usize);

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Action {
    KeyInputReceived(u8),
    HostInput(char),
    HostControlInput(u8),
    HostDeleteKey,
    HostInsertKey,
    HostPageUpKey,
    HostPageDownKey,
    HostInsertBlank(Column),
    HostPutTabs(i64),
    HostBackspace,
    HostCarriageReturn,
    HostLineFeed,
    HostNewline,
    HostInsertBlankLines(Line),
    HostDeleteLines(Line),
    HostEraseCharacters(Column),
    HostDeleteCharacters(Column),
    HostClearLine(LineClearMode),
    HostClearScreen(ClearMode),
    HostClearTabs(TabulationClearMode),
    HostSubtitute,
    
    HostMouseInputReceived(u8),
    HostCursorGoto(Line, Column),
    HostCursorMoveUp(Line),
    HostCursorMoveDown(Line),
    HostCursorMoveForward(Column),
    HostCursorMoveBackward(Column),
    HostCursorMoveForwardTabs(Column),
    HostCursorMoveBackwardsTabs(Column),
    HostCursorMoveUpAndCarriageReturn(Column),
    HostCursorMoveDownAndCarriageReturn(Column),
    HostCursorSavePosition,
    HostCursorRestorePosition,
    
    PtyOutReceived(usize, u8),

    // rename these (not actually PTY)
    PtyActiveChange(usize),
    PtyDead(usize),
    PtyResize(usize, Coord),

    // All these actions come from PTY itself.

    // Direct buffer manipulation
    PtyOutput(PtyIndex, char),
    PtyInsertBlank(PtyIndex, Column),
    PtyPutTabs(PtyIndex, i64),
    PtyBackspace(PtyIndex),
    PtyCarriageReturn(PtyIndex),
    PtyLineFeed(PtyIndex),
    PtyNewline(PtyIndex),
    PtyInsertBlankLines(PtyIndex, Line),
    PtyDeleteLines(PtyIndex, Line),
    PtyEraseCharacters(PtyIndex, Column),
    PtyDeleteCharacters(PtyIndex, Column),
    PtyClearLine(PtyIndex, LineClearMode),
    PtyClearScreen(PtyIndex, ClearMode),
    PtyClearTabs(PtyIndex, TabulationClearMode),
    PtySubtitute(PtyIndex),

    PtyBell(PtyIndex),
    PtySetHorizontalTabstop(PtyIndex),
    PtyVtScrollUp(PtyIndex, Line),
    PtyVtScrollDown(PtyIndex, Line),


    PtyReset(PtyIndex),

    // Cursor manipulation
    PtyCursorGoto(PtyIndex, Line, Column),
    PtyCursorMoveUp(PtyIndex, Line),
    PtyCursorMoveDown(PtyIndex, Line),
    PtyCursorMoveForward(PtyIndex, Column),
    PtyCursorMoveBackward(PtyIndex, Column),
    PtyCursorMoveForwardTabs(PtyIndex, Column),
    PtyCursorMoveBackwardsTabs(PtyIndex, Column),
    PtyCursorMoveUpAndCarriageReturn(PtyIndex, Column),
    PtyCursorMoveDownAndCarriageReturn(PtyIndex, Column),
    PtyCursorSavePosition(PtyIndex),
    PtyCursorRestorePosition(PtyIndex),

    // Unsupported
    PtySetCursorStyle(PtyIndex),
    PtySetKeypadApplicationMode(PtyIndex),
    PtyUnsetKeypadApplicationMode(PtyIndex),
    PtySetActiveCharset(PtyIndex, CharsetIndex),
    PtyConfigureCharset(PtyIndex, CharsetIndex, StandardCharset),
    PtySetColor(PtyIndex, usize, (u8, u8, u8)),
    PtyResetColor(PtyIndex, usize),
    // PtySetClipboard(PtyIndex, Box<String>),
    PtyDectest(PtyIndex),
    
    PtyReverseIndex(PtyIndex),
    PtyTerminalAttribute(PtyIndex, Attr),
    PtySetMode(PtyIndex, Mode),
    PtyUnsetMode(PtyIndex, Mode),
    // PtySetScrollingRegion(PtyIndex, Range<Line>),
    Noop,
    Startup,
    ModeChange,
}


pub struct Context<'a, T>
where
    T: PseudoConsole<T>,
    T: 'static,
{
    consoles: Vec<BufferedPseudoConsole<T>>,
    active_console: usize,
    _pd: PhantomData<&'a T>,
}

impl<'a, T> Context<'a, T>
where
    T: PseudoConsole<T>,
    T: 'static,
{
    pub fn new(_: ConsoleEnabledToken) -> Context<'a, T> {
        Context {
            consoles: Vec::new(),
            active_console: 0,
            _pd: PhantomData,
        }
    }

    fn add_console(&mut self, console: T) -> &mut BufferedPseudoConsole<T> {
        self.consoles.push(BufferedPseudoConsole::new(console));
        self.consoles.last_mut().unwrap()
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

    pub fn delete_console(&mut self, idx: usize) {
        self.consoles.remove(idx);
        // handle idx 0
        // handle active_console -1
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

pub struct EventContext<'a, T>
where
    T: PseudoConsole<T>,
    T: 'static,
{
    receivers: Vec<Receiver<Action>>,
    handlers: Vec<Box<FnMut(&mut Context<T>, Action) -> Option<()> + 'a>>,
    context: Context<'a, T>,
}

pub type QuitSignal = Sender<()>;

impl<'a, T> EventContext<'a, T>
where
    T: PseudoConsole<T>,
{
    pub fn new(t: ConsoleEnabledToken) -> EventContext<'a, T> {
        EventContext {
            receivers: Vec::new(),
            handlers: Vec::new(),
            context: Context::new(t),
        }
    }

    pub fn handler<F>(&mut self, f: F)
    where
        F: FnMut(&mut Context<T>, Action) -> Option<()>,
        F: 'a,
    {
        self.handlers.push(Box::new(f));
    }

    pub fn add_console(&mut self, console: T) {
        let console = self.context.add_console(console);
        console.start_shell().unwrap();
        let reader = console.as_ref().reader().clone();
        let ka = console.as_ref().keep_alive();

        self.sender(|tx| {
            let mut rx = reader.bytes();
            while let Some(Ok(c)) = rx.next() {
                tx.send(Action::PtyOutReceived(0, c)).unwrap();
            }
            Ok(())
        });

        self.sender(move |tx| {
            loop {
                if ka.dead() {
                    tx.send(Action::PtyDead(0)).unwrap();
                    break;
                }
            }
            Ok(())
        });

        self.handler(|ctx, action| {
            if let Action::PtyResize(0, c) = action {
                ctx.console_mut(0)?.resize(&c).unwrap();
            }
            None
        });
    }

    pub fn sender<F>(&mut self, f: F) -> (JoinHandle<Result<()>>, QuitSignal)
    where
        F: FnOnce(Sender<Action>) -> Result<()>,
        F: Send + 'static,
    {
        let (tx, rx) = unbounded();
        let (qtx, qrx) = bounded(1);
        self.receivers.push(rx.clone());

        (thread::spawn(move || f(tx.clone())), qtx)
    }

    fn next(&self) -> Option<Action> {
        if self.receivers.len() == 0 {
            return None;
        }
        let mut select = Select::new();
        let mut select_col = Vec::new();
        for recv in self.receivers.iter() {
            select_col.push((select.recv(recv), recv));
        }
        let oper = select.select();
        let index = oper.index();
        // todo: no need to make this n d we?
        // binary search?
        for (i, r) in select_col {
            if i == index {
                return oper.recv(r).ok();
            }
        }
        None
    }

    pub fn start_event_loop(&mut self) {
        self.sender(|tx| Ok(tx.send(Action::Startup).unwrap()));

        loop {
            if let Some(action) = self.next() {
                for f in self.handlers.iter_mut() {
                    f(&mut self.context, action);
                }
            }
        }
    }
}
