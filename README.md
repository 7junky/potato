# potato

Potato is a simple HTTP framework for rust.

## Example
```rust
use potato::{App, Method, Router, Request, Response, Status};

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    router.add(Method::GET, "/", get_potato);

    let mut app = App::new(router);

    if let Err(e) = app.serve("0.0.0.0:8080").await {
        eprintln!("{}", e)
    }
}


pub fn get_potato(request: Request) -> Response {
    let mut response = Response::new();

    let id = match request.query().get("id") {
        Some(id) => id,
        None => {
            response
                .with_status(Status::BadRequest)
                .with_content("You must provide an ID".to_owned());

            return response;
        }
    };

    let potato = Potato::from_id(id);

    response
        .with_header("Content-Type", "application/json")
        .with_content(potato.to_json());

    response
}
```

