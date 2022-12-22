use std::collections::HashMap;
use std::str::FromStr;
use tokio::io::{self, AsyncBufReadExt, AsyncRead, AsyncReadExt, BufReader};

#[derive(Debug)]
pub enum ParseError {
    NoStartLine,
    NoMethod,
    NoTarget,
    NoVersion,
    InvalidMethod,
    InvalidContentLength,
    UnexpectedEof,
    ReadError,
}

#[derive(Debug)]
struct StartLine {
    line: String,
    method: Method,
    target: String,
    version: String,
}

#[derive(Debug)]
struct PathAndQuery {
    path: String,
    query: HashMap<String, String>,
}

impl PathAndQuery {
    pub fn from_target(target: &str) -> Self {
        let mut query = HashMap::new();

        let (path, raw_query) = match target.split_once("?") {
            Some(params) => params,
            None => (target, ""),
        };

        for q in raw_query.rsplit("&") {
            let (key, value) = match q.split_once("=") {
                Some(kv) => kv,
                None => continue,
            };

            query.insert(key.into(), value.into());
        }

        Self {
            path: path.to_owned(),
            query,
        }
    }
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

#[derive(Debug)]
pub struct Request {
    start_line: StartLine,
    path_and_query: PathAndQuery,
    headers: HashMap<String, String>,
    route_key: Option<String>,
    content: Option<String>,
}

impl Request {
    pub async fn from_connection<R>(r: &mut R) -> Result<Self, ParseError>
    where
        R: AsyncRead + Unpin,
    {
        let mut lines = BufReader::new(r).lines();

        // The start line holds the method, path, params, and http_version:
        let start_line = match lines.next_line().await.unwrap() {
            Some(l) => l,
            None => Err(ParseError::NoStartLine)?,
        };

        let start_line = StartLine::from_request(&start_line)?;
        dbg!(&start_line);

        // Parse headers into a map:
        let headers = Self::parse_headers(&mut lines).await;

        // If a Content-Length header has been sent, read the content:
        let mut data = String::new();
        if let Some(length) = headers.get("Content-Length") {
            let length: usize = length
                .parse()
                .map_err(|_| ParseError::InvalidContentLength)?;
            let mut buf: Vec<u8> = vec![0; length];

            let mut reader = lines.into_inner();
            reader
                .read_exact(&mut buf)
                .await
                .map_err(|_| ParseError::ReadError)?;

            data = std::str::from_utf8(&buf)
                .map(|s| s.to_owned())
                .map_err(|_| ParseError::ReadError)?
        };

        let content = match data.len() {
            0 => None,
            _ => Some(data),
        };

        let path_and_query = PathAndQuery::from_target(&start_line.target);

        // Construct a key that can be used to locate the handler in Router:
        let route_key = Self::construct_route_key(
            &start_line.method,
            &path_and_query.path,
            &start_line.version,
        );

        Ok(Self {
            start_line,
            path_and_query,
            headers,
            route_key,
            content,
        })
    }

    async fn parse_headers(
        lines: &mut io::Lines<BufReader<impl AsyncRead + Unpin>>,
    ) -> HashMap<String, String> {
        let mut header_map: HashMap<String, String> = HashMap::new();

        while let Some(line) = lines.next_line().await.unwrap() {
            if line == "" {
                break;
            }

            let (key, value) = match line.split_once(": ") {
                Some((k, v)) => (k, v),
                None => continue,
            };

            header_map.insert(key.into(), value.into());
        }

        header_map
    }

    fn construct_route_key(
        method: &Method,
        path: &str,
        version: &String,
    ) -> Option<String> {
        if path == "" {
            return None;
        };

        Some(format!("{:?} {} {}", method, path, version))
    }

    pub fn get_route_key(&self) -> &String {
        match &self.route_key {
            Some(route_key) => route_key,
            None => &self.start_line.line,
        }
    }

    pub fn start_line(&self) -> &String {
        &self.start_line.line
    }

    pub fn method(&self) -> &Method {
        &self.start_line.method
    }

    pub fn target(&self) -> &String {
        &self.start_line.target
    }

    pub fn version(&self) -> &String {
        &self.start_line.version
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn query(&self) -> &HashMap<String, String> {
        &self.path_and_query.query
    }

    pub fn content(&self) -> &Option<String> {
        &self.content
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
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PATCH" => Ok(Method::PATCH),
            "DELETE" => Ok(Method::DELETE),
            _ => Err(ParseError::InvalidMethod),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Method, Request};

    #[tokio::test]
    async fn it_works() {
        let raw_request = "GET /search?q=test HTTP/2\r\nHost: www.bing.com\r\nContent-Length: 5\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n\r\nHello";

        let request = Request::from_connection(&mut raw_request.as_bytes())
            .await
            .unwrap();

        assert_eq!(
            request.start_line.line,
            "GET /search?q=test HTTP/2".to_owned()
        );
        assert_eq!(request.start_line.method, Method::GET);
        assert_eq!(request.start_line.target, "/search?q=test".to_owned());
        assert_eq!(request.start_line.version, "HTTP/2".to_owned());

        assert_eq!(request.headers.len(), 4);
        assert_eq!(
            request.headers.get("User-Agent"),
            Some(&"curl/7.54.0".to_owned())
        );
        assert_eq!(request.headers.get("Accept"), Some(&"*/*".to_owned()));
        assert_eq!(
            request.headers.get("Host"),
            Some(&"www.bing.com".to_owned())
        );

        assert_eq!(
            request.path_and_query.query.get("q"),
            Some(&"test".to_owned())
        );

        assert_eq!(request.route_key, Some("GET /search HTTP/2".to_owned()));

        assert!(request.content.is_some());
        assert_eq!(request.content.unwrap(), "Hello");
    }
}
