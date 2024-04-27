
#[derive(Debug)]
pub enum Status {
    Ok = 200,
    NotFound = 404
}

impl Status {
    pub fn get_status(&self) -> &str {
        match self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not Found"
        }
    }
}