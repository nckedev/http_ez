pub enum StatusCode {
    Success = 200,
    Created = 201,

    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    Conflict = 409,

    InternalserverError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl Into<u16> for StatusCode {
    fn into(self) -> u16 {
        self as u16
    }
}
