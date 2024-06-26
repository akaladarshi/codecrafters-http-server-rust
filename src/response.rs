use std::io;
use std::io::Write;

use crate::body::Body;
use crate::header::{Header, Serializer};

const CRLF: &str = "\r\n";

pub struct Response {
    header: Header,
    body: Body
}

impl Response {
    pub fn create_response(status: isize, encoding_types: Vec<&str>, body: Body ) -> Response {
        Response {
            header: Header::response(status, encoding_types, body.get_content_type(), body.get_content()),
            body
        }
    }

    pub fn write<W: Write>(&self, writer:  &mut W) -> Result<(), io::Error> {
        Serializer::serialize(&self.header, writer)?;

        // write end of header CRLF
        writer.write(CRLF.as_bytes())?;

        // write response body
        self.body.write(writer)?;

        Ok(())
    }
}