use crossterm::cursor::MoveTo;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size};
use crossterm::style::Print;
use std::io::{self, Result, Write};

pub struct Editor {
    shuld_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { shuld_quit: false }
    }

    pub fn run(&mut self) -> Result<()> {
        let mut out = io::stdout();
        self.initialize(&mut out)?;
        let result = self.repl(&mut out);
        Self::terminate()?;
        result
    }

    fn initialize<O: Write>(&self, out: &mut O) -> Result<()> {
        enable_raw_mode()?;
        Self::clear_screen(out)?;
        Self::draw_rows(out)
    }

    fn terminate() -> Result<()> {
        disable_raw_mode()
    }

    fn clear_screen<O: Write>(out: &mut O) -> Result<()> {
        execute!(out, Clear(ClearType::All))?;
        execute!(out, MoveTo(0, 0))
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
            Self::clear_screen(out)?;
            write!(out, "Goodbye. \r\n")?;
        }
        Ok(())
    }

    /// draw_rows draws ~ every rows.
    fn draw_rows<O: Write>(out: &mut O) -> Result<()> {
        let (_, rows) = size()?;
        for row in 0..rows {
            execute!(out, MoveTo(0, row),Print("~"))?;
        }
        execute!(out, MoveTo(1, 0))?;
        Ok(())
    }
}
