mod shaderconfig;

use std::path::PathBuf;

use rseed_renderapi::Backend;
pub use shaderconfig::ShaderConf;

#[derive(Clone)]
pub struct Config {
    asset_root : PathBuf,
    shader_conf : ShaderConf,
}

impl Config {

    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn shader(&mut self) -> &mut ShaderConf {
        &mut self.shader_conf
    } 

}

impl Default for Config {
    fn default() -> Self {
        Self {
            asset_root : PathBuf::from("asset"),
            shader_conf : ShaderConf {shader_target : Backend::GL},
        }
    }
}