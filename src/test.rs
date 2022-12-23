use crate::app::App;
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

    pub async fn request(
        &mut self,
        request: Request,
    ) -> Result<Response, Status> {
        self.app.router.build().await;
        let routes = self.app.router.get_routes().await;
        let route_key = request.route_key();
        let handler = match routes.get(route_key) {
            Some(h) => h,
            None => Err(Status::NotFound)?,
        };

        Ok(handler(request))
    }
}
