# potato

Potato is a simple HTTP framework for rust.

## Example
```rust
use potato::{app::App, request::Method, router::Router};

#[tokio::main]
async fn main() {
    let mut router = Router::new();

    router.add(Method::GET, "/", get_potato);

    let mut app = App::new(router);

    app.serve("127.0.0.1:8080").await.unwrap();
}


pub fn get_potato(request: Request) -> Response {
    let mut response = Response::new();

    let id = match request.query().get("id") {
        Some(id) => id,
        None => {
            response
                .with_status(Status::BadRequest)
                .with_content("You must provide an ID".to_owned())
                .build();

            return response;
        }
    };

    let potato = Potato::get_by_id(id);

    response
        .with_header("Content-Type", "application/json")
        .with_content(potato.to_json())
        .build();

    response
}
```

