use rseed_core::prelude::*;



use super::{
    config::Config,
};

#[derive(Debug, Error)]
pub enum LoaderError {

}

pub type Result<T> = std::result::Result<T, LoaderError>;

pub enum AssetState {
    Loaded,
    NotLoaded,
    Inexistent,
}

#[derive(Builder, Clone)]
pub struct AssetLoader {
    config : Config,
}

impl AssetLoader {
    pub fn from_cfg(cfg : Config) -> Self {
        Self {
            config : cfg,
        }
    }
}




