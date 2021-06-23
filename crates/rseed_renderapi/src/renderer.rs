/// All the renderer platform supported by the engine at the moment.

pub trait RendererT : Drop {
    fn swap_buffers(&self);

}


pub enum RenderCommand {
    Clear,
    SetClearColor,
    DrawIndexed,
    Draw,
    Flush,
}