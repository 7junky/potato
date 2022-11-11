use std::io::{BufRead, BufReader};
use std::net::ToSocketAddrs;

use crate::app::App;
use crate::request::Method;
use crate::request::Request;
use crate::response::Response;
use crate::status::Status;

pub struct TestApp<T>
where
    T: ToSocketAddrs,
{
    app: App<T>,
}

impl<T> TestApp<T>
where
    T: ToSocketAddrs,
{
    pub fn serve(app: App<T>) -> Self {
        Self { app }
    }

    fn construct_route_key(method: Method, path: &str) -> String {
        format!("{:?} {} HTTP/1.1", method, path)
    }

    fn fake_request(&self, path: &str, content: &str) -> String {
        format!("GET {} HTTP/2\r\nHost: www.test.com\r\nUser-Agent: curl/7.54.0\r\nAccept: */*\r\n\r\n{:?}", path, content)
    }

    pub fn request(
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
        let request = Request::new(BufReader::lines(reader));

        Ok(handler(request))
    }
}
