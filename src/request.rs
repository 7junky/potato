use std::collections::HashMap;
use std::io::{self, BufReader};
use std::net::TcpStream;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    start_line: String,
    method: RequestMethod,
    target: String,
    http_version: String,
    headers: HashMap<String, String>,
    params: HashMap<String, String>,
    route_key: Option<String>,
}

type Lines<'a> = io::Lines<BufReader<&'a mut TcpStream>>;

impl Request {
    pub fn new(mut lines: Lines) -> Self {
        let start_line = lines.next().unwrap().unwrap();
        let (method, target, http_version) =
            Self::parse_start_line(&start_line);
        let method = RequestMethod::from_str(&method).unwrap();
        let headers = Self::parse_headers(lines);
        let (path, params) = Self::parse_params(&target);
        let route_key = Self::generate_route_key(&method, path, &http_version);

        Self {
            start_line,
            method,
            target,
            http_version,
            headers,
            params,
            route_key,
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

    fn parse_params(target: &String) -> (&str, HashMap<String, String>) {
        let mut params = HashMap::new();

        let (path, raw_params) = match target.split_once("?") {
            Some(params) => params,
            None => return ("", params),
        };

        for param in raw_params.rsplit("&") {
            let (key, value) = match param.split_once("=") {
                Some(kv) => kv,
                None => continue,
            };

            params.insert(key.into(), value.into());
        }

        (path, params)
    }

    fn generate_route_key(
        method: &RequestMethod,
        path: &str,
        http_version: &String,
    ) -> Option<String> {
        if path == "" {
            return None;
        };

        Some(format!("{:?} {} {}", method, path, http_version))
    }

    pub fn get_route_key(&self) -> &String {
        match &self.route_key {
            Some(route_key) => route_key,
            None => &self.start_line,
        }
    }

    pub fn get_start_line(&self) -> &String {
        &self.start_line
    }

    pub fn get_method(&self) -> &RequestMethod {
        &self.method
    }

    pub fn get_target(&self) -> &String {
        &self.target
    }

    pub fn get_http_version(&self) -> &String {
        &self.http_version
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn get_params(&self) -> &HashMap<String, String> {
        &self.params
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
