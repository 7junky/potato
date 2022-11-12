use crate::request::{Method, Request};
use crate::response::Response;
use crate::status::Status;

use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

type Handler = fn(Request) -> Response;

pub type Routes = HashMap<String, Handler>;

pub struct App {
    routes: Routes,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        method: Method,
        route: &str,
        handle: Handler,
    ) -> Result<(), &str> {
        if !route.starts_with("/") {
            return Err("Route must start with /");
        }

        let method = method.to_str();
        let route = format!("{} {} HTTP/1.1", method, route);
        self.routes.insert(route, handle);

        Ok(())
    }

    pub async fn serve<T: ToSocketAddrs>(
        &self,
        addr: T,
    ) -> std::io::Result<()> {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            tokio::task::spawn(Self::handle_connection(
                socket,
                self.routes.clone(),
            ));
        }
    }

    pub fn get_routes(&self) -> &Routes {
        &self.routes
    }

    async fn respond(
        stream: &mut TcpStream,
        response: &mut Response,
    ) -> std::io::Result<()> {
        stream.write_all(response.raw().as_bytes()).await?;
        stream.flush().await?;
        Ok(())
    }

    async fn handle_connection(
        mut stream: TcpStream,
        routes: Routes,
    ) -> tokio::io::Result<()> {
        let buf_reader = BufReader::new(&mut stream);
        let request_lines = BufReader::lines(buf_reader);

        let req = Request::new(request_lines).await;

        let handle = match routes.get(req.get_route_key()) {
            Some(handle) => handle.clone(),
            None => {
                let mut res = Response::new();
                res.with_status(Status::NotFound)
                    .with_content("Not found".to_owned())
                    .build();
                return Self::respond(&mut stream, &mut res).await;
            }
        };

        let mut res = handle(req);
        Self::respond(&mut stream, &mut res).await?;

        Ok(())
    }
}
