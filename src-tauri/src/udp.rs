use std::io;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::Sender;
// use tauri::async_runtime::{Receiver,Sender};

pub async fn bind_socket(tx: Sender<String>) -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:12345").await?;
    println!("Listening on {}", sock.local_addr()?);
    let mut buf = [0; 1024];
    // let mut f = File::create("mugi_log.txt").await?;
    loop {
        let (size, _addr) = sock.recv_from(&mut buf).await?;
        let data = std::str::from_utf8(&buf[..size]).unwrap();
        let d = data.to_string();
        tx.send(d).await.unwrap();
    }
}
