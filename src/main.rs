use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::clear::All;
use termion::input::TermRead;
use termion::cursor;
use std::io::{Write, stdout, stdin, Stdout};

mod buffer;
mod editor;

use crate::buffer::Buffer;
use crate::editor::Editor;

fn main() {
    let mut editor = Editor::new();
    editor.run();
}
