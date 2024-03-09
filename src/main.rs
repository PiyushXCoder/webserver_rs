use async_std::net::SocketAddr;

mod server;

#[async_std::main]
async fn main() {
    let server = server::Server::new("0.0.0.0:8000").await.unwrap();
    server.clone().add_route("/hello", hello);
    server.clone().add_route("/bolo", bolo);
    server.clone().add_route("/ip", ip);
    server.clone().add_route("/", index);
    server.clone().run().await;
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
