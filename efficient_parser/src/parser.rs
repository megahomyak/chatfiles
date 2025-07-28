pub trait Cursor {
    fn back(&mut self) -> bool;
    fn read(&mut self) -> Option<char>;
    fn end(&mut self);
}

pub struct Line<C: Cursor> {
    cursor: C,
}

pub enum LineCategory {
    Special(),
    Regular(),
}

impl<C: Cursor> Line<C> {
    pub fn new(cursor: C) -> Self {
        Self { cursor }
    }
    fn back_real(&mut self) -> bool {
        if self.cursor.back() {
            let c = self.cursor.read().unwrap();
            self.cursor.back();
            if c == '\r' {
                self.cursor.back()
            } else {
                true
            }
        } else {
            false
        }
    }
    fn read_real(&mut self) -> Option<char> {
        match self.cursor.read() {
            None => None,
            Some(c) => if c == '\r' {
                self.cursor.read()
            } else {
                Some(c)
            }
        }
    }
    fn line_beginning(&mut self) {
        while self.cursor.back() && self.cursor.read().unwrap() != '\n' {
            self.cursor.back();
        }
    }
    pub fn prev_line(&mut self) -> bool {
        if !self.back_real() {
            return false;
        }
        self.line_beginning();
        true
    }
    pub fn next_line(&mut self) -> bool {
        while let Some(c) = self.read_real() {
            if c == '\n' {
                if self.cursor.read().is_some() {
                    self.cursor.back();
                    return true;
                }
            }
        }
        false
    }
    pub fn last_line(&mut self) {
        self.cursor.end();
        self.cursor.back();
        self.line_beginning();
    }
    pub fn categorize(&mut self) -> LineCategory {
        if self.read_real() == Some('\\') {
            if self.read_real() == Some('\\') {
                self.back_real();
            } else {
                return LineCategory::Special();
            }
        }
        LineCategory::Regular()
    }
}

impl<C: Cursor> Iterator for Line<C> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.read().and_then(|c| if c == '\n' {
            self.cursor.back();
            None
        } else {
            Some(c)
        })
    }
}
