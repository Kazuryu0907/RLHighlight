use std::{
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use time::Duration;

use obws::requests::custom::source_settings::SlideshowFile;
use tokio::sync::{Mutex, mpsc::Receiver};

static VIDEOPATHES: LazyLock<Mutex<Vec<PathBuf>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub struct VlcManager {}

impl VlcManager {
    pub fn new() -> Self {
        Self {}
    }
    // replay_bufferのpath一覧をVecに保存する
    // rx: OBSのreplay_bufferのpathが降ってくる
    pub fn set_event_listener(&self, mut rx: Receiver<PathBuf>) {
        tokio::spawn(async move {
            while let Some(path) = rx.recv().await {
                println!("path:{:?}", path);
                {
                    let video_pathes = &VIDEOPATHES;
                    let mut video_pathes = video_pathes.lock().await;
                    video_pathes.push(path);
                }
            }
        });
    }

    pub async fn get_pathes(&self) -> Vec<PathBuf> {
        let video_pathes = &VIDEOPATHES;
        let video_pathes = video_pathes.lock().await;
        video_pathes.clone()
    }

    pub async fn clear_videos(&self) {
        let video_pathes = &VIDEOPATHES;
        let mut video_pathes = video_pathes.lock().await;
        video_pathes.clear();
    }
}
