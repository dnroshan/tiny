use crate::buffer::Buffer;
use std::fmt::Write as OtherWrite;
use std::io::{Read, Stdout, Write, stdin, stdout};
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
    screen_size: (u16, u16),
    cursor: (u16, u16),
    buffer: Buffer,
}

impl Editor {
    pub fn new(buffer: Buffer) -> Self {
        let screen_size = terminal_size().unwrap();
        let screen = stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap();

        Editor {
            screen,
            screen_size,
            cursor: (0, 0),
            buffer,
        }
    }

    fn print_banner(&mut self) {
        let banner = "Tiny Text Editor";
        write!(
            self.screen,
            "{}{}{}",
            cursor::Hide,
            clear::All,
            cursor::Goto(
                self.screen_size.0 / 2 - banner.len() as u16 / 2,
                self.screen_size.1 / 2
            )
        )
        .unwrap();
        write!(self.screen, "{}", banner).unwrap();
        self.screen.flush().unwrap();
    }

    fn print_content(&mut self) {
        let mut render = String::new();
        write!(&mut render, "{}", cursor::Hide).unwrap();
        write!(&mut render, "{}", cursor::Goto(1, 1)).unwrap();
        for (index, line) in self.buffer.lines.iter().enumerate() {
            write!(
                &mut render,
                "{}{}{:>3} {}{}\n\r",
                clear::CurrentLine,
                color::Fg(color::Red),
                index + 1,
                color::Fg(color::Reset),
                line
            )
            .unwrap();
        }
        let (row, col) = self.buffer.get_cursor();
        write!(&mut render, "{}", cursor::Goto(col + 5, row + 1)).unwrap();
        write!(&mut render, "{}", cursor::Show).unwrap();
        write!(self.screen, "{}", render).unwrap();
        self.screen.flush().unwrap();
    }

    pub fn run(&mut self) {
        if self.buffer.is_empty() {
            self.print_banner();
        } else {
            self.print_content();
        }

        for key in stdin().keys() {
            match key.unwrap() {
                Key::F(1) => break,
                Key::F(2) => self.buffer.write(),
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

            self.print_content();
        }
    }
}
