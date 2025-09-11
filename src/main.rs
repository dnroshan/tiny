use crate::buffer::Buffer;
use crate::editor::Editor;
use std::env::args;

mod buffer;
mod editor;

fn main() {
    let args: Vec<_> = args().collect();
    let buffer;
    if args.len() == 1 {
        buffer = Buffer::new(None);
    } else {
        buffer = Buffer::new(Some(args[1].clone()));
    }
    let mut editor = Editor::new(buffer);
    editor.run();
}
