pub struct Buffer {
    name: String,
    cursor: (usize, usize),
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn new(name: Option<String>) -> Self {
        let mut lines = Vec::new();
        lines.push(String::new());
        let name = match name {
            Some(n) => n,
            None => String::from("untitled"),
        };

        Buffer {
            name,
            lines,
            cursor: (0, 0),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
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
}
