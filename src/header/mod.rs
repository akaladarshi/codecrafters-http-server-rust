use std::fmt::Error;
use std::io::Write;

use crate::header::request_header::RequestHeader;
use crate::header::response_header::ResponseHeader;

mod status;
mod methods;
mod errors;
mod request_header;
mod response_header;

const CRLF: &str = "\r\n";
const PROTOCOL_VERSION: &str = "HTTP/1.1";

pub enum Header {
    RequestHeader(RequestHeader),
    ResponseHeader(ResponseHeader),
}
impl Header {
    pub fn request() -> Header{
        Header::RequestHeader(RequestHeader::new())
    }

    pub fn response(code: isize, content_type: &str, content: Vec<u8>) -> Header {
        Header::ResponseHeader(ResponseHeader::new(code, content_type, content))
    }

    pub fn get_path(&self) -> &str {
        match self {
            Header::RequestHeader(req) => req.get_path(),
            _ => ""
        }
    }
}

pub trait Parser {
    fn parse(&mut self, header_data: &str) -> Result<(), Error>;
}

pub trait Serializer {
    fn serialize<W: Write>(&self, writer:  &mut W) -> Result<usize, Error>;
}


impl Parser for Header {
    fn parse(&mut self, header_data: &str) -> Result<(), Error> {
        match self {
            Header::RequestHeader(req_header) => req_header.parse(header_data),
            _ => Err(Default::default())
        }
    }
}

impl Serializer for Header {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        match self {
            Header::ResponseHeader(res_header) => res_header.serialize(writer),
            _ => Err(Default::default())
        }
    }
}