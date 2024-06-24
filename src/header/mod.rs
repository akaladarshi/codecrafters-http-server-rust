use std::io;
use std::io::Write;

use crate::header::request_header::RequestHeader;
use crate::header::response_header::ResponseHeader;

mod status;
mod methods;
mod errors;
mod request_header;
mod response_header;
mod encoding;

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

    pub fn response(code: isize, encoding_type: Vec<&str>, content_type: &str, content: Vec<u8>) -> Header {
        Header::ResponseHeader(ResponseHeader::new(code, encoding_type, content_type, content))
    }
}

pub trait HeaderData {
    fn get_path(&self) -> &str;
    fn get_data(&self, key: &str) -> String;

    fn get_method(&self) -> Option<&str>;
}

pub trait Parser {
    fn parse(&mut self, header_data: &str) -> Result<(), io::Error>;
}

pub trait Serializer {
    fn serialize<W: Write>(&self, writer:  &mut W) -> Result<usize, io::Error>;
}


impl Parser for Header {
    fn parse(&mut self, header_data: &str) -> Result<(), io::Error> {
        match self {
            Header::RequestHeader(req_header) => req_header.parse(header_data),
            _ => Err(io::Error::new(io::ErrorKind::Other, format!("{}", "failed to parse header")))
        }
    }
}

impl HeaderData for Header {
    fn get_path(&self) -> &str {
        match self {
            Header::RequestHeader(req) => req.get_path(),
            _ => ""
        }
    }

    fn get_data(&self, key: &str) -> String {
        match self {
            Header::RequestHeader(req) => req.get_data(key),
            _ => "".to_string()
        }
    }

    fn get_method(&self) -> Option<&str> {
        match self {
            Header::RequestHeader(req) => Some(req.get_method()),
            _ => None,
        }
    }
}

impl Serializer for Header {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<usize, io::Error> {
        match self {
            Header::ResponseHeader(res_header) => res_header.serialize(writer),
            _ => Err(io::Error::new(io::ErrorKind::Other, format!("{}", "failed to serialize header")))
        }
    }
}
