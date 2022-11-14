use chrono::prelude::*;

use potato::app::App;
use potato::request::{Method, Request};
use potato::response::{Cookie, Response};
use potato::status::Status;
use potato::test::TestApp;

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
            request.http_version()
        ))
        .build();

    response
}

fn delete(request: Request) -> Response {
    let mut response = Response::new();

    let id = match request.query().get("id") {
        Some(id) => id,
        None => {
            response
                .with_status(Status::BadRequest)
                .with_content("You need to give an ID!".to_owned())
                .build();
            return response;
        }
    };

    response.with_header("id", id).build();

    response
}

async fn init() -> TestApp {
    let mut app = App::new();

    app.add(Method::GET, "/potato", get).await.unwrap();
    app.add(Method::POST, "/potato", get).await.unwrap();
    app.add(Method::PATCH, "/potato", get).await.unwrap();
    app.add(Method::DELETE, "/potato", delete).await.unwrap();

    TestApp::serve(app)
}

#[tokio::test]
async fn test_get() {
    let app = init().await;

    let response = app.request(Method::GET, "/potato", "hello!").await.unwrap();

    assert_eq!(response.status(), &Status::OK);
    assert_eq!(response.raw(), &"HTTP/1.1 200 OK\r\n\
Content-Length: 35\r\n\
Content-Type: text/html\r\n\
Set-Cookie: secure=and http only; Secure; HttpOnly\r\n\
Set-Cookie: notsecure=with expiry; Expires=Thu, 01 Dec 2022 12:00:00 +0000\r\n\r\n\
You sent: GET, /potato and HTTP/1.1".to_owned());
}

#[tokio::test]
async fn test_delete() {
    let app = init().await;

    let response = app
        .request(Method::DELETE, "/potato?id=1234", "")
        .await
        .unwrap();

    assert_eq!(response.headers().get("id").unwrap(), "1234");
}
