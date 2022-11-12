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
        ));

    response
}

fn init() -> TestApp<&'static str> {
    let mut app = App::new("0.0.0.0:7357");

    app.add(Method::GET, "/potato", get).unwrap();
    app.add(Method::POST, "/potato", get).unwrap();
    app.add(Method::PATCH, "/potato", get).unwrap();
    app.add(Method::DELETE, "/potato", get).unwrap();

    TestApp::serve(app)
}

#[test]
fn test_get() {
    let app = init();

    let response = app.request(Method::GET, "/potato", "hello!").unwrap();

    assert_eq!(response.status(), &Status::OK)
}
