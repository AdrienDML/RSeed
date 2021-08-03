use std::path::PathBuf;

use rseed_core::{prelude::*, utils::version::Version};
use rseed_renderapi::Backend;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "serde")]
pub struct ProjectInfo {
    pub name: String,
    pub version: Version,
    pub asset_root: PathBuf,
    pub main_scene: PathBuf,
    pub window: WindowConf,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "serde")]
pub struct WindowConf {
    pub title: String,
    pub res: Resolution,
    pub visible: bool,
    pub mode: Mode,
    pub render_backend: Backend,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "serde")]
pub enum Mode {
    FullScreen,
    Windowed { size: Resolution, resizable: bool },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "serde")]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}
