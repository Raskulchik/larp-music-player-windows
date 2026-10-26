//! MPRIS is a Linux/Unix desktop integration. Windows has no session D-Bus,
//! so this module provides the same API surface as the real server without
//! starting anything.

use crate::api::Track;
use tokio::sync::mpsc;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MprisCommand {
    PlayPause,
    Pause,
    Resume,
    Stop,
    Next,
    Previous,
}

pub struct MprisServer;

impl MprisServer {
    pub async fn new(_cmd_tx: mpsc::UnboundedSender<MprisCommand>) -> anyhow::Result<Self> {
        Ok(Self)
    }

    pub async fn update_track(&self, _track: &Track) {}

    pub async fn set_playing(&self, _playing: bool) {}

    pub async fn set_volume(&self, _volume: f32) {}
}
