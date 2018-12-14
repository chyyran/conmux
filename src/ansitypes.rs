#![allow(dead_code)]
// Copyright 2016 Joe Wilm, The Alacritty Project Contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//! ANSI Terminal Stream Parsing Types


/// Describes shape of cursor
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CursorStyle {
    /// Cursor is a block like `▒`
    Block,

    /// Cursor is an underscore like `_`
    Underline,

    /// Cursor is a vertical bar `⎸`
    Beam,

    /// Cursor is a box like `☐`
    HollowBlock,
}

impl Default for CursorStyle {
    fn default() -> CursorStyle {
        CursorStyle::Block
    }
}

/// Terminal modes
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Mode {
    /// ?1
    CursorKeys = 1,
    /// Select 80 or 132 columns per page
    ///
    /// CSI ? 3 h -> set 132 column font
    /// CSI ? 3 l -> reset 80 column font
    ///
    /// Additionally,
    ///
    /// * set margins to default positions
    /// * erases all data in page memory
    /// * resets DECLRMM to unavailable
    /// * clears data from the status line (if set to host-writable)
    DECCOLM = 3,
    /// IRM Insert Mode
    ///
    /// NB should be part of non-private mode enum
    ///
    /// * `CSI 4 h` change to insert mode
    /// * `CSI 4 l` reset to replacement mode
    Insert = 4,
    /// ?6
    Origin = 6,
    /// ?7
    LineWrap = 7,
    /// ?12
    BlinkingCursor = 12,
    /// 20
    ///
    /// NB This is actually a private mode. We should consider adding a second
    /// enumeration for public/private modesets.
    LineFeedNewLine = 20,
    /// ?25
    ShowCursor = 25,
    /// ?1000
    ReportMouseClicks = 1000,
    /// ?1002
    ReportCellMouseMotion = 1002,
    /// ?1003
    ReportAllMouseMotion = 1003,
    /// ?1004
    ReportFocusInOut = 1004,
    /// ?1006
    SgrMouse = 1006,
    /// ?1049
    SwapScreenAndSetRestoreCursor = 1049,
    /// ?2004
    BracketedPaste = 2004,
}

impl Mode {
    /// Create mode from a primitive
    ///
    /// TODO lots of unhandled values..
    pub fn from_primitive(private: bool, num: i64) -> Option<Mode> {
        if private {
            Some(match num {
                1 => Mode::CursorKeys,
                3 => Mode::DECCOLM,
                6 => Mode::Origin,
                7 => Mode::LineWrap,
                12 => Mode::BlinkingCursor,
                25 => Mode::ShowCursor,
                1000 => Mode::ReportMouseClicks,
                1002 => Mode::ReportCellMouseMotion,
                1003 => Mode::ReportAllMouseMotion,
                1004 => Mode::ReportFocusInOut,
                1006 => Mode::SgrMouse,
                1049 => Mode::SwapScreenAndSetRestoreCursor,
                2004 => Mode::BracketedPaste,
                _ => {
                    return None
                }
            })
        } else {
            Some(match num {
                4 => Mode::Insert,
                20 => Mode::LineFeedNewLine,
                _ => return None
            })
        }
    }
}

/// Mode for clearing line
///
/// Relative to cursor
#[derive(Debug, Clone, Copy)]
pub enum LineClearMode {
    /// Clear right of cursor
    Right,
    /// Clear left of cursor
    Left,
    /// Clear entire line
    All,
}

/// Mode for clearing terminal
///
/// Relative to cursor
#[derive(Debug,  Clone, Copy)]
pub enum ClearMode {
    /// Clear below cursor
    Below,
    /// Clear above cursor
    Above,
    /// Clear entire terminal
    All,
    /// Clear 'saved' lines (scrollback)
    Saved
}

/// Mode for clearing tab stops
#[derive(Debug,  Clone, Copy)]
pub enum TabulationClearMode {
    /// Clear stop under cursor
    Current,
    /// Clear all stops
    All,
}

