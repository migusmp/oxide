use std::{collections::HashMap, pin::Pin, sync::Arc};

use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::responses::HttpResponse;

type BoxedFuture = Pin<Box<dyn Future<Output = ()> + Send>>;
type Handler = Arc<dyn Fn(TcpStream) -> BoxedFuture + Send + Sync>;

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

pub struct Route {
    pub method: Method,
    pub handler: Handler,
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

    // fn add_route<F>(mut self, method: Method, path: &str, handler: F) -> Self
    // where
    //     F: Fn(TcpStream) + Send + Sync + 'static,
    // {
    //     let handler = Arc::new(handler);
    //     self.routes.insert((path.to_string(), method), handler);
    //     self
    // }

    pub fn route(mut self, path: &str, route: Route) -> Self {
        self.routes
            .insert((path.to_string(), route.method), route.handler);
        self
    }

    pub async fn handle(&self, req_path: &str, method: Method, mut stream: TcpStream) {
        if let Some(handler) = self.routes.get(&(req_path.to_string(), method)) {
            handler(stream).await;
        } else {
            let _ = async move {
                stream
                    .write_all(HttpResponse::not_found().as_bytes())
                    .await
                    .unwrap();
            };
        }
    }
}

pub async fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    use std::str::from_utf8;
    use tokio::io::AsyncReadExt;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await.unwrap();

    let request = from_utf8(&buffer[..n]).unwrap_or("");
    println!("Request: {}", request);

    let mut split_request = request.split_whitespace();
    let method = split_request.next().unwrap_or("");
    let path = split_request.next().unwrap_or("/");

    match Method::from_str(method) {
        Some(m) => router.handle(path, m, stream).await,
        None => {
            let _ = stream
                .write_all(HttpResponse::method_not_allowed().as_bytes())
                .await;
        }
    }
}

pub fn get<F, Fut>(handler: F) -> Route
where
    F: Fn(TcpStream) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let wrapped = Arc::new(move |stream: TcpStream| Box::pin(handler(stream)) as BoxedFuture);
    Route {
        method: Method::GET,
        handler: wrapped,
    }
}

pub fn post<F, Fut>(handler: F) -> Route
where
    F: Fn(TcpStream) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let wrapped = Arc::new(move |stream: TcpStream| Box::pin(handler(stream)) as BoxedFuture);
    Route {
        method: Method::POST,
        handler: wrapped,
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
