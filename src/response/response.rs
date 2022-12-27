use std::collections::HashMap;

use super::cookie::Cookie;
use super::status::Status;

#[derive(Debug)]
pub struct Response {
    status: Status,
    headers: HashMap<String, String>,
    cookies: Vec<String>,
    content: String,
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

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn cookies(&self) -> &Vec<String> {
        &self.cookies
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn to_string(&self) -> String {
        let mut raw = format!(
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

        raw.push_str(&headers);
        raw.push_str("\r\n");
        raw.push_str(&self.content);

        raw
    }
}

#[cfg(test)]
mod test {
    use super::{Cookie, Response};

    use chrono::prelude::*;

    #[test]
    fn it_works() {
        let expected = "HTTP/1.1 200 OK\r\n\
Content-Length: 18\r\n\
Content-Type: text/html\r\n\
Set-Cookie: darkmode=true; Secure; HttpOnly\r\n\
Set-Cookie: token=abcdefg; Expires=Thu, 01 Dec 2022 12:00:00 +0000; Secure; HttpOnly\r\n\r\n\
<h1> Welcome </h1>";

        let mut response = Response::new();

        response
            .with_header("Content-Type", "text/html")
            .with_cookie(Cookie {
                key: "darkmode",
                value: "true",
                expires: None,
                secure: true,
                http_only: true,
            })
            .with_cookie(Cookie {
                key: "token",
                value: "abcdefg",
                expires: Some(chrono::Utc.ymd(2022, 12, 1).and_hms(12, 00, 00)),
                secure: true,
                http_only: true,
            })
            .with_content("<h1> Welcome </h1>".to_owned());

        assert_eq!(response.to_string(), expected);
    }
}
