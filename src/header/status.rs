use crate::constants::*;

#[derive(Debug)]
pub enum Status {
    Ok = HTTP_STATUS_OK,
    NotFound = HTTP_STATUS_NOT_FOUND,
    Created = HTTP_STATUS_CREATED,
}

impl Status {
    pub fn string(&self) -> &str {
        match self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not Found",
            Status::Created => "201",
        }
    }

    pub fn get_status(code: isize)  -> Option<Status> {
        match code {
            HTTP_STATUS_OK => Some(Status::Ok),
            HTTP_STATUS_NOT_FOUND => Some(Status::NotFound),
            HTTP_STATUS_CREATED => Some(Status::Created),
            _ => None
        }
    }
}