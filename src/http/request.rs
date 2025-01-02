use crate::http::method::Method;
use std::collections::HashMap;
use uuid::{NoContext, Timestamp, Uuid};

use super::path::Path;

#[derive(Debug)]
pub struct Request<'a> {
    id: Uuid,
    method: Method,
    path: Path<'a>,
    protocol_version: &'a str,
    headers: HashMap<String, String>,
    body: String,
}

impl<'a> Request<'a> {
    pub fn new(method: Method, path: Path<'a>, protocol_version: &'a str) -> Self {
        Request {
            id: Uuid::new_v7(Timestamp::now(NoContext)),
            method,
            path,
            protocol_version,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            id: Uuid::nil()
            method: todo!(),
            path: todo!(),
            protocol_version: todo!(),
            headers: todo!(),
            body: todo!(),
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_owned(), value.to_owned());
    }
}