/// Standard colors
///
/// The order here matters since the enum should be castable to a `usize` for
/// indexing a color list.
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum NamedColor {
    /// Black
    Black = 0,
    /// Red
    Red,
    /// Green
    Green,
    /// Yellow
    Yellow,
    /// Blue
    Blue,
    /// Magenta
    Magenta,
    /// Cyan
    Cyan,
    /// White
    White,
    /// Bright black
    BrightBlack,
    /// Bright red
    BrightRed,
    /// Bright green
    BrightGreen,
    /// Bright yellow
    BrightYellow,
    /// Bright blue
    BrightBlue,
    /// Bright magenta
    BrightMagenta,
    /// Bright cyan
    BrightCyan,
    /// Bright white
    BrightWhite,
    /// The foreground color
    Foreground = 256,
    /// The background color
    Background,
    /// Color for the text under the cursor
    CursorText,
    /// Color for the cursor itself
    Cursor,
    /// Dim black
    DimBlack,
    /// Dim red
    DimRed,
    /// Dim green
    DimGreen,
    /// Dim yellow
    DimYellow,
    /// Dim blue
    DimBlue,
    /// Dim magenta
    DimMagenta,
    /// Dim cyan
    DimCyan,
    /// Dim white
    DimWhite,
    /// The bright foreground color
    BrightForeground,
    /// Dim foreground
    DimForeground,
}

impl NamedColor {
    pub fn to_bright(self) -> Self {
        match self {
            NamedColor::Foreground => NamedColor::BrightForeground,
            NamedColor::Black => NamedColor::BrightBlack,
            NamedColor::Red => NamedColor::BrightRed,
            NamedColor::Green => NamedColor::BrightGreen,
            NamedColor::Yellow => NamedColor::BrightYellow,
            NamedColor::Blue => NamedColor::BrightBlue,
            NamedColor::Magenta => NamedColor::BrightMagenta,
            NamedColor::Cyan => NamedColor::BrightCyan,
            NamedColor::White => NamedColor::BrightWhite,
            NamedColor::DimForeground => NamedColor::Foreground,
            NamedColor::DimBlack => NamedColor::Black,
            NamedColor::DimRed => NamedColor::Red,
            NamedColor::DimGreen => NamedColor::Green,
            NamedColor::DimYellow => NamedColor::Yellow,
            NamedColor::DimBlue => NamedColor::Blue,
            NamedColor::DimMagenta => NamedColor::Magenta,
            NamedColor::DimCyan => NamedColor::Cyan,
            NamedColor::DimWhite => NamedColor::White,
            val => val
        }
    }

