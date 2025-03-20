mod udp;
mod mugi_scheme;
mod obs;


use mugi_scheme::MugiCmd;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot::{Receiver as OSRecever,Sender as OSSender};
use udp::bind_socket;


async fn udp_receiver(mut rx: Receiver<String>,tx: Sender<bool>){
    println!("spawned!");
    while let Some(d) = rx.recv().await {
        println!("udp {}",d);
        let cmd = mugi_scheme::parse_cmd(&d);
        match cmd{
            Err(_) => println!("Failed to parse:{}",d),
            Ok(cmd) => {
                // とりあえずゴール時のみ
                if cmd == MugiCmd::Scored{
                    tx.send(true).await.expect("failed to send from udp_receiver");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    obs::obs("localhost", 4455, None).await.unwrap();
    let (tx,  rx) = mpsc::channel::<String>(32);
    tokio::spawn(async move{
        bind_socket(tx).await.unwrap();
    });
    let (udp2obs_tx, mut udp2obs_rx) = mpsc::channel::<bool>(32);
    tokio::spawn(async move{
        udp_receiver(rx, udp2obs_tx).await;
    }); 
    tokio::spawn(async move{
        while let Some(_) = udp2obs_rx.recv().await{
            println!("OBS fire!");
        }
    });
    println!("Hello, world!");
    loop{}
}
