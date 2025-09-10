use crate::buffer::Buffer;
use std::io::{Read, Stdin, Stdout, Write, stdin, stdout};
use termion::clear;
use termion::color;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::screen::AlternateScreen;
use termion::screen::IntoAlternateScreen;
use termion::terminal_size;

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
        let screen = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();

        Editor {
            screen,
            input: stdin(),
            screen_size,
            cursor: (0, 0),
            buffer: Buffer::new(None),
        }
    }

    pub fn run(&mut self) {
        write!(
            self.screen,
            "{}{}{}",
            cursor::Hide,
            clear::All,
            cursor::Goto(self.screen_size.0 / 2, self.screen_size.1 / 2)
        )
        .unwrap();
        write!(self.screen, "Tiny Text Editor").unwrap();
        self.screen.flush().unwrap();

        for key in self.input.by_ref().keys() {
            match key.unwrap() {
                Key::F(1) => break,
                Key::Char(c) => {
                    if c == '\n' {
                        self.buffer.create_newline();
                    } else {
                        self.buffer.push_char(c)
                    }
                }
                Key::Up => self.buffer.cursor_up(),
                Key::Down => self.buffer.cursor_down(),
                Key::Left => self.buffer.cursor_left(),
                Key::Right => self.buffer.cursor_right(),
                Key::Backspace => self.buffer.pop_char(),
                _ => (),
            }

            write!(self.screen, "{}", cursor::Hide).unwrap();
            write!(self.screen, "{}", clear::All).unwrap();
            write!(self.screen, "{}", cursor::Goto(1, 1)).unwrap();
            for (index, line) in self.buffer.lines.iter().enumerate() {
                write!(
                    self.screen,
                    "{}{:>3}{}{}\n\r",
                    color::Fg(color::Red),
                    index + 1,
                    color::Fg(color::Reset),
                    line
                )
                .unwrap();
            }
            let (row, col) = self.buffer.get_cursor();
            write!(self.screen, "{}", cursor::Goto(col + 4, row + 1)).unwrap();
            write!(self.screen, "{}", cursor::Show).unwrap();
            self.screen.flush().unwrap();
        }
    }
}
