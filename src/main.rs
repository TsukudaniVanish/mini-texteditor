#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod cursor;




fn main() {
    let mut e = editor::Editor::new();
    if let Err(e) = e.run() {
        panic!("{}", e);
    }
}
