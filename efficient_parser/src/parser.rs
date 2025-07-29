pub trait Cursor {
    #[must_use]
    fn prev(&mut self) -> Option<char>;
    #[must_use]
    fn next(&mut self) -> Option<char>;
    fn end(&mut self);
}

pub struct ForwardLine<C: Cursor> {
    cursor: C,
}

fn prev_real(cursor: &mut impl Cursor) -> Option<char> {
    cursor.prev().and_then(|c| {
        if c == '\r' {
            cursor.prev()
        } else {
            Some(c)
        }
    })
}

fn next_real(cursor: &mut impl Cursor) -> Option<char> {
    cursor.next().and_then(|c| {
        if c == '\r' {
            cursor.next()
        } else {
            Some(c)
        }
    })
}

fn line_beginning(cursor: &mut impl Cursor) {
    while let Some(c) = cursor.prev() {
        if c == '\n' {
            cursor.next().unwrap();
            break;
        }
    }
}

fn next_real_in_line(cursor: &mut impl Cursor) -> Option<char> {
    next_real(cursor).and_then(|c| if c == '\n' {
        prev_real(cursor);
        None
    } else {
        Some(c)
    })
}

impl<C: Cursor> Iterator for ForwardLine<C> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        next_real_in_line(&mut self.cursor)
    }
}

impl<C: Cursor> ForwardLine<C> {
    pub fn new(cursor: C) -> Self {
        Self { cursor }
    }
    pub fn next_line(mut self) -> Result<Self, BackLine<C>> {
        while let Some(c) = next_real(&mut self.cursor) {
            if c == '\n' {
                if self.cursor.next().is_some() {
                    self.cursor.prev().unwrap();
                    break;
                } else {
                    return Err(self.end());
                }
            }
        }
        Ok(ForwardLine { cursor: self.cursor })
    }
    pub fn end(mut self) -> BackLine<C> {
        self.cursor.end();
        self.cursor.prev().unwrap();
        line_beginning(&mut self.cursor);
        BackLine { cursor: self.cursor }
    }
}

pub struct BackLine<C: Cursor> {
    cursor: C,
}

impl<C: Cursor> Iterator for BackLine<C> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        next_real_in_line(&mut self.cursor)
    }
}

impl<C: Cursor> BackLine<C> {
    pub fn prev_line(self) -> Self {

    }
}

pub struct LinesForward<'a, C: Cursor> {
    cursor: &'a mut C,
}

impl<'a, C: Cursor> LinesForward<'a, C> {
    pub fn new(cursor: &'a mut C) -> Self {
        Self { cursor }
    }
    pub fn end(mut self) -> LinesBack<C> {
    }
}

impl<'a, C: Cursor> Iterator for LinesForward<'a, C> {
    type Item = &'a mut C;

    fn next(&mut self) -> Option<Self::Item> {}
}

pub struct LinesBack<'a, C: Cursor> {
    cursor: &'a mut C,
}

pub struct Line<'a, C: Cursor> {
    cursor: &'a mut C,
}

impl<'a, C: Cursor> Iterator for Line<'a, C> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        match next_real(self.cursor) {
            None => None,
            Some(c) => {
                if c == '\n' {
                    prev_real(self.cursor).unwrap();
                    None
                } else {
                    Some(c)
                }
            }
        }
    }
}

pub enum LineCategory {
    Special(),
    Regular(),
}

/*
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
*/
