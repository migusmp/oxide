use std::{collections::HashMap, sync::Arc};

use futures::future::BoxFuture;
use hyper::{
    body::{Body, Incoming},
    Method, Request, Response,
};

type Handler = Arc<
    dyn Fn(Request<Incoming>) -> BoxFuture<'static, Result<Response<Incoming>, hyper::Error>>
        + Send
        + Sync,
>;

pub struct Router {
    routes: HashMap<(Method, String), Handler>,
    layers: Vec<Arc<dyn Fn(Handler) -> Handler + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            layers: Vec::new(),
        }
    }

    pub fn route<F, Fut>(mut self, path: &str, method: Method, handler: F) -> Self
    where
        F: Fn(Request<Incoming>) -> Fut + Send + Sync + 'static,
        Fut:
            std::future::Future<Output = Result<Response<Incoming>, hyper::Error>> + Send + 'static,
    {
        let handler =
            Arc::new(move |req: Request<Incoming>| -> BoxFuture<_> { Box::pin(handler(req)) })
                as Handler;

        let mut final_handler = handler;

        for layer in self.layers.iter().rev() {
            final_handler = layer(final_handler.clone());
        }

        self.routes
            .insert((method, path.to_string()), final_handler);

        self
    }
}
