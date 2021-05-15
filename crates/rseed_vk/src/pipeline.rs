use ash::{self, vk};

#[derive(Clone, Debug)]
pub enum PipelineError {}

pub type Result<T> = std::result::Result<T, PipelineError>;

pub(crate) struct Pipeline {
    pub pipeline: vk::Pipeline,
    pub layout: vk::PipelineLayout,
}

impl Pipeline {}
