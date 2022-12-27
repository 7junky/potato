#[derive(Debug, PartialEq)]
pub enum Status {
    OK,
    Created,
    BadRequest,
    NotFound,
    ImATeaPot,
    Internal,
}

impl Status {
    pub fn to_str(&self) -> &str {
        match self {
            Status::OK => "200 OK",
            Status::Created => "201 Created",
            Status::BadRequest => "400 Bad Request",
            Status::NotFound => "404 Not Found",
            Status::ImATeaPot => "418 I'm a teapot",
            Status::Internal => "500 Internal Server Error",
        }
    }
}
