use std::collections::HashMap;
use std::fmt::Error;

use itertools::Itertools;

use crate::header::{CRLF, PROTOCOL_VERSION};
use crate::header::errors::HeaderError;
use crate::header::methods::HTTPMethod;

pub struct RequestHeader {
    protocol_version: String,
    method: HTTPMethod,
    path: String,
    data: HashMap<String, String>
}

impl RequestHeader {
    pub fn new() -> RequestHeader {
        RequestHeader{
            protocol_version: PROTOCOL_VERSION.to_string(), // TODO: implement default
            method: HTTPMethod::GET, // TODO: implement default
            path: "".to_string(), // TODO: implement default
            data: Default::default(),
        }
    }

    // parse parses the header data
    // all the header data is separated by CRLF.
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
                self.data.insert(key.trim().to_lowercase(), value.trim().to_lowercase());
            }
        }

        Ok(())
    }


    // parse_status_line parse the first line of the header data which should contains
    // HTTPMethod path Protocol Version
    fn parse_status_line(&mut self, status_line: &str) -> Result<(), HeaderError> {
        let data = status_line.split(" ").collect_vec();
        if data.is_empty() {
            return Err(HeaderError::NoStatusLine)
        }

        let mut data_itr = data.into_iter();
        self.method = HTTPMethod::get_method(data_itr.next().unwrap())?;
        self.path = data_itr.next().unwrap().to_string();
        let version = data_itr.next().unwrap().to_string();
        if !version.eq(PROTOCOL_VERSION) {
            return Err(HeaderError::InvalidProtocolVersion)
        }

        self.protocol_version = version;
        Ok(())
    }

    pub fn get_data(&self, key: &str) -> String {
        match self.data.get(key) {
            None => "".to_string(),
            Some(s) => s.to_string()
        }
    }

    pub fn get_path(&self) -> &str {
        self.path.as_str()
    }
}
