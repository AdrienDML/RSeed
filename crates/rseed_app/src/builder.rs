use std::path::PathBuf;

use rseed_core::{
    utils::Version,
    prelude::*,
};
use rseed_renderapi::Backend;


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "serde")]
pub struct ProjectInfo {
    pub name : String,
    pub version : Version,
    pub asset_root : PathBuf,
    pub main_scene : PathBuf,
    pub window : Window,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "serde")]
pub struct Window {
    pub title : String,
    pub width: u32,
    pub height: u32,
    pub visible : bool,
    pub resizable : bool,
    pub render_backend : Backend,
}