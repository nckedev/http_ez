use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::{path::Path, status_code::StatusCode};
use crate::logger::Logger;
use crate::service::ServiceCollection;
use core::panic;
use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Server;
impl Server {
    ///Starts the listener
    pub fn run(app: &App, ip: &str, port: u32) -> () {
        println!("Listening @ {ip}:{port}");
        let listner = TcpListener::bind(format!("{ip}:{port}")).unwrap();

        for stream in listner.incoming() {
            let stream = stream.unwrap();
            Self::handle_connection(app, stream);
        }
    }

    fn handle_connection(app: &App, mut stream: TcpStream) -> () {
        let buf_reader = BufReader::new(stream);
        let request_raw: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let first: Vec<_> = request_raw
            .first()
            .expect("no method line")
            .split(' ')
            .collect();
        debug_assert!(first.len() == 3, "failed to parse method line");
        //create request object with method(0) path(1) version(2)
        let mut request = Request::new(first[0].into(), Path::new(first[1]), first[2]);

        // parse headers
        for h in request_raw.iter().skip(1) {
            if let Some((key, value)) = h.split_once(':') {
                request.add_header(key.trim_start(), value.trim_start());
            }
        }

        println!("logger");
        if let Some(logger) = app.service_collection.get::<Logger>() {
            println!("logger");
            logger.log_info("TESTING");
        }

        println!("{:#?}", request);
        //assert_eq!(app.service_collection.has_any(), true);

        //run request through middleware

        //run route handler
        //run response through middleware in reverse order

        //println!("{request_raw:#?}");
    }
}

pub struct AppBuilder {
    service_collection: ServiceCollection,
}

impl AppBuilder {
    pub fn new() -> Self {
        let mut builder = AppBuilder {
            service_collection: ServiceCollection::new(),
        };

        builder.register_service::<Logger>(ServiceScope::Singleton);
        builder
    }

    pub fn register_service<T: 'static + Default>(&mut self, scope: ServiceScope) -> &mut Self {
        println!("register service");
        match self.service_collection.add(T::default(), scope) {
            Ok(()) => self,
            Err(_) => panic!("could not create service"),
        }
    }

    pub fn map_route(&mut self) -> &mut Self {
        self
    }

    pub fn build(self) -> App {
        // service_collection (bara namnen på servicess) -> service_provider i Appcontext
        App::new(self.service_collection)
    }
}

#[derive(Default)]
pub enum Environment {
    #[default]
    Development,
    Stage,
    Production,
    Custom(String),
}

impl Environment {
    pub fn is_dev(&self) -> bool {
        match self {
            Self::Development => true,
            _ => false,
        }
    }
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "development" => Self::Development,
            "stage" => Self::Stage,
            "production" => Self::Production,
            x => Self::Custom(x.to_string()),
        }
    }
}

struct AppContext {}

pub struct App {
    service_collection: ServiceCollection,
    middleware: Option<Vec<Box<dyn Middleware>>>,
    environment: Environment,
    port: u32,
}

impl App {
    fn new(service_collection: ServiceCollection) -> Self {
        App {
            service_collection,
            middleware: None,
            environment: Environment::default(),
            port: 8080,
        }
    }
    fn add_middleware(&mut self, args: Box<dyn Middleware>) -> &mut Self {
        if let Some(mw) = &mut self.middleware {
            mw.push(args);
        };

        self
    }
}

struct Route {
    path: String,
    handler: Box<dyn FnOnce(&str) -> (StatusCode, Response)>,
}

struct RouteMap {
    routes: Vec<Route>,
}

impl RouteMap {
    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route)
    }
}
macro_rules! route {
    () => {};
}
trait Middleware {
    fn invoke(&self, req: Request) -> Response;
}

trait AddMiddleware {
    fn use_middleware(&mut self, mw: Box<dyn Middleware>) -> &mut Self;
}

#[derive(PartialEq, Eq)]
pub enum ServiceScope {
    ///always the same
    Singleton,
    //new for every http request
    Scoped,
    //new for every service request
    Transient,
}
