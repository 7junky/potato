#[derive(Debug, PartialEq)]
pub enum Status {
    OK,
    BadRequest,
    NotFound,
    Internal,
}

impl Status {
    pub fn to_str(&self) -> &str {
        match self {
            Status::OK => "200 OK",
            Status::BadRequest => "400 Bad Request",
            Status::NotFound => "404 Not Found",
            Status::Internal => "500 Internal Server Error",
        }
    }
}
