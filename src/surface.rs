use terminal_size::{terminal_size, Height, Width};
use std::clone::Clone;

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl From<(i16, i16)> for Coord {
    fn from(tuple: (i16, i16)) -> Coord {
        Coord {
            x: tuple.0 as usize,
            y: tuple.1 as usize,
        }
    }
}

impl From<&Coord> for Coord {
    fn from(c: &Coord) -> Coord {
        c.clone()
    }
}

pub struct Surface {
    pub dimensions: Coord,
}


impl Surface {
    pub fn new() -> Surface {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            Surface { dimensions: Coord { x: w as usize , y: h as usize } }
        } else {
            Surface { dimensions: Coord { x: 0, y: 0} }
        }
    }
}
