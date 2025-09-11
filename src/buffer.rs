use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Buffer {
    path: String,
    cursor: (usize, usize),
    pub lines: Vec<String>,
    empty: bool,
}

impl Buffer {
    pub fn new(path: Option<String>) -> Self {
        let mut lines: Vec<String>;
        let empty: bool;

        let path = match path {
            Some(path) => {
                lines = Self::open(&path);
                empty = false;
                path
            }
            None => {
                lines = Vec::new();
                lines.push(String::new());
                empty = true;
                String::from("untitled")
            }
        };

        Buffer {
            path,
            lines,
            cursor: (0, 0),
            empty,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.empty
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_cursor(&self) -> (u16, u16) {
        let (row, col) = self.cursor;
        (row as u16, col as u16)
    }

    pub fn set_cursor(&mut self, cursor: (u16, u16)) {
        let (row, col) = cursor;
        self.cursor = (row as usize, col as usize);
    }

    pub fn push_char(&mut self, c: char) {
        if self.lines.len() == 0 {
            self.lines.push(String::new());
        }

        let line = &mut self.lines[self.cursor.0];
        let len = line.len();
        if len == 0 || len == self.cursor.1 {
            line.push(c);
        } else {
            line.insert(self.cursor.1, c);
        }
        self.cursor.1 += 1;
    }

    pub fn create_newline(&mut self) {
        let len = self.lines[self.cursor.0].len();
        if len == 0 || (self.cursor.0 == self.lines.len() - 1 && len == self.cursor.1) {
            self.lines.push(String::new());
        } else {
            let (first_part, second_part) = {
                let (first_part, second_part) = self.lines[self.cursor.0].split_at(self.cursor.1);
                (first_part.to_string(), second_part.to_string())
            };
            self.lines[self.cursor.0] = first_part;
            if self.cursor.0 + 1 == self.lines.len() - 1 {
                self.lines.push(second_part);
            } else {
                self.lines.insert(self.cursor.0 + 1, second_part);
            }
        }
        self.cursor.0 += 1;
        self.cursor.1 = 0;
    }

    pub fn cursor_up(&mut self) {
        if self.cursor.0 == 0 {
            return;
        }

        self.cursor.0 -= 1;

        let len = self.lines[self.cursor.0].len();
        if self.cursor.1 >= self.lines[self.cursor.0].len() {
            self.cursor.1 = len;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor.0 == self.lines.len() - 1 {
            return;
        }

        self.cursor.0 += 1;
        let len = self.lines[self.cursor.0].len();
        if self.cursor.1 >= self.lines[self.cursor.0].len() {
            self.cursor.1 = len;
        }
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.1 == 0 {
            return;
        }

        self.cursor.1 -= 1;
    }

    pub fn cursor_right(&mut self) {
        let len = self.lines[self.cursor.0].len();
        if self.cursor.1 >= len {
            return;
        }

        self.cursor.1 += 1;
    }

    pub fn pop_char(&mut self) {
        if self.cursor.1 > 0 {
            self.lines[self.cursor.0].remove(self.cursor.1 - 1);
            self.cursor.1 -= 1;
        } else if self.cursor.0 == 0 {
            return;
        } else {
            self.lines.remove(self.cursor.0);
            self.cursor.0 -= 1;
            let len = self.lines[self.cursor.0].len();
            self.cursor.1 = len;
        }
    }

    fn open(path: &str) -> Vec<String> {
        let path = Path::new(path);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(String::from(line.unwrap()));
        }

        lines
    }

    pub fn write(&self) {
        let filename = "out.txt";

        let mut content = String::new();
        for line in self.lines.iter() {
            write!(&mut content, "{}\n", line).unwrap();
        }
        fs::write(&self.path, content.as_bytes()).unwrap();
    }
}
