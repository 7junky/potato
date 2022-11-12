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

    fn construct_route_key(method: Method, path: &str) -> String {
        format!("{:?} {} HTTP/1.1", method, path)
    }

    fn fake_request(&self, path: &str, content: &str) -> String {
        format!("GET {} HTTP/2\r\nHost: www.test.com\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n\r\n{:?}", path, content)
    }

    pub async fn request(
        &self,
        method: Method,
        path: &str,
        content: &str,
    ) -> Result<Response, Status> {
        let route_key = Self::construct_route_key(method, path);
        let handler = match self.app.get_routes().get(&route_key) {
            Some(h) => h,
            None => return Err(Status::NotFound),
        };

        let fake_request = self.fake_request(path, content);
        let reader = BufReader::new(fake_request.as_bytes());
        let request = Request::new(BufReader::lines(reader)).await;

        Ok(handler(request))
    }
}
