use std::path::PathBuf;

use futures_util::{StreamExt, pin_mut};
use obws::{Client, events::Event, requests::custom::source_settings::SlideshowFile};
use tokio::sync::{OnceCell, mpsc::Sender};
use log::{debug};

use time::Duration;
const UNIQUE_REPLAY_SOURCE_NAME: &str = "RL_REPLAY_VLC_SOURCE";

pub struct Obs {
    client: Option<Client>,
    host: OnceCell<String>,
    port: OnceCell<u16>,
    password: OnceCell<Option<String>>,
}

impl Obs {
    pub fn new() -> Self {
        Obs {
            client: None,
            host: OnceCell::new(),
            port: OnceCell::new(),
            password: OnceCell::new(),
        }
    }
    pub async fn connect(
        &mut self,
        host: &str,
        port: u16,
        password: Option<&str>,
    ) -> Result<(), obws::error::Error> {
        let client = Client::connect(host, port, password).await?;
        self.client = Some(client);
        self.host.set(host.to_string()).unwrap();
        self.port.set(port).unwrap();
        if let Some(pass) = password {
            self.password.set(Some(pass.to_string())).unwrap();
        } else {
            self.password.set(None).unwrap();
        }
        Ok(())
    }

    fn get_client(&self) -> Result<&Client, String> {
        let client = &self.client;
        let client = match client {
            Some(c) => c,
            None => return Err("failed to get client".to_string()),
        };
        Ok(client)
    }

    async fn get_replay_buffer_status(&self, client: &Client) -> Result<bool, String> {
        let res = client.replay_buffer().status().await;
        match res {
            Ok(res) => Ok(res),
            Err(_) => Err("failed to get replay_buffer status".to_string()),
        }
    }
    pub async fn set_replay_buffer(&self) -> Result<(), String> {
        let client = self.get_client()?;
        let status = self.get_replay_buffer_status(client).await?;
        // もうONだったらreturn
        if status {
            return Ok(());
        }
        let res = client.replay_buffer().start().await;
        if let Err(e) = res {
            return Err(e.to_string());
        }
        Ok(())
    }

    pub async fn save_replay_buffer(&self) -> Result<(), String> {
        let client = self.get_client()?;
        let res = client.replay_buffer().save().await;
        if let Err(e) = res {
            return Err(e.to_string());
        }
        Ok(())
    }

    pub async fn init_vlc_source(&self) -> Result<(), String> {
        if self.is_exit_vlc_soruce().await? {
            return Ok(());
        }

        let client = self.get_client()?;
        let current_scene = self.get_current_scene().await?;

        let vlc_setting = obws::requests::custom::source_settings::VlcSource {
            loop_: false,
            shuffle: false,
            playback_behavior:
                obws::requests::custom::source_settings::PlaybackBehavior::StopRestart,
            playlist: &[],
            network_caching: Duration::milliseconds(100),
            track: 1,
            subtitle_enable: false,
            subtitle: 0,
        };
        let create = obws::requests::inputs::Create {
            scene: current_scene.id.into(),
            input: UNIQUE_REPLAY_SOURCE_NAME,
            kind: obws::requests::custom::source_settings::SOURCE_VLC_SOURCE,
            settings: Some(vlc_setting),
            enabled: Some(false),
        };
        let res = client.inputs().create(create).await;
        match res {
            Ok(_) => debug!("VLC source created"),
            Err(e) => return Err(format!("Failed to create VLC source: {e}")),
        }
        Ok(())
    }

    pub async fn play_vlc_source(&self, movie_pathes: &[PathBuf]) -> Result<(), String> {
        let client = self.get_client()?;
        let playlists: Vec<SlideshowFile> = movie_pathes
            .iter()
            .map(|path| SlideshowFile {
                value: path.as_path(),
                hidden: false,
                selected: false,
            })
            .collect();
        let vlc_setting = obws::requests::custom::source_settings::VlcSource {
            loop_: false,
            shuffle: false,
            playback_behavior:
                obws::requests::custom::source_settings::PlaybackBehavior::StopRestart,
            playlist: &playlists,
            network_caching: Duration::milliseconds(100),
            track: 1,
            subtitle_enable: false,
            subtitle: 0,
        };
        let input_setting = obws::requests::inputs::SetSettings {
            input: obws::requests::inputs::InputId::Name(UNIQUE_REPLAY_SOURCE_NAME),
            overlay: Some(true),
            settings: &vlc_setting,
        };
        let res = client.inputs().set_settings(input_setting).await;
        match res {
            Ok(_) => debug!("VLC source updated"),
            Err(e) => return Err(format!("Failed to create VLC source: {e}")),
        }
        // Sourceの有効化
        let current_scene = self.get_current_scene().await?;
        let current_scene_id = current_scene.id;
        let scene_items = client
            .scene_items()
            .list(current_scene_id.clone().into())
            .await;
        let scene_items = match scene_items {
            Ok(scene_items) => scene_items,
            Err(_) => return Err("Failed to get scene items".to_string()),
        };
        let unique_replay_source_item = match scene_items
            .iter()
            .find(|&item| item.source_name == UNIQUE_REPLAY_SOURCE_NAME)
        {
            Some(d) => d,
            None => return Err("Failed to find unique_replay_source_item".to_string()),
        };
        let set_enabled: obws::requests::scene_items::SetEnabled<'_> =
            obws::requests::scene_items::SetEnabled {
                scene: current_scene_id.into(),
                item_id: unique_replay_source_item.id,
                enabled: true,
            };
        let res = client.scene_items().set_enabled(set_enabled).await;
        if let Err(e) = res {
            return Err(format!("Failed to set VLC source enabled: {e}"));
        }
        Ok(())
    }

    async fn get_current_scene(
        &self,
    ) -> Result<obws::responses::scenes::CurrentProgramScene, String> {
        let client = self.get_client()?;
        let current_scene = client.scenes().current_program_scene().await;
        match current_scene {
            Ok(current_scene) => Ok(current_scene),
            Err(_) => Err("Failed to get current scene".to_string()),
        }
    }

    async fn is_exit_vlc_soruce(&self) -> Result<bool, String> {
        let client = self.get_client()?;
        let res = client
            .inputs()
            .list(Some(
                obws::requests::custom::source_settings::SOURCE_VLC_SOURCE,
            ))
            .await;
        match res {
            Ok(inputs) => {
                let is_exist = inputs
                    .iter()
                    .find(|&i| i.id.name == UNIQUE_REPLAY_SOURCE_NAME);
                Ok(is_exist.is_some())
            }
            Err(_) => Err("Failed to get inputs".to_string()),
        }
    }

    pub async fn set_event_listener(&self, tx: Sender<PathBuf>) -> Result<(), String> {
        let host = self.host.get().unwrap();
        let port = self.port.get().unwrap().to_owned();
        let password = self.password.get().unwrap().as_ref().map(|d| d.as_str());

        let client = Client::connect(host, port, password).await.unwrap();
        tokio::spawn(async move {
            let events = client.events().unwrap();
            pin_mut!(events);
            while let Some(event) = events.next().await {
                if let Event::ReplayBufferSaved { path } = event {
                    tx.send(path).await.unwrap();
                }
            }
        });

        Ok(())
    }
}
