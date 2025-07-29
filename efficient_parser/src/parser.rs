pub trait Cursor {
    fn prev(&mut self) -> bool;
    fn next(&mut self) -> Option<char>;
    fn end(&mut self);
}

mod correct_cursor {
    /// Makes any cursor give out no '\r's and no final newline
    pub struct CorrectCursor<C: super::Cursor> {
        faulty_cursor: C,
    }

    impl<C: super::Cursor> CorrectCursor<C> {
        pub fn new(faulty_cursor: C) -> Self {
            Self { faulty_cursor }
        }
        fn faulty_prev_char(&mut self) -> Option<char> {
            if self.faulty_cursor.prev() {
                let c = self.faulty_cursor.next().unwrap();
                self.faulty_cursor.prev();
                Some(c)
            } else {
                None
            }
        }
        pub fn next(&mut self) -> Option<char> {
            while let Some(c) = self.faulty_cursor.next() {
                if c != '\r' {
                    if c == '\n' {
                        let ended = self.faulty_cursor.next().is_none();
                        self.faulty_cursor.prev();
                        if ended {
                            return None;
                        }
                    }
                    return Some(c);
                }
            }
            None
        }
        pub fn prev(&mut self) -> Option<char> {
            while let Some(c) = self.faulty_prev_char() {
                if c != '\r' {
                    return Some(c);
                }
            }
            None
        }
        pub fn end(&mut self) {
            self.faulty_cursor.end();
            if self.faulty_prev_char().is_some_and(|c| c != '\n') {
                self.faulty_cursor.next().unwrap();
            }
        }
    }
}
use correct_cursor::CorrectCursor;

pub enum LineKind {
    Regular(),
    Special(),
}

pub struct Line<C: Cursor> {
    cursor: CorrectCursor<C>,
    pub kind: LineKind,
}

impl<C: Cursor> Line<C> {
    fn from_line_beginning(mut cursor: CorrectCursor<C>) -> Self {
        let mut kind = LineKind::Regular();
        if let Some(c) = cursor.next() {
            if c == '\\' {
                kind = LineKind::Special();
                if let Some(c) = cursor.next() {
                    if c == '\\' {
                        kind = LineKind::Regular();
                    }
                    cursor.prev().unwrap();
                }
            } else {
                cursor.prev().unwrap();
            }
        }
        Self { kind, cursor }
    }
    pub fn iter(&mut self) -> LineIter<C> {
        LineIter { cursor: &mut self.cursor }
    }
}

pub struct ForwardLine<C: Cursor> {
    pub contents: Line<C>,
}

pub struct BackLine<C: Cursor> {
    pub contents: Line<C>,
}

pub struct LineIter<'a, C: Cursor> {
    cursor: &'a mut CorrectCursor<C>,
}

impl<'a, C: Cursor> Iterator for LineIter<'a, C> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.next().and_then(|c| if c == '\n' {
            self.cursor.prev().unwrap();
            None
        } else {
            Some(c)
        })
    }
}

impl<C: Cursor> ForwardLine<C> {
    pub fn from_text_beginning(cursor: C) -> Self {
        Self { contents: Line::from_line_beginning(CorrectCursor::new(cursor)) }
    }
    pub fn next(mut self) -> Result<Self, BackLine<C>> {
        while let Some(c) = self.contents.cursor.next() {
            if c == '\n' {
                return Ok(ForwardLine {
                    contents: Line::from_line_beginning(self.contents.cursor),
                });
            }
        }
        let kind = Some(self.contents.kind);
        Err(BackLine::from_inside_line(self.contents.cursor, kind))
    }
    pub fn end(mut self) -> BackLine<C> {
        self.contents.cursor.end();
        let kind = None;
        BackLine::from_inside_line(self.contents.cursor, kind)
    }
}

impl<C: Cursor> BackLine<C> {
    fn from_inside_line(mut cursor: CorrectCursor<C>, kind: Option<LineKind>) -> Self {
        while let Some(c) = cursor.prev() {
            if c == '\n' {
                cursor.next().unwrap();
                break;
            }
        }
        Self {
            contents: match kind {
                None => Line::from_line_beginning(cursor),
                Some(kind) => Line { cursor, kind },
            },
        }
    }
    pub fn prev(mut self) -> Option<BackLine<C>> {
        while let Some(c) = self.contents.cursor.prev() {
            if c == '\n' {
                let kind = None;
                return Some(Self::from_inside_line(self.contents.cursor, kind));
            }
        }
        None
    }
}
