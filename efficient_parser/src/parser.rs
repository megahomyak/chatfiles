#[derive(Debug)]
pub struct ParsingResult {
    pub room_name: String,
    pub server_ip: String,
    pub username: String,
    pub password: String,
    pub new_message: Option<String>,
}

pub trait Cursor {
    fn back(&mut self) -> bool;
    fn read(&mut self) -> Option<char>;
    fn end(&mut self);
}

// starts at line beginning, ends at line beginning on success or wherever on error
fn next_line(cursor: &mut impl Cursor) -> bool {
    while let Some(c) = cursor.read() {
        if c == '\n' {
            return true;
        }
    }
    false
}

// starts wherever, ends at line beginning
fn line_beginning(cursor: &mut impl Cursor) {
    while cursor.back() && cursor.read().unwrap() != '\n' {
        cursor.back();
    }
}

// starts at line beginning, ends at line beginning on success or wherever on error
fn prev_line(cursor: &mut impl Cursor) -> bool {
    if !cursor.back() {
        return false;
    }
    line_beginning(cursor);
    true
}

// starts wherever, ends after pattern on success or where started on error
fn match_str(cursor: &mut impl Cursor, pattern: &'static str) -> bool {
    let mut distance = 0;
    'iteration: {
        for c in pattern.chars() {
            match cursor.read() {
                Some(next) => {
                    distance += 1;
                    if next != c {
                        break 'iteration;
                    }
                },
                None => break 'iteration,
            }
        }
        return true;
    };
    for _ in 0..distance {
        cursor.back();
    }
    false
}

// starts wherever, ends in the same line at the line separator
fn read_line(cursor: &mut impl Cursor) -> String {
    let mut buffer = String::new();
    while let Some(c) = cursor.read() {
        if c == '\n' {
            cursor.back();
            break;
        } else {
            buffer.push(c);
        }
    }
    buffer
}

enum LineCategory {
    Special(),
    Regular(),
}

// starts at line beginning, ends at line category's contents
fn categorize_line(cursor: &mut impl Cursor) -> LineCategory {
    if match_str(cursor, "\\\\") {
        cursor.back();
    } else if match_str(cursor, "\\") {
        return LineCategory::Special();
    }
    LineCategory::Regular()
}

// starts at chatfile beginning, ends wherever
pub fn parse(beginning: &mut impl Cursor) -> Option<ParsingResult> {
    let mut room_name = None;
    let mut server_ip = None;
    let mut username = None;
    let mut password = None;
    loop {
        let mut line_read = false;
        struct LineChecker<'a, C: Cursor> {
            line_read: &'a mut bool,
            cursor: &'a mut C,
        }
        impl<'a, C: Cursor> LineChecker<'a, C> {
            fn check_line(&mut self, receiver: &mut Option<String>, pattern: &'static str) -> bool {
                let matched = match_str(self.cursor, pattern);
                if matched {
                    *receiver = Some(read_line(self.cursor));
                    self.cursor.read();
                    *self.line_read = true;
                }
                matched
            }
        }
        if let LineCategory::Special() = categorize_line(beginning) {
            if match_str(beginning, "config\\") {
                let mut line_reader = LineChecker { line_read: &mut line_read, cursor: beginning };
                if line_reader.check_line(&mut room_name, "room name\\") {
                } else if line_reader.check_line(&mut server_ip, "server ip\\") {
                } else if line_reader.check_line(&mut username, "username\\") {
                } else if line_reader.check_line(&mut password, "password\\") {
                }
            }
        }
        if !line_read && !next_line(beginning) {
            break;
        }
    };
    let room_name = room_name?;
    let username = username?;
    let password = password?;
    let server_ip = server_ip?;
    beginning.end();
    let end = beginning;
    line_beginning(end);
    if !end.read().is_some() {
        end.back();
    }
    line_beginning(end);
    let mut new_message_lines = Vec::new();
    loop {
        match categorize_line(end) {
            LineCategory::Special() => break,
            LineCategory::Regular() => new_message_lines.push(read_line(end)),
        }
        line_beginning(end);
        if !prev_line(end) {
            break;
        }
    }
    new_message_lines.reverse();
    let new_message = if new_message_lines.len() == 0 {
        None
    } else {
        Some(new_message_lines.join("\n"))
    };
    Some(ParsingResult {
        room_name,
        username,
        password,
        server_ip,
        new_message,
    })
}
