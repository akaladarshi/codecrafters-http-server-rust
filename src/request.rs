use std::io;
use std::io::Read;

use crate::header::{Header, HeaderData, Parser};

const CRLF: &str = "\r\n";
pub struct Request {
    header: Header,
    #[allow(dead_code)]
    body: String
}

impl Request {

    pub fn new() -> Self {
        Request {
            header: Header::request(),
            body: String::new()
        }
    }
    pub fn parse_data<R: Read>(&mut self, reader: &mut R) -> Result<(), io::Error> {
        let mut buf = [0; 1048];
        let n = reader.read(&mut buf)?;
        self.process_buffer(&buf[..n])
    }

    fn process_buffer(&mut self, buf: &[u8]) -> Result<(), io::Error>{
        let data_in_str = String::from_utf8_lossy(buf);
        let splitter = format!("{}{}", CRLF, CRLF);
        let mut req_data = data_in_str.split(&splitter);

        // request data will contain
        // Header data will be separated by \r\n
        // but the last value of header and end of header will exist together
        // HEADER \r\n\r\n DATA
        if let Some(header_data) = req_data.next() {
            Parser::parse(&mut self.header, header_data)?
        } else {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Header data is missing"));
        }

        if let Some(body_data) = req_data.next() {
            self.body = body_data.to_string();
        }

        Ok(())
    }

    pub fn get_path(&self) -> &str {
        HeaderData::get_path(&self.header)
    }

    pub fn get_data(&self, key: &str) -> String {
        HeaderData::get_data(&self.header, key)
    }

    pub fn get_method(&self) -> Option<&str> {
        self.header.get_method()
    }

    pub fn get_body(&self) -> &[u8] {
        self.body.as_bytes()
    }
}