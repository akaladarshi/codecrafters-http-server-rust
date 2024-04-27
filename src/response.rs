use crate::header::Header;

use std::io::{self, Write};
use nom::AsBytes;

const CRLF: &str = "\r\n";

pub struct Response {
    header: Header,
    data: Vec<u8>
}

impl Response {
    pub fn create_ok_response_with_data(data: Vec<u8>) -> Response{
        Response{
            header: Header::new_with_ok_status(),
            data,
        }
    }
    pub fn write<W: Write>(&self, writer:  &mut W) -> io::Result<()> {
        self.header.write(writer)?;

        // write end of header CRLF
        writer.write(CRLF.as_bytes())?;

        // write header data
        writer.write_all(self.data.as_bytes())
    }
}