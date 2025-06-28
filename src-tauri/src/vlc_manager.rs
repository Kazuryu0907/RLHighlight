use std::path::PathBuf;

use log::{error, info};
use tauri::Emitter;
use tokio::sync::mpsc::Receiver;

pub struct VlcManager {}

impl VlcManager {
    pub fn new() -> Self {
        Self {}
    }
    // replay_bufferのpathをフロントエンドに送信
    // rx: OBSのreplay_bufferのpathが降ってくる
    pub fn set_event_listener(&self, mut rx: Receiver<PathBuf>, app_handle: tauri::AppHandle) {
        tokio::spawn(async move {
            while let Some(path) = rx.recv().await {
                info!("path:{:?}", path);
                // フロントエンドに個別のパスを送信
                if let Err(e) = app_handle.emit("video_path_added", path) {
                    error!("Failed to emit video_path_added event: {}", e);
                }
            }
        });
    }
}
