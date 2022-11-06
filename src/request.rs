use std::collections::HashMap;
use std::io::{self, BufReader};
use std::net::TcpStream;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    pub start_line: String,
    pub method: RequestMethod,
    pub target: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
}

type Lines<'a> = io::Lines<BufReader<&'a mut TcpStream>>;

impl Request {
    pub fn new(mut lines: Lines) -> Self {
        let start_line = lines.next().unwrap().unwrap();
        let (method, target, http_version) = Self::parse_start_line(&start_line);
        let method = RequestMethod::from_str(&method).unwrap();
        let headers = Self::parse_headers(lines);

        Self {
            start_line,
            method,
            target,
            http_version,
            headers,
        }
    }

    fn parse_start_line(start_line: &String) -> (String, String, String) {
        let mut start_line_iter = start_line.split_whitespace();

        let method = start_line_iter.next().unwrap();
        let target = start_line_iter.next().unwrap();
        let http_version = start_line_iter.next().unwrap();

        (method.into(), target.into(), http_version.into())
    }

    fn parse_headers(lines: Lines) -> HashMap<String, String> {
        let mut header_map: HashMap<String, String> = HashMap::new();

        for line in lines {
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

#[derive(Debug, PartialEq)]
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

impl FromStr for RequestMethod {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "GET" => Ok(RequestMethod::GET),
            "POST" => Ok(RequestMethod::POST),
            "PATCH" => Ok(RequestMethod::PATCH),
            "DELETE" => Ok(RequestMethod::DELETE),
            _ => Err(()),
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::{Request, RequestMethod};
//     use std::collections::HashMap;

//     #[test]
//     fn new_request() {
//         let mut headers: HashMap<String, String> = HashMap::new();
//         headers.insert("host".into(), "localhost:7357".into());

//         let request = Request::new("GET / HTTP/1.1".into(), headers);

//         assert_eq!(request.method, RequestMethod::GET);
//         assert_eq!(request.target, "/".to_owned());
//         assert_eq!(request.http_version, "HTTP/1.1".to_owned());

//         assert_eq!(request.headers["host"], "localhost:7357");
//     }
// }
