// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod mugi_schema;
mod obs;
mod udp;
mod vlc_manager;

use std::sync::{Arc, Mutex};
use mugi_schema::MugiCmd;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::oneshot::{Receiver as OSRecever, Sender as OSSender};
use udp::bind_socket;
use vlc_manager::VlcManager;

// グローバル状態管理用の構造体
struct AppState {
    obs_connection_info: Arc<Mutex<Option<(String, u16, Option<String>)>>>,
    is_system_running: Arc<Mutex<bool>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            obs_connection_info: Arc::new(Mutex::new(None)),
            is_system_running: Arc::new(Mutex::new(false)),
        }
    }
}



#[tauri::command]
async fn play_highlights(
    video_paths: Vec<String>,
    state: tauri::State<'_, AppState>
) -> Result<String, String> {
    if video_paths.is_empty() {
        return Ok("再生する動画がありません".to_string());
    }
    
    // OBS接続情報を取得
    let (host, port, password) = {
        let conn_info = state.obs_connection_info.lock().unwrap();
        match conn_info.as_ref() {
            Some((host, port, password)) => (host.clone(), *port, password.clone()),
            None => return Err("OBS接続情報が見つかりません".to_string()),
        }
    };
    
    // OBS接続を作成
    let mut obs = obs::Obs::new();
    let password_ref = password.as_deref();
    obs.connect(&host, port, password_ref).await
        .map_err(|e| format!("Failed to connect to OBS: {}", e))?;
    
    // ファイル名からPathBufに変換（仮想的なパスとして扱う）
    let movie_pathes: Vec<std::path::PathBuf> = video_paths
        .iter()
        .map(|filename| std::path::PathBuf::from(filename))
        .collect();
    
    // VLCソースで動画再生
    if let Err(e) = obs.play_vlc_source(&movie_pathes).await {
        return Err(format!("Failed to play VLC source: {}", e));
    }
    
    Ok(format!("{}個のハイライト動画を再生しました", video_paths.len()))
}

#[tauri::command]
async fn connect_obs(
    host: String,
    port: u16,
    password: Option<String>,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    println!("Attempting to connect to OBS at {}:{}", host, port);
    
    // 既にシステムが動作中の場合はエラー
    {
        let is_running = state.is_system_running.lock().unwrap();
        if *is_running {
            return Err("システムは既に動作中です".to_string());
        }
    }
    
    let mut obs = obs::Obs::new();
    let password_ref = password.as_deref();
    
    // OBS接続試行
    match obs.connect(&host, port, password_ref).await {
        Ok(_) => {
            println!("Connected to OBS successfully");
            
            // リプレイバッファ設定
            if let Err(e) = obs.set_replay_buffer().await {
                return Err(format!("Failed to set replay buffer: {}", e));
            }
            
            // VLCソース初期化
            if let Err(e) = obs.init_vlc_source().await {
                return Err(format!("Failed to init VLC source: {}", e));
            }
            
            // 接続情報を保存
            {
                let mut conn_info = state.obs_connection_info.lock().unwrap();
                *conn_info = Some((host.clone(), port, password.clone()));
            }
            
            // システム開始
            start_system(host, port, password, state, app_handle).await?;
            
            Ok("OBS接続に成功しました".to_string())
        }
        Err(e) => {
            println!("Failed to connect to OBS: {}", e);
            Err(format!("OBS接続に失敗しました: {}", e))
        }
    }
}

async fn start_system(
    host: String,
    port: u16,
    password: Option<String>,
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    println!("Starting RL Replay system...");
    
    // システム動作中のフラグを設定
    {
        let mut is_running = state.is_system_running.lock().unwrap();
        *is_running = true;
    }
    
    // 別タスクでメインシステムを起動
    let host_clone = host.clone();
    let password_clone = password.clone();
    tokio::spawn(async move {
        if let Err(e) = run_main_system(host_clone, port, password_clone, app_handle).await {
            println!("Main system error: {}", e);
        }
    });
    
    println!("RL Replay system started successfully");
    Ok(())
}

async fn run_main_system(
    host: String,
    port: u16,
    password: Option<String>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // OBS接続を再作成
    let mut obs = obs::Obs::new();
    let password_ref = password.as_deref();
    obs.connect(&host, port, password_ref).await
        .map_err(|e| format!("Failed to reconnect to OBS: {}", e))?;
    
    obs.set_replay_buffer().await
        .map_err(|e| format!("Failed to set replay buffer: {}", e))?;
    
    obs.init_vlc_source().await
        .map_err(|e| format!("Failed to init VLC source: {}", e))?;
    
    // VlcManager初期化
    let vlc_manager = VlcManager::new();
    
    // イベントリスナー設定
    let (rb_tx, rb_rx) = mpsc::channel(32);
    obs.set_event_listener(rb_tx).await
        .map_err(|e| format!("Failed to set event listener: {}", e))?;
    
    vlc_manager.set_event_listener(rb_rx, app_handle.clone());
    
    // UDPサーバー開始
    let (tx, mut rx) = mpsc::channel::<String>(32);
    tokio::spawn(async {
        if let Err(e) = bind_socket(tx).await {
            println!("UDP socket error: {}", e);
        }
    });
    
    // UDPメッセージ処理 - 無限ループで動作し続ける
    while let Some(d) = rx.recv().await {
        let cmd = mugi_schema::parse_cmd(&d);
        match cmd {
            Err(_) => println!("Failed to parse:{}", d),
            Ok(cmd) => {
                if cmd == MugiCmd::Scored || cmd == MugiCmd::EpicSave {
                    println!("OBS fire!");
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    if let Err(e) = obs.save_replay_buffer().await {
                        println!("Failed to save replay buffer: {}", e);
                    }
                }
                if cmd == MugiCmd::Dbg {
                    // DBGコマンドは現在フロントエンド経由で処理される
                    println!("DBG command received - handled by frontend");
                }
            }
        }
    }
    
    println!("UDP receiver closed, system shutting down");
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    console_subscriber::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![connect_obs, play_highlights])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
