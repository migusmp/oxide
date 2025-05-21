use std::sync::Arc;

use router::{handle_connection, Router};
use tokio::net::TcpListener;

pub mod responses;
pub mod router;
pub mod server;

pub struct App {
    router: Router,
    addr: String,
}

impl App {
    pub async fn init(router: Router, addr: String) {
        let listener = TcpListener::bind(&addr).await.unwrap();

        let router = Arc::new(router);

        println!("Listening on http://127.0.0.1:3000");

        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let router = Arc::clone(&router);

            // Spawn a new task for each connection
            tokio::task::spawn(async move {
                handle_connection(stream, router).await;
            });
        }
    }
}
