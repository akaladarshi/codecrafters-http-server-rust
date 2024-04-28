
use std::fmt::Error;

use std::io::Write;
use nom::AsBytes;


use crate::header::Header;

const CRLF: &str = "\r\n";

pub struct Response {
    header: Header,
    data: Vec<u8>
}

impl Response {
    pub fn create_response(status: isize, data: Vec<u8> ) -> Response {
        Response {
            header: Header::new_with_status(status),
            data
        }
    }

    pub fn write<W: Write>(&self, writer:  &mut W) -> Result<(), Error> {
        self.header.write(writer)?;

        // write end of header CRLF
        writer.write(CRLF.as_bytes()).map_err(|_| Error)?;

        // write header data
        writer.write_all(self.data.as_bytes()).map_err(|_| Error)
    }
}