mod mugi_schema;
mod obs;
mod udp;
mod vlc_manager;

use mugi_schema::MugiCmd;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot::{Receiver as OSRecever, Sender as OSSender};
use udp::bind_socket;
use vlc_manager::VlcManager;

#[tokio::main]
async fn main() {
    console_subscriber::init();
    let mut obs = obs::Obs::new();
    let vlc_manager = VlcManager::new();
    obs.connect("localhost", 4455, None)
        .await
        .expect("failed to login to obs");
    obs.set_replay_buffer()
        .await
        .expect("failed to set replay_buffer");
    let (rb_tx, rb_rx) = mpsc::channel(32);
    obs.set_event_listener(rb_tx)
        .await
        .expect("failed to set_event_listener");

    obs.init_vlc_source().await.unwrap();
    // pathをこれを送る
    vlc_manager.set_event_listener(rb_rx);

    let (tx, mut rx) = mpsc::channel::<String>(32);
    tokio::spawn(async {
        bind_socket(tx).await.unwrap();
    });
    // UDPソケット受信
    tokio::spawn(async move {
        while let Some(d) = rx.recv().await {
            let cmd = mugi_schema::parse_cmd(&d);
            match cmd {
                Err(_) => println!("Failed to parse:{}", d),
                Ok(cmd) => {
                    // とりあえずゴール時のみ
                    if cmd == MugiCmd::Scored || cmd == MugiCmd::EpicSave {
                        println!("OBS fire!");
                        // 3秒寝かせる
                        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                        obs.save_replay_buffer()
                            .await
                            .expect("failed to save replay_buffer");
                    }
                    if cmd == MugiCmd::Dbg {
                        let movie_pathes = vlc_manager.get_pathes().await;
                        obs.play_vlc_source(&movie_pathes).await.unwrap();
                        vlc_manager.clear_videos().await;
                    }
                }
            }
        }
    })
    .await
    .unwrap();
}
