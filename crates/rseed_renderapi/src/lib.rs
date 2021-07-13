/// This crates define all the api for the renderer components as traits wich have to be implemented for each render target.
pub mod context;
pub mod renderer;
pub mod buffer;
pub mod texture;

use rseed_core::prelude::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum Backend {
    GL,
    VK,
    UNDEFINED,
}
