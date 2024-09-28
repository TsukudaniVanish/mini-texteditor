use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Result, Write};

pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) -> Result<()> {
        let mut out = io::stdout();
        enable_raw_mode()?;
        loop {
            match read() {
                Ok(Key(event)) => {
                    write!(&mut out, "{event:?}\r\n")?;
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                }
                Err(e) => write!(&mut out, "Err: {e:?}\r\n")?,
                _ => (),
            }
        }

        disable_raw_mode()?;
        Ok(())
    }
}