    pub fn to_dim(self) -> Self {
        match self {
            NamedColor::Black => NamedColor::DimBlack,
            NamedColor::Red => NamedColor::DimRed,
            NamedColor::Green => NamedColor::DimGreen,
            NamedColor::Yellow => NamedColor::DimYellow,
            NamedColor::Blue => NamedColor::DimBlue,
            NamedColor::Magenta => NamedColor::DimMagenta,
            NamedColor::Cyan => NamedColor::DimCyan,
            NamedColor::White => NamedColor::DimWhite,
            NamedColor::Foreground => NamedColor::DimForeground,
            NamedColor::BrightBlack => NamedColor::Black,
            NamedColor::BrightRed => NamedColor::Red,
            NamedColor::BrightGreen => NamedColor::Green,
            NamedColor::BrightYellow => NamedColor::Yellow,
            NamedColor::BrightBlue => NamedColor::Blue,
            NamedColor::BrightMagenta => NamedColor::Magenta,
            NamedColor::BrightCyan => NamedColor::Cyan,
            NamedColor::BrightWhite => NamedColor::White,
            NamedColor::BrightForeground => NamedColor::Foreground,
            val => val
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Named(NamedColor),
    Spec(u8, u8, u8),
    Indexed(u8),
}

/// Terminal character attributes
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Attr {
    /// Clear all special abilities
    Reset,
    /// Bold text
    Bold,
    /// Dim or secondary color
    Dim,
    /// Italic text
    Italic,
    /// Underscore text
    Underscore,
    /// Blink cursor slowly
    BlinkSlow,
    /// Blink cursor fast
    BlinkFast,
    /// Invert colors
    Reverse,
    /// Do not display characters
    Hidden,
    /// Strikethrough text
    Strike,
    /// Cancel bold
    CancelBold,
    /// Cancel bold and dim
    CancelBoldDim,
    /// Cancel italic
    CancelItalic,
    /// Cancel underline
    CancelUnderline,
    /// Cancel blink
    CancelBlink,
    /// Cancel inversion
    CancelReverse,
    /// Cancel text hiding
    CancelHidden,
    /// Cancel strike through
    CancelStrike,
    /// Set indexed foreground color
    Foreground(Color),
    /// Set indexed background color
    Background(Color),
}

/// Identifiers which can be assigned to a graphic character set
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharsetIndex {
    /// Default set, is designated as ASCII at startup
    G0,
    G1,
    G2,
    G3,
}

impl Default for CharsetIndex {
    fn default() -> Self {
        CharsetIndex::G0
    }
}

/// Standard or common character sets which can be designated as G0-G3
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StandardCharset {
    Ascii,
    SpecialCharacterAndLineDrawing,
}

impl Default for StandardCharset {
    fn default() -> Self {
        StandardCharset::Ascii
    }
}


/// C0 set of 7-bit control characters (from ANSI X3.4-1977).
#[allow(non_snake_case)]
pub mod C0 {
    /// Null filler, terminal should ignore this character
    pub const NUL: u8 = 0x00;
    /// Start of Header
    pub const SOH: u8 = 0x01;
    /// Start of Text, implied end of header
    pub const STX: u8 = 0x02;
    /// End of Text, causes some terminal to respond with ACK or NAK
    pub const ETX: u8 = 0x03;
    /// End of Transmission
    pub const EOT: u8 = 0x04;
    /// Enquiry, causes terminal to send ANSWER-BACK ID
    pub const ENQ: u8 = 0x05;
    /// Acknowledge, usually sent by terminal in response to ETX
    pub const ACK: u8 = 0x06;
    /// Bell, triggers the bell, buzzer, or beeper on the terminal
    pub const BEL: u8 = 0x07;
    /// Backspace, can be used to define overstruck characters
    pub const BS: u8 = 0x08;
    /// Horizontal Tabulation, move to next predetermined position
    pub const HT: u8 = 0x09;
    /// Linefeed, move to same position on next line (see also NL)
    pub const LF: u8 = 0x0A;
    /// Vertical Tabulation, move to next predetermined line
    pub const VT: u8 = 0x0B;
    /// Form Feed, move to next form or page
    pub const FF: u8 = 0x0C;
    /// Carriage Return, move to first character of current line
    pub const CR: u8 = 0x0D;
    /// Shift Out, switch to G1 (other half of character set)
    pub const SO: u8 = 0x0E;
    /// Shift In, switch to G0 (normal half of character set)
    pub const SI: u8 = 0x0F;
    /// Data Link Escape, interpret next control character specially
    pub const DLE: u8 = 0x10;
    /// (DC1) Terminal is allowed to resume transmitting
    pub const XON: u8 = 0x11;
    /// Device Control 2, causes ASR-33 to activate paper-tape reader
    pub const DC2: u8 = 0x12;
    /// (DC2) Terminal must pause and refrain from transmitting
    pub const XOFF: u8 = 0x13;
    /// Device Control 4, causes ASR-33 to deactivate paper-tape reader
    pub const DC4: u8 = 0x14;
    /// Negative Acknowledge, used sometimes with ETX and ACK
    pub const NAK: u8 = 0x15;
    /// Synchronous Idle, used to maintain timing in Sync communication
    pub const SYN: u8 = 0x16;
    /// End of Transmission block
    pub const ETB: u8 = 0x17;
    /// Cancel (makes VT100 abort current escape sequence if any)
    pub const CAN: u8 = 0x18;
    /// End of Medium
    pub const EM: u8 = 0x19;
    /// Substitute (VT100 uses this to display parity errors)
    pub const SUB: u8 = 0x1A;
    /// Prefix to an escape sequence
    pub const ESC: u8 = 0x1B;
    /// File Separator
    pub const FS: u8 = 0x1C;
    /// Group Separator
    pub const GS: u8 = 0x1D;
    /// Record Separator (sent by VT132 in block-transfer mode)
    pub const RS: u8 = 0x1E;
    /// Unit Separator
    pub const US: u8 = 0x1F;
    /// Delete, should be ignored by terminal
    pub const DEL: u8 = 0x7f;
}


/// C1 set of 8-bit control characters (from ANSI X3.64-1979)
///
/// 0x80 (@), 0x81 (A), 0x82 (B), 0x83 (C) are reserved
/// 0x98 (X), 0x99 (Y) are reserved
/// 0x9a (Z) is 'reserved', but causes DEC terminals to respond with DA codes
#[allow(non_snake_case)]
pub mod C1 {
    /// Reserved
    pub const PAD: u8 = 0x80;
    /// Reserved
    pub const HOP: u8 = 0x81;
    /// Reserved
    pub const BPH: u8 = 0x82;
    /// Reserved
    pub const NBH: u8 = 0x83;
    /// Index, moves down one line same column regardless of NL
    pub const IND: u8 = 0x84;
    /// New line, moves done one line and to first column (CR+LF)
    pub const NEL: u8 = 0x85;
    /// Start of Selected Area to be sent to auxiliary output device
    pub const SSA: u8 = 0x86;
    /// End of Selected Area to be sent to auxiliary output device
    pub const ESA: u8 = 0x87;
    /// Horizontal Tabulation Set at current position
    pub const HTS: u8 = 0x88;
    /// Hor Tab Justify, moves string to next tab position
    pub const HTJ: u8 = 0x89;
    /// Vertical Tabulation Set at current line
    pub const VTS: u8 = 0x8A;
    /// Partial Line Down (subscript)
    pub const PLD: u8 = 0x8B;
    /// Partial Line Up (superscript)
    pub const PLU: u8 = 0x8C;
    /// Reverse Index, go up one line, reverse scroll if necessary
    pub const RI: u8 = 0x8D;
    /// Single Shift to G2
    pub const SS2: u8 = 0x8E;
    /// Single Shift to G3 (VT100 uses this for sending PF keys)
    pub const SS3: u8 = 0x8F;
    /// Device Control String, terminated by ST (VT125 enters graphics)
    pub const DCS: u8 = 0x90;
    /// Private Use 1
    pub const PU1: u8 = 0x91;
    /// Private Use 2
    pub const PU2: u8 = 0x92;
    /// Set Transmit State
    pub const STS: u8 = 0x93;
    /// Cancel character, ignore previous character
    pub const CCH: u8 = 0x94;
    /// Message Waiting, turns on an indicator on the terminal
    pub const MW: u8 = 0x95;
    /// Start of Protected Area
    pub const SPA: u8 = 0x96;
    /// End of Protected Area
    pub const EPA: u8 = 0x97;
    /// SOS
    pub const SOS: u8 = 0x98;
    /// SGCI
    pub const SGCI: u8 = 0x99;
    /// DECID - Identify Terminal
    pub const DECID: u8 = 0x9a;
    /// Control Sequence Introducer
    pub const CSI: u8 = 0x9B;
    /// String Terminator (VT125 exits graphics)
    pub const ST: u8 = 0x9C;
    /// Operating System Command (reprograms intelligent terminal)
    pub const OSC: u8 = 0x9D;
    /// Privacy Message (password verification), terminated by ST
    pub const PM: u8 = 0x9E;
    /// Application Program Command (to word processor), term by ST
    pub const APC: u8 = 0x9F;
}