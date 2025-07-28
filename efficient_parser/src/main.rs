use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom},
};

mod parser;

struct FileCursor<File: Seek + Read> {
    file: File,
}

impl<File: Seek + Read> parser::Cursor for FileCursor<File> {
    fn back(&mut self) -> bool {
        self.file.seek(SeekFrom::Current(-1)).is_ok()
    }
    fn read(&mut self) -> Option<char> {
        let mut c = 0;
        if self.file.read(std::array::from_mut(&mut c)).unwrap() == 0 {
            return None;
        } else {
            return Some(char::from(c));
        }
    }
    fn end(&mut self) {
        self.file.seek(SeekFrom::End(0)).unwrap();
    }
}

#[derive(Debug)]
pub struct ParsingResult {
    pub room_name: String,
    pub server_ip: String,
    pub username: String,
    pub password: String,
    pub new_message: Option<String>,
}

fn parse(mut line: parser::Line<impl parser::Cursor>) -> Option<ParsingResult> {
    let mut config_kv = HashMap::new();
    loop {
        if let parser::LineCategory::Special() = line.categorize() {
            let line_string: String = (&mut line).collect();
            if let Some((command, arg)) = line_string.split_once('\\') {
                if command == "config" {
                    let (k, v) = arg.split_once('\\')?;
                    config_kv.insert(k.to_owned(), v.to_owned());
                }
            }
        } else {
            break;
        }
        if !line.next_line() {
            break;
        }
    }
    line.last_line();
    let mut new_message_lines = Vec::new();
    loop {
        if let parser::LineCategory::Regular() = line.categorize() {
            let line_string: String = (&mut line).collect();
            new_message_lines.push(line_string);
        } else {
            break;
        }
        if !line.prev_line() {
            break;
        }
    }
    let new_message = if new_message_lines.len() == 0 {
        None
    } else {
        new_message_lines.reverse();
        Some(new_message_lines.join("\n"))
    };
    Some(ParsingResult {
        room_name: config_kv.remove("room name")?,
        server_ip: config_kv.remove("server ip")?,
        username: config_kv.remove("username")?,
        password: config_kv.remove("password")?,
        new_message,
    })
}

fn main() {
    let chatfile_path = std::env::args().nth(1).unwrap();
    let line = parser::Line::new(FileCursor {
        file: std::fs::File::open(chatfile_path).unwrap(),
    });
    println!("{:#?}", parse(line));
}
