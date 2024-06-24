use std::io;
use std::io::Write;

use crate::header::{CRLF, PROTOCOL_VERSION};
use crate::header::encoding::EncodingScheme;
use crate::header::status::Status;

pub struct ResponseHeader {
    protocol_version: String,
    status: Status,
    content_type: String,
    content: Vec<u8>,
    content_encoding: Option<EncodingScheme>
}

impl ResponseHeader {
    pub fn new(code: isize, encoding_types: Vec<&str>, content_type: &str, content: Vec<u8>) -> ResponseHeader {
        ResponseHeader{
            protocol_version: PROTOCOL_VERSION.to_string(),
            status:  Status::get_status(code).unwrap(),
            content_type: content_type.to_string(),
            content,
            content_encoding: encoding_types
                .iter()
                .find_map(|&encoding_type| EncodingScheme::get_encoding_scheme(encoding_type.trim()))
        }
    }
    pub fn serialize<W: Write>(&self, writer:  &mut W) -> Result<usize, io::Error> {
        // write status line with end of status CRLF
        let n = writer.write(format!("{} {}{}", self.protocol_version, self.status.string(), CRLF).as_bytes())?;
        if self.content.is_empty() {
            return Ok(n)
        }

        if let Some(content_encoding) = self.content_encoding.as_ref() {
           writer.write(format!("{}: {}{}", "Content-Encoding", content_encoding.string(), CRLF).as_bytes())?;
        }

        writer.write(format!("{}: {}{}", "Content-Type", self.content_type, CRLF).as_bytes())?;
        writer.write(format!("{}: {}{}", "Content-Length", self.content.len(), CRLF).as_bytes())
    }

}