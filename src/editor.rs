use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read, Result, Write};

pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Self{}
    }

    pub fn run(&self) -> Result<()> {
        let mut out = io::stdout();
        enable_raw_mode()?;
        for b in io::stdin().bytes() {
            let c = b.unwrap() as char;
            if c.is_control() {
                writeln!(&mut out, "Binary: {0:08b} ASCII: {0:#03}\r\n", c as u8)?;
            } else {
                writeln!(
                    &mut out,
                    "Binary: {0:08b} ASCII: {0:#03} Charactor: {1:#?}\r\n",
                    c as u8, c
                )?;
            }
            if c == 'q' {
                break;
            }
        }
        disable_raw_mode()?;
        return Ok(());
    }
}
