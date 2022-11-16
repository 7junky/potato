use crate::request::Request;
use crate::response::Response;
use crate::router::Router;
use crate::router::Routes;
use crate::status::Status;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub struct App {
    pub(crate) router: Router,
}

impl App {
    pub fn new(router: Router) -> Self {
        Self { router }
    }

    pub async fn serve<T: ToSocketAddrs>(
        &mut self,
        addr: T,
    ) -> std::io::Result<()> {
        self.router.build().await;

        let listener = TcpListener::bind(addr).await?;

        loop {
            let (socket, _) = listener.accept().await?;
            tokio::task::spawn(Self::handle_connection(
                socket,
                self.router.routes.clone(),
            ));
        }
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
