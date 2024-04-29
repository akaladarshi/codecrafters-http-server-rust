use std::str::FromStr;

use crate::constants::{HTTP_GET, HTTP_POST};
use crate::header::errors::HeaderError;

pub enum HTTPMethod {
    GET,
    POST
}

impl FromStr for HTTPMethod {
    type Err = HeaderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
           HTTP_GET => Ok(HTTPMethod::GET),
           HTTP_POST => Ok(HTTPMethod::POST),
           _ => Err(HeaderError::InvalidMethod)
       }
    }
}

impl HTTPMethod {
    pub fn get_method(m: &str) -> Result<HTTPMethod, HeaderError> {
        m.parse::<HTTPMethod>()
    }

    pub fn string(&self) -> &str {
        match *self {
            HTTPMethod::GET => HTTP_GET,
            HTTPMethod::POST => HTTP_POST
        }
    }
}