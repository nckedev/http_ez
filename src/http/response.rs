use crate::http::status_code::StatusCode;
use std::collections::HashMap;

pub struct Response<'a> {
    headers: HashMap<String, String>,
    status_code: StatusCode,
    body: &'a str,
}

impl<'a> Response<'a> {
    fn ok(body: &'a str) -> Self {
        Response {
            headers: HashMap::new(),
            status_code: StatusCode::Success,
            body,
        }
    }
}

impl<'a> Into<Response<'a>> for &'a str {
    fn into(self) -> Response<'a> {
        Response {
            headers: HashMap::new(),
            status_code: StatusCode::Success,
            body: self,
        }
    }
}

impl<'a> Into<Response<'a>> for (StatusCode, &'a str) {
    fn into(self) -> Response<'a> {
        Response {
            headers: HashMap::new(),
            status_code: self.0,
            body: self.1,
        }
    }
}
