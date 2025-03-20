use std::io;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::{Sender,};
// use tauri::async_runtime::{Receiver,Sender};

pub async fn bind_socket(tx:Sender<String>) -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:12345").await?;
    println!("Listening on {}", sock.local_addr()?);
    let mut buf = [0; 1024];
    // let mut f = File::create("mugi_log.txt").await?;
    loop {
        match sock.recv_from(&mut buf).await? {
            (size, addr) => {
                let data = std::str::from_utf8(&buf[..size]).unwrap();
                println!("{:?} from {:?}", data, addr);
                let d = data.to_string();
                // f.write(data.as_bytes()).await?;
                // f.write("\n".as_bytes()).await?;
                // f.flush().await?;
                tx.send(d).await.unwrap();
                // sock.send_to(&buf[..size], addr).await?;
            }
        }
    }
    Ok(())
}