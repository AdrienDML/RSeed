use rseed_renderapi::renderer::RendererT;

pub mod context;

pub struct VkRenderer {
    #[allow(unused)]
    ctx: context::VkContext,
}

impl VkRenderer {
    pub fn new(ctx: context::VkContext) -> Self {
        Self { ctx }
    }
}

impl Drop for VkRenderer {
    fn drop(&mut self) {}
}

impl RendererT for VkRenderer {
    fn swap_buffers(&self) {}
}
