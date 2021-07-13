use rseed_core::prelude::*;

use rseed_renderapi::Backend;

#[derive(Builder, Clone, Copy)]
pub struct ShaderConf {
    pub shader_target : Backend,
}

