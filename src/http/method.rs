#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    PATCH,
    PUT,
    DETELE,
    HEAD,
    OPTIONS,
    CONNECT,
    TRACE,
    OTHER(String),
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::GET,
            "POST" => Method::POST,
            x => Method::OTHER(x.to_string()),
        }
    }
}
