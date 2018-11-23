use crate::surface::Coord;
use std::convert::{AsMut, AsRef};
use std::io::{Read, Result, Write};

pub trait PseudoConsole<T>: Send + Sync {
    type Reader: Read + Send + Sync + Clone;
    type Writer: Write + Send + Sync;
    fn dimensions(&self) -> &Coord;
    fn resize(&mut self, coord: &Coord) -> Result<&Coord>;
    fn start_shell(&self) -> Result<()>;
    fn writer(&mut self) -> &mut Self::Writer;
    fn reader(&self) -> &Self::Reader;
}

pub struct BufferedPseudoConsole<T>
where
    T: PseudoConsole<T>,
{
    console: T
}

impl<T> BufferedPseudoConsole<T>
where
    T: PseudoConsole<T>,
{
    pub fn new(console: T) -> BufferedPseudoConsole<T> {
        BufferedPseudoConsole { console }
    }
}

impl<T> AsMut<T> for BufferedPseudoConsole<T>
where
    T: PseudoConsole<T>,
{
    fn as_mut(&mut self) -> &mut T {
        &mut self.console
    }
}

impl<T> AsRef<T> for BufferedPseudoConsole<T>
where
    T: PseudoConsole<T>,
{
    fn as_ref(&self) -> &T {
        &self.console
    }
}
