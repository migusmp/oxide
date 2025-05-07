use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
    sync::Arc,
};

use crate::responses::HttpResponse;

type Handler = Arc<dyn Fn(TcpStream) + Send + Sync>;

#[derive(Eq, Hash, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

impl Method {
    pub fn from_str(m: &str) -> Option<Self> {
        match m {
            "GET" => Some(Method::GET),
            "POST" => Some(Method::POST),
            "PUT" => Some(Method::PUT),
            "DELETE" => Some(Method::DELETE),
            _ => None,
        }
    }
}

pub struct Router {
    routes: HashMap<(String, Method), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    fn add_route<F>(mut self, method: Method, path: &str, handler: F) -> Self
    where
        F: Fn(TcpStream) + Send + Sync + 'static,
    {
        let handler = Arc::new(handler);
        self.routes.insert((path.to_string(), method), handler);
        self
    }

    pub fn get<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(TcpStream) + Send + Sync + 'static,
    {
        self.add_route(Method::GET, path, handler)
    }

    pub fn post<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(TcpStream) + Send + Sync + 'static,
    {
        self.add_route(Method::POST, path, handler)
    }

    pub fn put<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(TcpStream) + Send + Sync + 'static,
    {
        self.add_route(Method::PUT, path, handler)
    }

    pub fn delete<F>(self, path: &str, handler: F) -> Self
    where
        F: Fn(TcpStream) + Send + Sync + 'static,
    {
        self.add_route(Method::DELETE, path, handler)
    }

    pub fn handle(&self, req_path: &str, method: Method, mut stream: TcpStream) {
        if let Some(handler) = self.routes.get(&(req_path.to_string(), method)) {
            handler(stream);
        } else {
            stream
                .write_all(HttpResponse::not_found().as_bytes())
                .unwrap();
        }
    }
}

pub async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).unwrap();

    let request = from_utf8(&buffer[..n]).unwrap();
    println!("Request: {}", request);

    let mut split_request = request.split_whitespace();
    let method = split_request.next().unwrap(); // GET, POST, etc.
    let path = split_request.next().unwrap();

    match Method::from_str(method) {
        Some(m) => router.handle(path, m, stream),
        None => stream
            .write_all(HttpResponse::method_not_allowed().as_bytes())
            .unwrap(),
    }
}

// use std::{collections::HashMap, sync::Arc};
//
// use futures::future::BoxFuture;
// use hyper::{body::Incoming, Method, Request, Response};
//
// type Handler = Arc<
//     dyn Fn(Request<Incoming>) -> BoxFuture<'static, Result<Response<Incoming>, hyper::Error>>
//         + Send
//         + Sync,
// >;
//
// pub struct Router {
//     routes: HashMap<(Method, String), Handler>,
//     layers: Vec<Arc<dyn Fn(Handler) -> Handler + Send + Sync>>,
// }
//
// impl Router {
//     pub fn new() -> Self {
//         Self {
//             routes: HashMap::new(),
//             layers: Vec::new(),
//         }
//     }
//
//     pub fn route<F, Fut>(mut self, path: &str, method: Method, handler: F) -> Self
//     where
//         F: Fn(Request<Incoming>) -> Fut + Send + Sync + 'static,
//         Fut:
//             std::future::Future<Output = Result<Response<Incoming>, hyper::Error>> + Send + 'static,
//     {
//         let handler =
//             Arc::new(move |req: Request<Incoming>| -> BoxFuture<_> { Box::pin(handler(req)) })
//                 as Handler;
//
//         let mut final_handler = handler;
//
//         for layer in self.layers.iter().rev() {
//             final_handler = layer(final_handler.clone());
//         }
//
//         self.routes
//             .insert((method, path.to_string()), final_handler);
//
//         self
//     }
// }
