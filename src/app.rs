use crate::request::{Request, RequestMethod};
use crate::response::Response;
use crate::status::Status;

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;

type Handler = fn(Request) -> Response;

pub struct App<T>
where
    T: ToSocketAddrs,
{
    addr: T,
    routes: HashMap<String, Arc<Handler>>,
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

    pub fn add(
        &mut self,
        method: RequestMethod,
        route: &str,
        handle: Handler,
    ) -> Result<(), &str> {
        if !route.starts_with("/") {
            return Err("Route must start with /");
        }

        let method = method.to_str();
        let route = format!("{} {} HTTP/1.1", method, route);
        self.routes.insert(route, Arc::new(handle));

        Ok(())
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

    fn respond(
        stream: &mut TcpStream,
        response: &mut Response,
    ) -> std::io::Result<()> {
        stream.write_all(response.data().as_bytes())?;
        stream.flush()?;
        Ok(())
    }

    fn handle_connection<'a>(
        &'a self,
        mut stream: TcpStream,
    ) -> std::io::Result<()> {
        let buf_reader = BufReader::new(&mut stream);
        let request_lines = BufReader::lines(buf_reader);

        let req = Request::new(request_lines);

        let handle = match self.routes.get(req.get_start_line()) {
            Some(handle) => handle.clone(),
            None => {
                let mut res = Response::new();
                res.with_status(Status::NotFound)
                    .with_content("Not found".to_owned());
                return Self::respond(&mut stream, &mut res);
            }
        };

        let mut res = handle(req);
        Self::respond(&mut stream, &mut res)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::RequestMethod::*;
    use super::{App, Request, Response};

    use crate::response::Cookie;

    use chrono::prelude::*;

    fn get_handle(req: Request) -> Response {
        let mut res = Response::new();
        res.with_header("Content-Type", "text/html")
            .with_cookie(Cookie {
                key: "secure",
                value: "and http only",
                expires: None,
                secure: true,
                http_only: true,
            })
            .with_cookie(Cookie {
                key: "notsecure",
                value: "with expiry",
                expires: Some(chrono::Utc.ymd(2022, 12, 1).and_hms(12, 00, 00)),
                secure: false,
                http_only: false,
            })
            .with_content(format!(
                "You sent: {:?}, {} and {}",
                req.get_method(),
                req.get_target(),
                req.get_http_version()
            ));
        res
    }

    #[test]
    fn it_works() {
        let mut app = App::new(("0.0.0.0", 8080));
        app.add(GET, "/test", get_handle).unwrap();

        app.serve();
    }
}
