use std::str::FromStr;
use crate::header::errors::HeaderError;

pub enum HTTPMethod {
    GET,
    POST
}

impl FromStr for HTTPMethod {
    type Err = HeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
           "GET" => Ok(HTTPMethod::GET),
           "POST" => Ok(HTTPMethod::POST),
           _ => Err(HeaderError::InvalidMethod)
       }
    }
}

impl HTTPMethod {
    pub fn get_method(m: &str) -> Result<HTTPMethod, HeaderError> {
        m.parse::<HTTPMethod>()
    }

}