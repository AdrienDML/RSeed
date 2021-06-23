pub mod context;

use rseed_renderapi::renderer::RendererT;


pub struct GlRenderer {
    ctx : context::GlContext,
}

impl GlRenderer {

    pub fn new(ctx: context::GlContext) -> Self {
        Self {
            ctx,
        }
    }

}

impl Drop for GlRenderer {
    fn drop(&mut self) {
    }
}

impl RendererT for GlRenderer {

    fn swap_buffers(&self) {
        self.ctx.swap_buffers()
    }
}