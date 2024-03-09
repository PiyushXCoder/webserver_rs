use async_std::net::SocketAddr;

mod server;

#[async_std::main]
async fn main() {
    let server = server::Server::new("0.0.0.0:8000").await.unwrap();
    server.clone().add_route("/hello", hello);
    server.clone().add_route("/bolo", bolo);
    server.clone().add_route("/ip", ip);
    server.clone().run().await;
}

async fn hello(_: SocketAddr) -> String {
    "hello".to_string()
}

async fn bolo(_: SocketAddr) -> String {
    "bolo bhai?".to_string()
}

async fn ip(addr: SocketAddr) -> String {
    format!("{:?}", addr)
}
