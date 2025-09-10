use crate::editor::Editor;

mod buffer;
mod editor;

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
