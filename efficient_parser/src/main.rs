use std::io::{Read, Seek, SeekFrom};

mod parser;

struct FileCursor<File: Seek + Read> {
    file: File,
}

impl<File: Seek + Read> parser::Cursor for FileCursor<File> {
    fn prev(&mut self) -> bool {
        self.file.seek(SeekFrom::Current(-1)).is_ok()
    }
    fn next(&mut self) -> Option<char> {
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

fn parse(mut forward_line: parser::ForwardLine<impl parser::Cursor>) -> Option<ParsingResult> {
    let mut room_name = None;
    let mut server_ip = None;
    let mut username = None;
    let mut password = None;
    let (room_name, server_ip, username, password, mut back_line) = loop {
        if let parser::LineKind::Special() = forward_line.contents.kind {
            let line_string: String = forward_line.contents.iter().collect();
            if let Some((command, arg)) = line_string.split_once('\\') {
                if command == "config" {
                    let (k, v) = arg.split_once('\\')?;
                    let v = v.to_owned();
                    if k == "room name" {
                        room_name = Some(v);
                    } else if k == "server ip" {
                        server_ip = Some(v);
                    } else if k == "username" {
                        username = Some(v);
                    } else if k == "password" {
                        password = Some(v);
                    }
                    if room_name.is_some() && server_ip.is_some() && username.is_some() && password.is_some() {
                        break (room_name.unwrap(), server_ip.unwrap(), username.unwrap(), password.unwrap(), forward_line.end());
                    }
                }
            }
        }
        forward_line = forward_line.next().ok()?;
    };
    let mut new_message_lines = Vec::new();
    loop {
        if let parser::LineKind::Regular() = back_line.contents.kind {
            new_message_lines.push(back_line.contents.iter().collect::<String>());
        } else {
            break;
        }
        back_line = match back_line.prev() {
            None => break,
            Some(back_line) => back_line,
        };
    }
    let new_message = if new_message_lines.len() == 0 {
        None
    } else {
        new_message_lines.reverse();
        Some(new_message_lines.join("\n"))
    };
    Some(ParsingResult {
        room_name,
        server_ip,
        username,
        password,
        new_message,
    })
}

fn main() {
    let chatfile_path = std::env::args().nth(1).unwrap();
    let forward_line = parser::ForwardLine::from_text_beginning(FileCursor {
        file: std::fs::File::open(chatfile_path).unwrap(),
    });
    println!("{:#?}", parse(forward_line));
}
