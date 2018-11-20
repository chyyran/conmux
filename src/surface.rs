use terminal_size::{terminal_size, Height, Width};
use std::clone::Clone;

#[derive(Clone)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

impl From<(i16, i16)> for Coord {
    fn from(tuple: (i16, i16)) -> Coord {
        Coord {
            x: tuple.0,
            y: tuple.1,
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
            Surface { dimensions: Coord { x: w as i16, y: h as i16} }
        } else {
            Surface { dimensions: Coord { x: 0, y: 0} }
        }
    }
}
