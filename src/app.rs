use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;

pub enum Status {
    OK,
}
impl Status {
    fn to_str<'a>(&self) -> &'a str {
        // match self
        "200 OK"
    }
}

pub struct Request;

pub struct Response {
    status: Status,
    content: String,
}

impl Response {
    pub fn with_status(&mut self, status: Status) {
        self.status = status
    }

    pub fn with_content(&mut self, content: String) {
        self.content = content;
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

    fn handle_connection<'a>(&'a self, mut stream: TcpStream) -> std::io::Result<()> {
        let buf_reader = BufReader::new(&mut stream);
        let http_header = buf_reader.lines().next().unwrap().unwrap();

        let handle = match self.routes.get(&http_header) {
            Some(handle) => handle.clone(),
            None => {
                // Send 404
                todo!();
            }
        };

        let mut response = handle(Request);
        stream.write_all(response.data().as_bytes());
        stream.flush();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RequestMethod::*;
    use super::{App, Request, Response, Status};

    fn get_handle(req: Request) -> Response {
        println!("This works!");
        Response {
            status: Status::OK,
            content: "Hello".to_owned(),
        }
    }

    #[test]
    fn it_works() {
        let mut app = App::new(("0.0.0.0", 8080));
        app.add(GET, "/", get_handle);

        app.serve();
    }
}
