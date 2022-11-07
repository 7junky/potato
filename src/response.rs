use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::status::Status;

#[derive(Debug)]
pub struct Response {
    status: Status,
    headers: HashMap<String, String>,
    cookies: Vec<String>,
    content: String,
}

pub struct Cookie<'a> {
    pub key: &'a str,
    pub value: &'a str,
    pub expires: Option<DateTime<Utc>>,
    pub secure: bool,
    pub http_only: bool,
}

impl<'a> Cookie<'a> {
    fn to_string(&self) -> String {
        let mut cookie = format!("{}={}", self.key, self.value);

        if let Some(expires) = self.expires {
            let expires = expires.to_rfc2822();
            cookie.push_str(&format!("; Expires={}", expires));
        };

        if self.secure {
            cookie.push_str("; Secure")
        };

        if self.http_only {
            cookie.push_str("; HttpOnly")
        }

        cookie
    }
}

impl Response {
    pub fn new() -> Self {
        Self {
            status: Status::OK,
            headers: HashMap::default(),
            cookies: Vec::new(),
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

    pub fn with_cookie(&mut self, cookie: Cookie) -> &mut Self {
        let cookie = cookie.to_string();
        self.cookies.push(cookie);
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

        for cookie in &self.cookies {
            headers.push_str(&format!("Set-Cookie: {}\r\n", cookie))
        }

        response.push_str(&headers);
        response.push_str("\r\n");
        response.push_str(&self.content);
        dbg!(&response);
        response
    }
}
