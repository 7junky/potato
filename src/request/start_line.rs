use super::{method::Method, request::ParseError};
use std::str::FromStr;

#[derive(Debug)]
pub(super) struct StartLine {
    pub(super) line: String,
    pub(super) method: Method,
    pub(super) target: String,
    pub(super) version: String,
}

impl StartLine {
    pub fn from_request(line: &str) -> Result<Self, ParseError> {
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
}
