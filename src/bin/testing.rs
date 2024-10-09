use http_ez;
use http_ez::server::*;

trait Test {}

#[derive(Default, Clone)]
struct MyService;

impl MyService {
    pub fn test_fun(&self) {
        println!("testing!!!");
    }
}

fn main() {
    let mut app = AppBuilder::new()
        // .register_service::<MyService>(ServiceScope::Singleton)
        //.map_route(Method::GET, "api/resource/:id", ResourceHandler::handle)
        .build();

    Server::run(&mut app, "localhost", 8080);
}
