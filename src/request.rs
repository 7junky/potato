use std::collections::HashMap;
use std::io::{self, BufReader};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub target: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
}

type Lines<'a> = io::Lines<BufReader<&'a mut TcpStream>>;

impl Request {
    pub fn new(raw: Lines, start_line: String) -> Self {
        let (method, target, http_version) = Self::parse_start_line(start_line);
        let headers = Self::parse_headers(raw);

        Self {
            method,
            target,
            http_version,
            headers,
        }
    }

    fn parse_start_line(start_line: String) -> (String, String, String) {
        let mut start_line_iter = start_line.split_whitespace();

        let method = start_line_iter.next().unwrap();
        let target = start_line_iter.next().unwrap();
        let http_version = start_line_iter.next().unwrap();

        (method.into(), target.into(), http_version.into())
    }

    fn parse_headers(raw: Lines) -> HashMap<String, String> {
        let mut header_map: HashMap<String, String> = HashMap::new();
        for line in raw {
            let line = line.unwrap();

            if line == "" {
                break;
            }

            let (key, value) = line.split_once(": ").unwrap();

            header_map.insert(key.into(), value.into());
        }

        header_map
    }
}

pub enum RequestMethod {
    GET,
    POST,
    PATCH,
    DELETE,
}

impl RequestMethod {
    pub fn to_str(&self) -> &str {
        match self {
            RequestMethod::GET => "GET",
            RequestMethod::POST => "POST",
            RequestMethod::PATCH => "PATCH",
            RequestMethod::DELETE => "DELETE",
        }
    }
}
