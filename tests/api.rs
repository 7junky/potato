use chrono::prelude::*;

use potato::app::App;
use potato::request::{Method, Request};
use potato::response::{Cookie, Response};
use potato::router::Router;
use potato::status::Status;

fn get(request: Request) -> Response {
    let mut response = Response::new();
    response
        .with_header("Content-Type", "text/html")
        .with_cookie(Cookie {
            key: "secure",
            value: "and http only",
            expires: None,
            secure: true,
            http_only: true,
        })
        .with_cookie(Cookie {
            key: "notsecure",
            value: "with expiry",
            expires: Some(chrono::Utc.ymd(2022, 12, 1).and_hms(12, 00, 00)),
            secure: false,
            http_only: false,
        })
        .with_content(format!(
            "You sent: {:?}, {} and {}",
            request.method(),
            request.target(),
            request.version()
        ));

    response
}

fn delete(request: Request) -> Response {
    let mut response = Response::new();

    let id = match request.query().get("id") {
        Some(id) => id,
        None => {
            response
                .with_status(Status::BadRequest)
                .with_content("You need to give an ID!".to_owned());
            return response;
        }
    };

    response.with_header("id", id);

    response
}

async fn init() -> App {
    let mut router = Router::new();

    router
        .add(Method::GET, "/potato", get)
        .add(Method::POST, "/potato", get)
        .add(Method::PATCH, "/potato", get)
        .add(Method::DELETE, "/potato", delete);

    App::new(router)
}

#[tokio::test]
async fn test_get() {
    let mut app = init().await;

    let mut request = Request::default();
    request.with_start_line(Method::GET, "/potato", "HTTP/1.1");

    let response = app.request(request).await.unwrap();

    assert_eq!(response.status(), &Status::OK);
    assert_eq!(response.to_string(), "HTTP/1.1 200 OK\r\n\
Content-Length: 35\r\n\
Content-Type: text/html\r\n\
Set-Cookie: secure=and http only; Secure; HttpOnly\r\n\
Set-Cookie: notsecure=with expiry; Expires=Thu, 01 Dec 2022 12:00:00 +0000\r\n\r\n\
You sent: GET, /potato and HTTP/1.1".to_owned());
}

#[tokio::test]
async fn test_post() {
    let mut app = init().await;

    let json = "\
{
    \"name\": \"bob\",
    \"age\": 22
}";

    let mut request = Request::default();
    request
        .with_start_line(Method::POST, "/potato", "HTTP/1.1")
        .with_content(json);

    let _response = app.request(request).await.unwrap();
}

#[tokio::test]
async fn test_delete() {
    let mut app = init().await;

    let mut request = Request::default();
    request.with_start_line(Method::DELETE, "/potato?id=1234", "HTTP/1.1");

    let response = app.request(request).await.unwrap();

    assert_eq!(response.headers().get("id").unwrap(), "1234");
}
