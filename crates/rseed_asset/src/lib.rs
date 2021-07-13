use rseed_core::prelude::*;


mod config;
mod assetloader;
mod asset;
mod assetcache;

use assetloader::AssetLoader;

#[derive(Debug, Error)]
pub enum AssetError {
}

pub type Result<T> = std::result::Result<T, AssetError>;

pub struct AssetManager {
    loader : AssetLoader,
}

impl AssetManager {

    pub fn init() -> Self {
        todo!()
    }

}

