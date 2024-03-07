use async_std::{
    io::WriteExt,
    net::{SocketAddr, TcpListener, TcpStream},
    println,
    task::spawn_local,
};

#[async_std::main]
async fn main() {
    let listner = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    loop {
        let (stream, addr) = listner.accept().await.unwrap();
        spawn_local(responder(stream, addr));
    }
}

async fn responder(mut stream: TcpStream, addr: SocketAddr) {
    println!("{:?} ", addr).await;
    let msg = "This is yet a demo server in rust using Async!";
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
