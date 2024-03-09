use std::{collections::HashMap, future::Future, pin::Pin};

use async_std::{
    io::{ReadExt, WriteExt},
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    println,
    sync::Arc,
};

use arc_swap::ArcSwap;

type ResponderFn = Arc<dyn Fn(SocketAddr) -> Pin<Box<dyn Future<Output = String> + 'static>>>;

#[allow(dead_code)]
pub(crate) struct Server<A>
where
    A: ToSocketAddrs + ToString + 'static,
{
    tcp_listener: TcpListener,
    routes: ArcSwap<HashMap<Arc<str>, ResponderFn>>,
    addr: A,
}

impl<A> Server<A>
where
    A: ToSocketAddrs + ToString + 'static,
{
    pub(crate) async fn new(addr: A) -> std::io::Result<Arc<Self>> {
        let tcp_listener = TcpListener::bind(&addr).await?;
        Ok(Arc::new(Self {
            tcp_listener,
            routes: ArcSwap::new(Arc::new(HashMap::<Arc<str>, ResponderFn>::new())),
            addr,
        }))
    }

    pub(crate) fn add_route<R: Future<Output = String> + 'static>(
        self: Arc<Self>,
        route: &str,
        callback: fn(addr: SocketAddr) -> R,
    ) {
        let mut tmp = (*self.routes.load().clone()).clone();
        tmp.insert(route.into(), Arc::new(move |a| Box::pin(callback(a))));
        self.routes.swap(Arc::new(tmp));
    }

    pub(crate) async fn run(self: Arc<Self>) {
        loop {
            let tcp_listener = &self.tcp_listener;
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
        let msg = match self.routes.load().get(uri) {
            Some(f) => f(addr).await,
            None => "".to_string(),
        };

        stream
            .write_all(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
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
