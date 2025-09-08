use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::clear::All;
use termion::input::{Keys, TermRead};
use termion::cursor;
use std::io::{Write, Read, stdout, stdin};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout().into_raw_mode()?;

    write!(stdout, "{}{}", All, cursor::Goto(1, 1)).unwrap();
    write!(stdout, "Tiny Text Editor").unwrap();
    stdout.flush();

    let stdin = stdin();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            _ => continue,
        }
    }

    Ok(())
}

