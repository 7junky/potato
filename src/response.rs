use crate::status::Status;

#[derive(Debug)]
pub struct Response {
    status: Status,
    content: String,
}

impl Response {
    pub fn with_status(&mut self, status: Status) -> &mut Self {
        self.status = status;
        self
    }

    pub fn with_content(&mut self, content: String) -> &mut Self {
        self.content = content;
        self
    }

    pub fn data(&mut self) -> String {
        format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status.to_str(),
            self.content.len(),
            self.content
        )
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: Status::OK,
            content: String::from("OK"),
        }
    }
}
