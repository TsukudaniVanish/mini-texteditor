use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
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
        self.repl(&mut out)
    }

    fn repl<O: Write>(&mut self, out: &mut O) -> Result<()> {
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                write!(
                    out,
                    "Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?}\r\n"
                )?;
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.shuld_quit = true;
                    }
                    _ => (),
                }
            }
            if self.shuld_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
