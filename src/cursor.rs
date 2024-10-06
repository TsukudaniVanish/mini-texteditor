use std::io::{Write, Error};
use std::fmt::Display;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;

pub trait Cursor {
    fn move_to<O: Write>(self: &Self, out: &mut O, col: u16, row: u16) -> Result<(), Error>;
    fn print<O: Write, T: Display>(self: &Self, out: &mut O, content: T) -> Result<(), Error>;
}

pub struct TerminalCursor {}

impl TerminalCursor {
    pub fn new () -> Self {
    return Self {}
    }
}


impl Cursor for TerminalCursor {
    fn move_to<O: Write>(self: &Self, out: &mut O, col: u16, row: u16) -> std::result::Result<(), Error> {
        execute!(out, MoveTo(col, row))
    }

    fn print<O: Write, T: std::fmt::Display>(self: &Self, out: &mut O, content: T) -> std::result::Result<(), Error> {
        execute!(out, Print(content))
    }
}
