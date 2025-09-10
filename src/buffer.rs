
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
            None => String::from("untitled")
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

        let mut line = &mut self.lines[self.cursor.0];
        if line.len() == 0 || line.len() == self.cursor.1 {
            line.push(c);
        } else {
            line.insert(self.cursor.1, c);
        }
        self.cursor.1 += 1;
    }
}
