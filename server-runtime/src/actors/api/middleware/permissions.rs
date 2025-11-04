use axum::{
    extract::Request,
    http,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};

pub async fn permissions_check(request: Request, next: Next) -> Response {
    println!("Checking Permissions in");

    // Do something with request

    let response = next.run(request).await;

    // Do something with response
    println!("Checking Permissions out");

    response
}
