use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;

pub enum Status {
    OK,
    BadRequest,
    NotFound,
    Internal,
}

impl Status {
    fn to_str(&self) -> &str {
        match self {
            Status::OK => "200 OK",
            Status::BadRequest => "400 Bad Request",
            Status::NotFound => "404 Not Found",
            Status::Internal => "500 Internal Server Error",
        }
    }
}

pub struct Request {
    start_line: String,
    host: String,
    user_agent: String,
    accept: String,
}

impl Request {
    pub fn new(start_line: String, host: String, user_agent: String, accept: String) -> Self {
        Self {
            start_line,
            host,
            user_agent,
            accept,
        }
    }
}

pub struct Response {
    status: Status,
    content: String,
}

impl Response {
    pub fn with_status(&mut self, status: Status) -> &mut Self {
        self.status = status;
        self
    }

    pub fn with_content(&mut self, content: String) -> &mut Self {
        self.content = content;
        self
    }

    pub fn data(&mut self) -> String {
        format!(
            "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status.to_str(),
            self.content.len(),
            self.content
        )
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: Status::OK,
            content: String::from("OK"),
        }
    }
}

type Handler = fn(Request) -> Response;

pub struct App<T>
where
    T: ToSocketAddrs,
{
    addr: T,
    routes: HashMap<String, Arc<Handler>>,
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

impl<T> App<T>
where
    T: ToSocketAddrs,
{
    pub fn new(addr: T) -> Self {
        Self {
            addr,
            routes: HashMap::default(),
        }
    }

    pub fn add(&mut self, method: RequestMethod, route: &str, handle: Handler) {
        assert!(route.starts_with("/"));

        let method = method.to_str();
        let route = format!("{} {} HTTP/1.1", method, route);
        self.routes.insert(route, Arc::new(handle));
    }

    pub fn serve(&self) -> std::io::Result<()> {
        let stream = TcpListener::bind(&self.addr)?;

        for stream in stream.incoming() {
            let stream = match stream {
                Ok(stream) => stream,
                Err(e) => panic!("{}", e),
            };
            // Thread pool
            self.handle_connection(stream)?;
        }

        Ok(())
    }

    fn respond(stream: &mut TcpStream, response: &mut Response) -> std::io::Result<()> {
        stream.write_all(response.data().as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn handle_connection<'a>(&'a self, mut stream: TcpStream) -> std::io::Result<()> {
        let buf_reader = BufReader::new(&mut stream);
        let mut request_lines = BufReader::lines(buf_reader);

        let start_line = request_lines.next().unwrap().unwrap();
        let host = request_lines.next().unwrap().unwrap();
        let user_agent = request_lines.next().unwrap().unwrap();
        let accept = request_lines.next().unwrap().unwrap();

        let handle = match self.routes.get(&start_line) {
            Some(handle) => handle.clone(),
            None => {
                let mut res = Response::default();
                res.with_status(Status::NotFound)
                    .with_content("Not found".to_owned());
                return Self::respond(&mut stream, &mut res);
            }
        };

        let req = Request::new(start_line, host, user_agent, accept);

        let mut res = handle(req);
        Self::respond(&mut stream, &mut res)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RequestMethod::*;
    use super::{App, Request, Response};

    fn get_handle(req: Request) -> Response {
        let mut res = Response::default();
        res.with_content(format!(
            "You sent: {}, {}, {} and {}",
            req.start_line, req.host, req.user_agent, req.accept
        ));
        res
    }

    #[test]
    fn it_works() {
        let mut app = App::new(("0.0.0.0", 8080));
        app.add(GET, "/", get_handle);

        app.serve();
    }
}
