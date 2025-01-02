use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::{path::Path, status_code::StatusCode};
use crate::logger::{Logger, Severity};
use crate::service::{ServiceCollection, ServiceProvider};
use core::panic;
use std::collections::HashMap;
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
            Self::handle_connection(&app, stream);
        }
    }

    fn handle_connection(app: &App, mut stream: TcpStream) -> () {
        let buf_reader = BufReader::new(stream);
        let request_raw: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let first_line: Vec<_> = request_raw
            .first()
            .expect("no method line")
            .split(' ')
            .collect();
        debug_assert!(first_line.len() == 3, "failed to parse method line");

        let [method, path, version] = first_line[..] else {
            panic!("Failed to parse method line")
        };

        let mut request = Request::new(method.into(), Path::new(path), version);

        // parse headers
        for h in request_raw.iter().skip(1) {
            if let Some((key, value)) = h.split_once(':') {
                request.add_header(key.trim_start(), value.trim_start());
            }
        }

        let mut context = app.get_context(request);

        let logger = context.service_provider.get::<Logger>();
        if app.environment == Environment::Development {
            logger.borrow_mut().set_log_level(Severity::Trace)
        }
        logger.borrow().log_trace("context created");

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
        match self.service_collection.add::<T>(scope) {
            Ok(()) => self,
            Err(_) => panic!("could not create service"),
        }
    }

    pub fn build(self) -> App {
        // service_collection (bara namnen på servicess) -> service_provider i Appcontext
        App::new(self.service_collection)
    }
}

#[derive(Default, PartialEq, Eq)]
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

pub(crate) struct AppContext<'a> {
    environment: Environment,
    service_provider: ServiceProvider,
    request: Request<'a>,
}

pub struct App {
    service_collection: ServiceCollection,
    middleware: Option<Vec<Box<dyn Middleware>>>,
    environment: Environment,
    routes: Vec<Route>,
    port: u32,
}

impl App {
    fn new(service_collection: ServiceCollection) -> Self {
        App {
            service_collection,
            middleware: None,
            environment: Environment::default(),
            port: 8080,
            routes: Vec::new(),
        }
    }

    fn add_middleware(&mut self, args: Box<dyn Middleware>) -> &Self {
        if let Some(mw) = &mut self.middleware {
            mw.push(args);
        };

        self
    }

    pub fn regiter_route_closure(
        &mut self,
        method: Method,
        route: &str,
        handler: impl FnOnce(&str) -> Response,
    ) -> &Self {
        // TODO: how to match number of parameters to the handler?
        let a = handler();
        self
    }

    pub fn regiter_route<T: Endpoint<Body = T>>(
        &mut self,
        method: Method,
        route: &str,
        endpoint: T,
    ) -> &Self {
        let route = Self::parse_route(route, endpoint);

        self.routes.push(route);
        self
    }

    fn parse_route<T: Endpoint<Body = T>>(route: &str, handler: T) -> Route<T> {
        Route {
            path: route.to_string(),
            handler: Box::new(handler),
        }
    }

    pub fn set_env(&mut self, env: Environment) {
        self.environment = env;
    }

    pub(crate) fn get_context<'a>(&'a self, request: Request<'a>) -> AppContext {
        AppContext {
            service_provider: self.service_collection.clone().into(),
            environment: Environment::Development,
            request,
        }
    }
}

pub trait Endpoint {
    type Body;
    fn handle(&self, params: HashMap<String, String>, body: Self::Body) -> impl Into<Response>;
}

struct Route<T: Endpoint<Body = T>> {
    path: String,
    handler: Box<T>,
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
