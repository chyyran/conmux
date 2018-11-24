use crate::surface::Coord;
use std::convert::{AsMut, AsRef};
use std::io::{Read, Result, Write};
use std::ops::{Deref, DerefMut};

pub trait KeepAlive: Send + Sync + Clone {
    fn dead(&self) -> bool;
}

pub trait PseudoConsole<T>: Send + Sync
where
    T: 'static,
{
    type Reader: Read + Send + Sync + Clone + 'static;
    type Writer: Write + Send + Sync + 'static;
    type KeepAlive: KeepAlive + 'static;

    fn dimensions(&self) -> &Coord;
    fn resize(&mut self, coord: &Coord) -> Result<&Coord>;
    fn start_shell(&mut self) -> Result<()>;
    fn writer(&mut self) -> &mut Self::Writer;
    fn reader(&self) -> &Self::Reader;
    fn keep_alive(&self) -> Self::KeepAlive;
}

pub struct BufferedPseudoConsole<T>
where
    T: PseudoConsole<T> + 'static,
    T: 'static,
{
    console: T,
}

impl<T> BufferedPseudoConsole<T>
where
    T: PseudoConsole<T> + 'static,
    T: 'static,
{
    pub fn new(console: T) -> BufferedPseudoConsole<T> {
        BufferedPseudoConsole { console }
    }
}

impl<T> AsMut<T> for BufferedPseudoConsole<T>
where
    T: PseudoConsole<T> + 'static,
    T: 'static,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.console
    }
}

impl<T> AsRef<T> for BufferedPseudoConsole<T>
where
    T: PseudoConsole<T> + 'static,
    T: 'static,
{
    fn as_ref(&self) -> &T {
        &self.console
    }
}

impl<T> Deref for BufferedPseudoConsole<T>
where
    T: PseudoConsole<T> + 'static,
    T: 'static,
{
    type Target = T;
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T> DerefMut for BufferedPseudoConsole<T>
where
    T: PseudoConsole<T> + 'static,
    T: 'static,
{
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}
