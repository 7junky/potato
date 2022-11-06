use std::collections::HashMap;

use crate::status::Status;

#[derive(Debug)]
pub struct Response {
    status: Status,
    headers: HashMap<String, String>,
    content: String,
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: Status::OK,
            headers: HashMap::default(),
            content: "".into(),
        }
    }

    pub fn with_status(&mut self, status: Status) -> &mut Self {
        self.status = status;
        self
    }

    pub fn with_header(&mut self, key: &str, value: &str) -> &mut Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn with_content(&mut self, content: String) -> &mut Self {
        self.content = content;
        self
    }

    pub fn data(&mut self) -> String {
        let mut response = format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n",
            self.status.to_str(),
            self.content.len(),
        );

        let mut headers = String::new();

        for (key, value) in &self.headers {
            headers.push_str(&format!("{}: {}\r\n", key, value))
        }

        response.push_str(&headers);
        response.push_str("\r\n");
        response.push_str(&self.content);

        response
    }
}
