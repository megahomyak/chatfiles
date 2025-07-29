pub trait Cursor {
    fn prev(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;
    fn end(&mut self);
}

pub enum LineKind {
    Special(),
    Regular(),
}

pub struct LineIter<'a, C: Cursor> {
    cursor: &'a mut C,
}

impl<'a, C: Cursor> Iterator for LineIter<'a, C> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        next_real(self.cursor).and_then(|c| {
            if c == '\n' {
                prev_real(self.cursor);
                None
            } else {
                Some(c)
            }
        })
    }
}

pub struct ForwardLine<C: Cursor> {
    cursor: C,
    kind: LineKind,
}

fn prev_real(cursor: &mut impl Cursor) -> Option<char> {
    cursor
        .prev()
        .and_then(|c| if c == '\r' { cursor.prev() } else { Some(c) })
}

fn next_real(cursor: &mut impl Cursor) -> Option<char> {
    cursor
        .next()
        .and_then(|c| if c == '\r' { cursor.next() } else { Some(c) })
}

fn line_beginning(cursor: &mut impl Cursor) {
    while let Some(c) = cursor.prev() {
        if c == '\n' {
            cursor.next().unwrap();
            break;
        }
    }
}

fn kind(cursor: &mut impl Cursor) -> LineKind {
    if next_real(cursor) == Some('\\') {
        if next_real(cursor) == Some('\\') {
            prev_real(cursor).unwrap();
        } else {
            return LineKind::Special();
        }
    }
    LineKind::Regular()
}

impl<C: Cursor> ForwardLine<C> {
    pub fn new(mut cursor: C) -> Self {
        let kind = kind(&mut cursor);
        Self { kind, cursor }
    }
    pub fn kind(&self) -> &LineKind {
        &self.kind
    }
    pub fn next(mut self) -> Result<Self, BackLine<C>> {
        while let Some(c) = next_real(&mut self.cursor) {
            if c == '\n' {
                let ended = self.cursor.next().is_none();
                self.cursor.prev().unwrap();
                if ended {
                    return Err(BackLine::new(self.cursor));
                }
                if self.cursor.next().is_some() {
                    self.cursor.prev().unwrap();
                    break;
                } else {
                    return Err(self.end());
                }
            }
        }
        let _ = self.cursor.prev();
    }
    pub fn iter<'a>(&'a mut self) -> LineIter<'a, C> {
        LineIter {
            cursor: &mut self.cursor,
        }
    }
    pub fn end(mut self) -> BackLine<C> {
        self.cursor.end();
        self.cursor.prev().unwrap();
        line_beginning(&mut self.cursor);
        BackLine {
            cursor: self.cursor,
        }
    }
}

pub struct BackLine<C: Cursor> {
    cursor: C,
    kind: LineKind,
}

impl<C: Cursor> BackLine<C> {
    fn new(mut cursor: C) -> Self {
        line_beginning(&mut cursor);
        let kind = kind(&mut cursor);
        Self { kind, cursor }
    }
    pub fn prev(self) -> Option<Self> {

    }
    pub fn iter<'a>(&'a mut self) -> LineIter<'a, C> {
        LineIter {
            cursor: &mut self.cursor,
        }
    }
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
