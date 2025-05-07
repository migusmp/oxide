use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;

use oxide::responses::HttpResponse;
use oxide::router::{handle_connection, Router};

pub fn hello_world(mut stream: TcpStream) {
    stream
        .write_all(HttpResponse::ok_plaintext("Hello world!!").as_bytes())
        .unwrap();
}

pub fn index(mut stream: TcpStream) {
    stream
        .write_all(HttpResponse::ok_plaintext("Pagina principal").as_bytes())
        .unwrap();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr)?;

    let router = Arc::new(Router::new().get("/", index).get("/hello", hello_world));

    println!("Listening on http://127.0.0.1:3000");

    loop {
        let (stream, _) = listener.accept()?;
        let router = Arc::clone(&router);

        // Spawn a new task for each connection
        tokio::task::spawn(async move {
            handle_connection(stream, router).await;
        });
    }
}
