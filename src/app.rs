use crate::request::{Method, Request};
use crate::response::Response;
use crate::status::Status;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::{RwLock, RwLockReadGuard};

type Handler = fn(Request) -> Response;

type RouteMap = HashMap<String, Handler>;
pub type Routes = Arc<RwLock<RouteMap>>;

pub struct App {
    routes: Routes,
    before_routes: Vec<(String, Handler)>,
}

impl App {
    pub fn new() -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            before_routes: Vec::new(),
        }
    }

    pub fn add(
        &mut self,
        method: Method,
        route: &str,
        handle: Handler,
    ) -> &mut Self {
        assert!(route.starts_with("/"));

        let route_key = format!("{:?} {} HTTP/1.1", method, route);
        self.before_routes.push((route_key, handle));

        self
    }

    pub(crate) async fn build_routes(&mut self) {
        while let Some((key, handle)) = self.before_routes.pop() {
            self.routes.write().await.insert(key.to_owned(), handle);
        }
    }

    pub async fn serve<T: ToSocketAddrs>(
        &mut self,
        addr: T,
    ) -> std::io::Result<()> {
        self.build_routes().await;

        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            tokio::task::spawn(Self::handle_connection(
                socket,
                self.routes.clone(),
            ));
        }
    }

    pub async fn get_routes(&self) -> RwLockReadGuard<'_, RouteMap> {
        self.routes.read().await
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

        let routes = routes.read().await;
        let handle = match routes.get(req.get_route_key()) {
            Some(handle) => handle,
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
