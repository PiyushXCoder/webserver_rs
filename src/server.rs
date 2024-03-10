use std::{collections::HashMap, future::Future, pin::Pin};

use async_std::{
    io::{ReadExt, WriteExt},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    println,
    sync::Arc,
};

type ResponderFn = Box<dyn Fn(SocketAddr) -> Pin<Box<dyn Future<Output = String> + 'static>>>;
type Routes = HashMap<Box<str>, ResponderFn>;

pub(crate) struct ServerBuilder<A>
where
    A: ToSocketAddrs + ToString + 'static,
{
    routers: Routes,
    addr: A,
}

impl<A> ServerBuilder<A>
where
    A: ToSocketAddrs + ToString + 'static,
{
    pub(crate) fn new(addr: A) -> Self {
        Self {
            routers: HashMap::new(),
            addr,
        }
    }

    pub(crate) fn add_route<R: Future<Output = String> + 'static>(
        mut self,
        route: &str,
        callback: fn(addr: SocketAddr) -> R,
    ) -> Self {
        self.routers
            .insert(route.into(), Box::new(move |a| Box::pin(callback(a))));
        self
    }

    pub(crate) fn build(self) -> Arc<Server<A>> {
        Server::new(self.addr, self.routers)
    }
}

pub(crate) struct Server<A>
where
    A: ToSocketAddrs + ToString + 'static,
{
    routes: Routes,
    addr: A,
}

impl<A> Server<A>
where
    A: ToSocketAddrs + ToString + 'static,
{
    pub(crate) fn new(addr: A, routes: Routes) -> Arc<Self> {
        Arc::new(Self { routes, addr })
    }

    pub(crate) async fn listen(self: Arc<Self>) -> std::io::Result<()> {
        let tcp_listener = TcpListener::bind(&self.addr).await?;
        loop {
            let (stream, addr) = tcp_listener.accept().await.unwrap();

            async_std::task::spawn_local(Arc::clone(&self).responder(stream, addr));
        }
    }

    pub(crate) async fn responder(self: Arc<Self>, mut stream: TcpStream, addr: SocketAddr) {
        let mut buf = [0; 1024];
        stream.read(&mut buf).await.unwrap();
        let data = std::str::from_utf8(&buf).unwrap();
        let head_data = data.split("\r\n\r\n").next().unwrap();
        let head_lines = head_data.split("\r\n").collect::<Vec<&str>>();

        let parts = head_lines[0].split(" ").collect::<Vec<&str>>();

        let method = parts[0];
        let uri = parts[1];

        println!("{}, {}", method, uri).await;
        let (method, msg) = match self.routes.get(uri) {
            Some(f) => ("200 OK", f(addr).await),
            None => ("404 NOT FOUND", "NOT FOUND".to_string()),
        };

        stream
            .write_all(
                format!(
                    "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                    method,
                    msg.len(),
                    msg
                )
                .as_bytes(),
            )
            .await
            .unwrap();
        stream.flush().await.unwrap();
    }
}
