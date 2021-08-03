use ash::{self, vk};

#[derive(Clone, Debug)]
pub enum PipelineError {}

pub type Result<T> = std::result::Result<T, PipelineError>;

#[allow(unused)]
pub(crate) struct Pipeline {
    pub pipeline: vk::Pipeline,
    pub layout: vk::PipelineLayout,
}

impl Pipeline {
    #[allow(unused)]
    pub fn init() -> () {
        todo!()
    }
}
