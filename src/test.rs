use tokio::io::{AsyncBufReadExt, BufReader};

use crate::app::App;
use crate::request::Method;
use crate::request::Request;
use crate::response::Response;
use crate::status::Status;

pub struct TestApp {
    app: App,
}

impl TestApp {
    pub fn serve(app: App) -> Self {
        Self { app }
    }

    fn fake_request(
        &self,
        method: Method,
        path: &str,
        content: &str,
    ) -> String {
        format!("{:?} {} HTTP/1.1\r\nHost: www.test.com\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n\r\n{:?}", method, path, content)
    }

    pub async fn request(
        &mut self,
        method: Method,
        path: &str,
        content: &str,
    ) -> Result<Response, Status> {
        let fake_request = self.fake_request(method, path, content);
        let reader = BufReader::new(fake_request.as_bytes());
        let request = Request::new(BufReader::lines(reader)).await;

        self.app.router.build().await;
        let routes = self.app.router.get_routes().await;
        let route_key = request.get_route_key();
        let handler = match routes.get(route_key) {
            Some(h) => h,
            None => Err(Status::NotFound)?,
        };

        Ok(handler(request))
    }
}
