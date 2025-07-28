use std::io::{Read, Seek, SeekFrom};

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

fn main() {
    let chatfile_path = std::env::args().nth(1).unwrap();
    let mut beginning = FileCursor {
        file: std::fs::File::open(chatfile_path).unwrap(),
    };
    println!("{:#?}", parser::parse(&mut beginning));
}
