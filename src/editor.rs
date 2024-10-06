use crate::cursor::{Cursor, TerminalCursor};
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{self, Result, Write};

pub struct Editor<C: Cursor> {
    cur: C,
    shuld_quit: bool,
}

impl Editor<TerminalCursor> {
    pub fn new() -> Self {
        Self {
            cur: TerminalCursor::new(),
            shuld_quit: false,
        }
    }
}

impl<C: Cursor> Editor<C> {
    pub fn run(&mut self) -> Result<()> {
        let mut out = io::stdout();
        self.initialize(&mut out)?;
        let result = self.repl(&mut out);
        Self::terminate()?;
        result
    }

    fn initialize<O: Write>(&self, out: &mut O) -> Result<()> {
        enable_raw_mode()?;
        self.clear_screen(out)?;
        self.draw_rows(out)
    }

    fn terminate() -> Result<()> {
        disable_raw_mode()
    }

    fn clear_screen<O: Write>(&self, out: &mut O) -> Result<()> {
        execute!(out, Clear(ClearType::All))?;
        self.move_to(out, 0, 0)
    }

    fn repl<O: Write>(&mut self, out: &mut O) -> Result<()> {
        enable_raw_mode()?;
        loop {
            let event = read()?;
            self.evaluate_event(&event);
            self.referesh_screen(out)?;
            if self.shuld_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.shuld_quit = true;
                }
                _ => (),
            }
        }
    }

    fn referesh_screen<O: Write>(&self, out: &mut O) -> Result<()> {
        if self.shuld_quit {
            self.clear_screen(out)?;
            write!(out, "Goodbye. \r\n")?;
        }
        Ok(())
    }

    /// draw_rows draws ~ every rows.
    fn draw_rows<O: Write>(&self, out: &mut O) -> Result<()> {
        let (_, rows) = size()?;
        for row in 0..rows {
            self.move_to(out, 0, row)?;
            self.print(out, "~")?;
        }
        self.move_to(out, 1, 0)?;
        Ok(())
    }
}

impl<C: Cursor> Cursor for Editor<C> {
    fn move_to<O: Write>(
        self: &Self,
        out: &mut O,
        col: u16,
        row: u16,
    ) -> std::result::Result<(), io::Error> {
        self.cur.move_to(out, col, row)
    }

    fn print<O: Write, T: std::fmt::Display>(
        self: &Self,
        out: &mut O,
        content: T,
    ) -> std::result::Result<(), io::Error> {
        self.cur.print(out, content)
    }
}
