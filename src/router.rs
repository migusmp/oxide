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
}
