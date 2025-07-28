pub trait Char: Sized + Copy {
    fn next(&self) -> Option<Self>;
    fn prev(&self) -> Option<Self>;
    fn value(&self) -> char;
}

pub trait FileContents {
    type Char: Char;

    fn beginning(&self) -> Self::Char;
    fn end(&self) -> Self::Char;
}

pub trait File {
    type Contents: FileContents;

    fn contents(&self) -> Option<Self::Contents>;
}

pub struct Password {
    pub password: String,
}

pub struct ParsingResult {
    pub room_name: String,
    pub server_ip: String,
    pub username: String,
    pub password: Password,
    pub new_message: Option<String>,
}

enum NextCharCategory {
    SameLine(),
    OtherLine(),
}

fn categorize_next_char<C: Char>(cur: &C) -> NextCharCategory {
    if cur.value() == '\n' {
        NextCharCategory::OtherLine()
    } else {
        NextCharCategory::SameLine()
    }
}

fn next_line<C: Char>(mut cur: C) -> Option<C> {
    loop {
        let next = cur.next()?;
        match categorize_next_char(&cur) {
            NextCharCategory::SameLine() => cur = next,
            NextCharCategory::OtherLine() => break Some(next),
        }
    }
}

fn find_line_beginning<C: Char>(mut cur: C) -> C {
    loop {
        match cur.prev() {
            None => break,
            Some(prev) => match categorize_next_char(&prev) {
                NextCharCategory::OtherLine() => break,
                NextCharCategory::SameLine() => cur = prev,
            }
        }
    }
    cur
}

fn prev_line<C: Char>(cur: C) -> Option<C> {
    cur.prev().map(|prev| find_line_beginning(prev))
}

fn read_line<C: Char>(mut cur: C) -> String {
    let mut buffer = String::new();
    loop {
        match cur.next() {
            None => break,
            Some(next) => match categorize_next_char(&cur) {
                NextCharCategory::OtherLine() => break,
                NextCharCategory::SameLine() => {
                    buffer.push(next.value());
                    cur = next;
                }
            }
        }
    }
    buffer
}

pub fn parse(file: impl File) -> Option<ParsingResult> {
    pub fn parse(file_contents: impl FileContents) -> Option<ParsingResult> {
        let mut beginning_line = file_contents.beginning();
        loop {
            let next = beginning_line.next();
        }
    }
    file.contents().and_then(|contents| parse(contents))
}
