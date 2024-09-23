use std::io::{self, Read, Write };
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};


fn main() {
    let mut out = io::stdout();
    if let Err(e) = enable_raw_mode() {
        println!("failed to start a row mode");
        panic!("{}", e);
    }
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        if c.is_control() {
            writeln!(&mut out,"Binary: {0:08b} ASCII: {0:#03}\r\n", c as u8).unwrap();
        } else {
            writeln!(&mut out, "Binary: {0:08b} ASCII: {0:#03} Charactor: {1:#?}\r\n", c as u8, c).unwrap();
        }
        if c ==  'q' {
            break;
        }
    }
    if let Err(e) = disable_raw_mode() {
        writeln!(&mut out, "failed to stop a row mod\r\n").unwrap();
        panic!("{}", e);
    }
}
