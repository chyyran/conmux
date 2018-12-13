use crate::surface::Coord;

const CSI: &'static str = "\x1b[";
const OSC: &'static str = "\x1b]";

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Extended(usize, usize, usize),
    Default,
}

#[derive(Clone, Copy)]
pub enum Formatting {
    Default,
    Bright,
    Underline,
    NoUnderline,
    Negative,
    Positive,
    ForegroundColor(Color),
    BrightForegroundColor(Color),
    BackgroundColor(Color),
    BrightBackgroundColor(Color),
}

#[derive(Clone)]
pub struct Cell {
    c: Vec<CellContents>,
}

impl Default for Cell {
    fn default() -> Cell {
        Cell {
            c: vec![CellContents::Empty; 1],
        }
    }
}

#[derive(Clone, Copy)]
pub enum CellContents {
    Empty,
    Character(char),
    Format(Formatting),
}

pub enum CursorDirection {
    Forward,
    Backward,
    Up,
    Down,
    Position(usize, usize),
}

#[derive(Clone)]
pub struct Row {
    inner: Vec<Cell>,
    width: usize,
}

impl Row {
    fn new(dimensions: Coord) -> Row {
        Row {
            inner: vec![Cell::default(); dimensions.x],
            width: dimensions.x,
        }
    }

    fn clear(&mut self, c: Cell) {
        self.inner = vec![Cell::default(); self.width];
    }

    fn write(&mut self, c: Cell, cursor: usize) {
        if cursor < self.inner.len() {
            // cursor is at most len - 1
            self.inner[cursor] = c;
        } else {
            // cursor is at least len here.
            for _ in self.inner.len()..cursor {
                self.inner.push(Cell::default());
            }
            self.inner.push(c);
        }
    }
}

pub struct Buffer {
    buffer: Vec<Row>,
    cursor: Coord,
    dimensions: Coord,
}

impl Buffer {
    fn new(dimensions: Coord) -> Buffer {
        Buffer {
            buffer: vec![Row::new(dimensions); dimensions.y],
            cursor: (0, 0).into(),
            dimensions: dimensions,
        }
    }

    fn push_cell(&mut self, c: Cell) {
        let row = &mut self.buffer[self.cursor.y];
        row.write(c, self.cursor.x);
        self.cursor.x += 1;
    }

    fn push_newline(&mut self, c: Cell) {
        self.cursor.x = 0;
        self.cursor.y += 1;
        if self.cursor.y > self.buffer.len() {
            self.buffer.push(Row::new(self.dimensions));
        }
    }

    fn set_cursor(&mut self, c: CursorDirection) {
        match c {
            CursorDirection::Forward => {
                if self.cursor.x < self.dimensions.x {
                    self.cursor.x += 1;
                }
            }
            CursorDirection::Backward => {
                if self.cursor.x > 0 {
                    self.cursor.x -= 1;
                }
            }
            CursorDirection::Up => {
                if self.cursor.y > 0 {
                    self.cursor.y -= 1;
                }
            }
            CursorDirection::Down => {
                if self.cursor.y < self.buffer.len() {
                    self.cursor.y += 1;
                }
            }
            CursorDirection::Position(x, y) => {
                if y >= 0 && y < self.buffer.len() {
                    self.cursor.y = y;
                }
                if x >= 0 && x < self.dimensions.x {
                    self.cursor.x = x;
                }
            }
        }
    }

    fn raw() -> String {
        String::new()
    }
}
