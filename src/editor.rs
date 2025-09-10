use termion::screen::IntoAlternateScreen;
use termion::raw::RawTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::input::TermRead;
use termion::clear;
use termion::event::Key;
use termion::terminal_size;
use termion::cursor;
use std::io::{Write, Read, stdout, stdin, Stdout, Stdin};
use crate::buffer::Buffer;

pub struct Editor {
    screen: AlternateScreen<RawTerminal<Stdout>>,
    input: Stdin,
    screen_size: (u16, u16),
    cursor: (u16, u16),
    buffer: Buffer,
}

impl Editor {
    pub fn new() -> Self {
        let screen_size = terminal_size().unwrap();
        // let _ = stdout().into_alternate_screen().unwrap().into_raw_mode().unwrap();
        let mut screen = stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap();

        Editor {
            screen,
            input: stdin(),
            screen_size,
            cursor: (0, 0),
            buffer: Buffer::new(None),
        }
    }

    pub fn run(&mut self) {
        write!(self.screen, "{}{}{}", cursor::Hide, clear::All, cursor::Goto(self.screen_size.0 / 2, self.screen_size.1/2)).unwrap();
        write!(self.screen, "Tiny Text Editor").unwrap();
        self.screen.flush().unwrap();

        for key in self.input.by_ref().keys() {
            match key.unwrap() {
                Key::F(1) => break,
                Key::Char(c) => self.buffer.push_char(c),
                _ => (),
            }

            write!(self.screen, "{}", cursor::Hide).unwrap();
            write!(self.screen, "{}", clear::All).unwrap();
            write!(self.screen, "{}", cursor::Goto(1, 1)).unwrap();
            for line in self.buffer.lines.iter() {
                write!(self.screen, "{}", line).unwrap();
            }
            let (row, col) = self.buffer.get_cursor();
            write!(self.screen, "{}", cursor::Goto(col + 1, row + 1)).unwrap();
            write!(self.screen, "{}", cursor::Show).unwrap();
            self.screen.flush().unwrap();
        }
    }
}
