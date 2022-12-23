use crate::request::{Method, Request};
use crate::response::Response;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

type Handler = fn(Request) -> Response;

type RouteMap = HashMap<String, Handler>;
pub(crate) type Routes = Arc<RwLock<RouteMap>>;

pub struct Router {
    pub(crate) routes: Routes,
    before_routes: Vec<(String, Handler)>,
}

impl Router {
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

    pub(crate) async fn build(&mut self) {
        while let Some((key, handle)) = self.before_routes.pop() {
            self.routes.write().await.insert(key.to_owned(), handle);
        }
    }

    pub(crate) async fn get_routes(&self) -> RwLockReadGuard<'_, RouteMap> {
        self.routes.read().await
    }
}
