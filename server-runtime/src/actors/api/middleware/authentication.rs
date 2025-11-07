use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request, State},
    http::{self, HeaderMap, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use tracing::debug;

use crate::actors::api::{errors::ApiError, state::AxumApiState, utils::get_client_ip};

pub async fn authenticated(
    state: State<AxumApiState>,
    ConnectInfo(socket_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    println!("{:#?}", headers);

    // println!(
    //     "Client IP: {:#?}",
    //     get_client_ip(state.behind_proxy, &headers, socket_addr)
    // );

    debug!("Extracting bearer token from request");
    let bearer_token = get_bearer_token(headers);

    if bearer_token.is_none() {
        return Err(ApiError::Unauthorized(
            "Bearer token was not provided".to_string(),
        ));
    }

    match state.db_client.authenticate(bearer_token.unwrap()).await {
        Ok(_) => {
            let response = next.run(request).await;
            Ok(response)
        }
        Err(_error) => Err(ApiError::Unauthorized(format!("Bearer token is invalid"))),
    }
}

fn get_bearer_token(headers: HeaderMap) -> Option<String> {
    match headers.get("authorization") {
        Some(token) => Some(token.to_str().ok()?.to_string().replace("Bearer", "")),
        None => None,
    }
}
