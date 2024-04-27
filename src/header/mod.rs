mod status;

use std::io;
use std::io::Write;
use status::Status;

const CRLF: &str = "\r\n";
const PROTOCOL_VERSION: &str = "HTTP/1.1";

pub struct Header {
    pub status: Status,
    pub data: Vec<u8>
}

impl Header {

    pub fn new() -> Header {
        Header {
            status: Status::NotFound,
            data: vec![],
        }
    }
    pub fn new_with_ok_status() -> Header {
        let mut header = Header::new();
        header.status = Status::Ok;
        return header
    }

    pub fn write<W: Write>(&self, writer:  &mut W) -> io::Result<usize>{
        // write status line with end of status CRLF
        writer.write(format!("{} {}{}", PROTOCOL_VERSION, self.status.get_status(), CRLF).as_bytes())
    }
}