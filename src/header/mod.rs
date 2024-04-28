mod status;
mod methods;
mod errors;

use std::collections::HashMap;
use std::fmt::Error;
use std::io::Write;
use std::string::ToString;
use itertools::Itertools;
use status::Status;
use crate::header::errors::HeaderError;
use crate::header::methods::HTTPMethod;

const CRLF: &str = "\r\n";
const PROTOCOL_VERSION: &str = "HTTP/1.1";

pub struct Header {
    protocol_version: String,
    status: Status,
    method: HTTPMethod,
    path: String,
    data: HashMap<String, String>
}

impl Header {

    pub fn new() -> Header {
        Header {
            protocol_version: PROTOCOL_VERSION.to_string(),
            status: Status::NotFound,
            method: HTTPMethod::GET,
            path: "".to_string(),
            data: HashMap::new(),
        }
    }

    pub fn new_with_status(code: isize) -> Header {
        let mut h = Header::new();
        h.status = Status::get_status(code).unwrap();
        return h
    }

    pub fn write<W: Write>(&self, writer:  &mut W) -> Result<usize, Error>{
        // write status line with end of status CRLF
        writer.write(format!("{} {}{}", self.protocol_version, self.status.string(), CRLF).as_bytes()).map_err(|_| Error)
    }

    pub fn parse(&mut self, header_data: &str) -> Result<(), Error> {
        let data = header_data.split(CRLF).collect_vec();
        if data.is_empty() {
            return Err(HeaderError::NoHeaderFound.into())
        }

        // first element will be status line
        self.parse_status_line(data[0])?;

        // rest will be header data
        for d in data.into_iter() {
            if let Some((key, value)) = d.split_once(":") {
                self.data.insert(key.to_string(), value.to_string());
            }
        }

        Ok(())
    }

    fn parse_status_line(&mut self, status_line: &str) -> Result<(), HeaderError> {
        let data = status_line.split(" ").collect_vec();
        if data.is_empty() {
            return Err(HeaderError::NoStatusLine)
        }

        let mut data_itr = data.into_iter();
        self.method = HTTPMethod::get_method(data_itr.next().unwrap())?;
        self.path = data_itr.next().unwrap().to_string();
        if !data_itr.next().unwrap().eq(PROTOCOL_VERSION) {
            return Err(HeaderError::InvalidProtocolVersion)
        }

        Ok(())
    }

    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }
}