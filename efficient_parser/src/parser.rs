pub trait Cursor {
    fn prev(&mut self) -> Option<char>;
    fn next(&mut self) -> Option<char>;
    fn end(&mut self);
}

/// Makes any cursor contain no '\r's and no final newline
struct CorrectCursor<C: Cursor> {
    faulty_cursor: C,
}

fn next_real(cursor: &mut impl Cursor) -> Option<char> {
    while let Some(c) = cursor.next() {
        if c != '\r' {
            if c == '\n' {
                let ended = cursor.next().is_none();
                cursor.prev().unwrap();
                if ended {
                    return None;
                }
            }
            return Some(c);
        }
    }
    None
}

fn prev_real(cursor: &mut impl Cursor) -> Option<char> {
    while let Some(c) = cursor.prev() {
        if c != '\r' {
            return Some(c);
        }
    }
    None
}

fn real_end(cursor: &mut impl Cursor) {
    cursor.end();
    if cursor.prev().is_some_and(|c| c != '\n') {
        cursor.next().unwrap();
    }
}

pub struct ForwardLine<C: Cursor> {
    cursor: C,
}

pub struct BackLine<C: Cursor> {
    cursor: C,
}

pub struct LineIter<'a, C: Cursor> {
    cursor: &'a mut C,
}

impl<C: Cursor> ForwardLine<C> {
    pub fn new(cursor: C) -> Self {
        Self { cursor }
    }
    pub fn next(self) -> Result<Self, BackLine<C>> {

    }
    pub fn end(self) -> BackLine<C> {
        real_end(&mut self.cursor);
    }
    pub fn iter<'a>(&'a mut self) -> LineIter<'a, C> {
        LineIter { cursor: &mut self.cursor }
    }
}

impl<C: Cursor> BackLine<C> {
    fn new(mut cursor: C) -> Self {
        while let Some(c) = prev_real(&mut cursor) {
            if c == '\n' {
                next_real(&mut cursor).unwrap();
                break;
            }
        }
        Self { cursor }
    }
    pub fn prev(self) -> Option<BackLine<C>> {

    }
    pub fn iter<'a>(&'a mut self) -> LineIter<'a, C> {
        LineIter { cursor: &mut self.cursor }
    }
}
