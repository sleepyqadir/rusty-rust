use super::status_code::StatusCode;
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
}
