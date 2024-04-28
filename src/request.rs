use std::fmt::Error;
use crate::header::Header;
use std::io::{Read};
use itertools::Itertools;


const CRLF: &str = "\r\n";
#[allow(dead_code)]
pub struct Request {
    header: Header,
    data: Vec<u8>
}

impl Request {

    pub fn new() -> Self {
        Request {
            header: Header::new(),
            data: vec![]
        }
    }
    pub fn parse_data<R: Read>(&mut self, reader: &mut R) -> Result<(), Error> {
        let mut buf = [0; 1048];
        reader.read(&mut buf).expect("TODO: panic message");
        let data_in_str = String::from_utf8_lossy(&buf);
        let req_data = data_in_str.
            split(&format!("{}{}", CRLF, CRLF)).
            collect_vec();

        // request data will contain
        // Header data will be separated by \r\n
        // but the last value of header and end of header will exist together
        // HEADER \r\n\r\n DATA
        self.header.parse(req_data[0])
    }

    pub fn get_path(&self) -> &str {
        self.header.get_path()
    }
}