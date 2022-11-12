use std::collections::HashMap;
use std::io::{self, BufReader, Read};
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    start_line: String,
    method: Method,
    target: String,
    http_version: String,
    headers: HashMap<String, String>,
    query: HashMap<String, String>,
    route_key: Option<String>,
}

impl Request {
    pub fn new(mut lines: io::Lines<BufReader<impl Read>>) -> Self {
        let start_line = lines.next().unwrap().unwrap();
        let (method, target, http_version) =
            Self::parse_start_line(&start_line);
        let method = Method::from_str(&method).unwrap();
        let headers = Self::parse_headers(lines);
        let (path, query) = Self::parse_params(&target);
        let route_key = Self::generate_route_key(&method, path, &http_version);

        Self {
            start_line,
            method,
            target,
            http_version,
            headers,
            query,
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

    fn parse_headers(
        lines: io::Lines<BufReader<impl Read>>,
    ) -> HashMap<String, String> {
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
        method: &Method,
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

    pub fn start_line(&self) -> &String {
        &self.start_line
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn target(&self) -> &String {
        &self.target
    }

    pub fn http_version(&self) -> &String {
        &self.http_version
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn query(&self) -> &HashMap<String, String> {
        &self.query
    }
}

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
}

impl Method {
    pub fn to_str(&self) -> &str {
        match self {
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PATCH => "PATCH",
            Method::DELETE => "DELETE",
        }
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{BufRead, BufReader};

    use super::{Method, Request};

    #[test]
    fn it_works() {
        let raw_request = r#"GET /search?q=test HTTP/2
Host: www.bing.com
User-Agent: curl/7.54.0
Accept: */*
"#;

        let reader = BufReader::new(raw_request.as_bytes());
        let lines = BufReader::lines(reader);

        let request = Request::new(lines);

        assert_eq!(request.start_line, "GET /search?q=test HTTP/2".to_owned());
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.target, "/search?q=test".to_owned());
        assert_eq!(request.http_version, "HTTP/2".to_owned());

        assert_eq!(request.headers.len(), 3);
        assert_eq!(
            request.headers.get("User-Agent"),
            Some(&"curl/7.54.0".to_owned())
        );
        assert_eq!(request.headers.get("Accept"), Some(&"*/*".to_owned()));
        assert_eq!(
            request.headers.get("Host"),
            Some(&"www.bing.com".to_owned())
        );

        assert_eq!(request.query.get("q"), Some(&"test".to_owned()));

        assert_eq!(request.route_key, Some("GET /search HTTP/2".to_owned()));
    }
}
