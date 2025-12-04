use axum::http::HeaderName;

// Return the header name we will use to store the request id
// for each call to the api
pub fn get_request_id_header_name() -> HeaderName {
    HeaderName::from_static("x-request-id")
}
