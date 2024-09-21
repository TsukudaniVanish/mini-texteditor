use std::io::{self, Read, Write };

fn main() {
    let mut out = io::stdout();
    for b in io::stdin().bytes() {
        let c = b.unwrap();
        writeln!(&mut out ,"{}", c).unwrap();
    }
}
