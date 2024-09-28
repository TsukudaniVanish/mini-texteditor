mod editor;




fn main() {
    let e = editor::Editor::new();
    if let Err(e) = e.run() {
        panic!("{}", e);
    }
}
