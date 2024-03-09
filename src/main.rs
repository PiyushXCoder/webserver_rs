mod server;

use async_std::{io, net::SocketAddr};
use server::ServerBuilder;

#[async_std::main]
async fn main() -> io::Result<()> {
    ServerBuilder::new("0.0.0.0:8000")
        .add_route("/", index)
        .add_route("/hello", hello)
        .add_route("/bolo", bolo)
        .add_route("/ip", ip)
        .build()
        .listen()
        .await
}

async fn index(_: SocketAddr) -> String {
    "\
<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <title>webserver-rs</title>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
  </head>
  <body>
    <h1>Welcome to webserver-rs</h1>
    <a href=\"https://github.com/PiyushXCoder/webserver_rs\">Check source code for mode</a>
  </body>
</html>"
        .to_string()
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
