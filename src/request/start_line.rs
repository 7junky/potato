use super::{method::Method, request::ParseError};
use std::str::FromStr;

#[derive(Debug)]
pub(super) struct StartLine {
    line: String,
    method: Method,
    target: String,
    version: String,
}

impl StartLine {
    pub fn new(method: Method, target: &str, version: &str) -> Self {
        let line = format!("{:?} {} {}", method, target, version);

        Self {
            line,
            method,
            target: target.to_owned(),
            version: version.to_owned(),
        }
    }

    pub fn from_line(line: &str) -> Result<Self, ParseError> {
        let mut line_iter = line.split_whitespace();

        let method = match line_iter.next() {
            Some(m) => Method::from_str(m)?,
            None => Err(ParseError::NoMethod)?,
        };

        let target = match line_iter.next() {
            Some(m) => m.to_owned(),
            None => Err(ParseError::NoTarget)?,
        };

        let version = match line_iter.next() {
            Some(m) => m.to_owned(),
            None => Err(ParseError::NoVersion)?,
        };

        Ok(Self {
            line: line.to_owned(),
            method,
            target,
            version,
        })
    }

    pub fn line(&self) -> &String {
        &self.line
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn version(&self) -> &String {
        &self.version
    }
}

impl Default for StartLine {
    fn default() -> Self {
        Self {
            line: String::from("GET / HTTP/1.1"),
            method: Method::GET,
            target: String::from("/"),
            version: String::from("HTTP/1.1"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Method;

    use super::StartLine;

    #[test]
    fn test_from_request() {
        let line = "GET /path/to/resource?a=1&b=2 HTTP/2".to_owned();

        let res = StartLine::from_line(&line).unwrap();

        assert_eq!(res.line(), &line);
        assert_eq!(res.method(), &Method::GET);
        assert_eq!(res.target(), "/path/to/resource?a=1&b=2");
        assert_eq!(res.version(), &"HTTP/2");
    }
}
