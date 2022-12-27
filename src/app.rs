use crate::request::Request;
use crate::response::Response;
use crate::router::Router;
use crate::router::Routes;
use crate::Status;

use tokio::io::AsyncWriteExt;
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
        stream.write_all(response.to_string().as_bytes()).await?;
        stream.flush().await?;
        Ok(())
    }

    async fn handle_connection(
        mut stream: TcpStream,
        routes: Routes,
    ) -> tokio::io::Result<()> {
        let mut res = Response::new();

        let req = match Request::from_connection(&mut stream).await {
            Ok(r) => r,
            Err(e) => {
                dbg!(e);
                res.with_status(Status::BadRequest)
                    .with_content("Bad request".to_owned());
                return Self::respond(&mut stream, &mut res).await;
            }
        };

        let routes = routes.read().await;
        let handle = match routes.get(req.route_key()) {
            Some(handle) => handle,
            None => {
                res.with_status(Status::NotFound)
                    .with_content("Not found".to_owned());
                return Self::respond(&mut stream, &mut res).await;
            }
        };

        let mut res = handle(req);
        Self::respond(&mut stream, &mut res).await?;

        Ok(())
    }

    pub async fn request(
        &mut self,
        request: Request,
    ) -> Result<Response, Status> {
        self.router.build().await;
        let routes = self.router.get_routes().await;
        let route_key = request.route_key();
        let handler = match routes.get(route_key) {
            Some(h) => h,
            None => Err(Status::NotFound)?,
        };

        Ok(handler(request))
    }
}
