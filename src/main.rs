#![warn(clippy::all, clippy::pedantic)]
mod editor;




fn main() {
    let mut e = editor::Editor::new();
    if let Err(e) = e.run() {
        panic!("{}", e);
    }
}
