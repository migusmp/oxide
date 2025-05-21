use oxide::responses::HttpResponse;
use oxide::router::{get, Router};
use oxide::App;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn hello_world(mut stream: TcpStream) {
    stream
        .write_all(HttpResponse::ok_plaintext("Hello world!!").as_bytes())
        .await
        .unwrap();
}

pub async fn index(mut stream: TcpStream) {
    stream
        .write_all(HttpResponse::ok_plaintext("Pagina principal").as_bytes())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(index))
        .route("/hello", get(hello_world));

    App::init(router, "127.0.0.1:3000".to_string()).await;
}
