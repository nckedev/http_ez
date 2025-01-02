use http_ez;
use http_ez::http::method::Method;
use http_ez::http::response::Response;
use http_ez::http::status_code::StatusCode;
use http_ez::server::*;

#[derive(Default, Clone)]
struct MyService;

impl MyService {
    pub fn test_fun(&self) {
        println!("testing!!!");
    }
}

struct MyEndpoint {}

impl Endpoint for MyEndpoint {
    type Param;

    type Body = String;

    fn handle<'a>(&self, params: Self::Param) -> impl Into<Response<'a>> {
        todo!()
    }
}

fn main() {
    let mut app = AppBuilder::new()
        .register_service::<MyService>(ServiceScope::Singleton)
        //.map_route(Method::GET, "api/resource/:id", ResourceHandler::handle)
        .build();

    app.set_env(Environment::Development);
    app.regiter_route(Method::GET, "/test/:id", MyEndpoint);
    Server::run(&mut app, "localhost", 8080);
}
