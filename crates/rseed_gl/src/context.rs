use gl;

use glutin::{NotCurrent, PossiblyCurrent};

#[derive(Debug)]
pub enum ContextError {
    ContextCurentError(glutin::ContextError),
}

pub type Result<T> = std::result::Result<T, ContextError>;


pub struct GlContext {
    pub gl : gl::Gl,
    pub raw_ctx : glutin::RawContext<PossiblyCurrent>,
} 



impl GlContext
{
    pub fn init(
        raw_context : glutin::RawContext<NotCurrent>
    ) -> Result<Self> {

        let raw_ctx = unsafe {raw_context.make_current()}
        .or_else(
            |(_ctx, err)| Err(ContextError::ContextCurentError(err)))?;
        println!("Acitve ctx ? {}", raw_ctx.is_current());
        let gl = gl::Gl::load_with(|s| raw_ctx.get_proc_address(s));
        Ok(Self {
            gl,
            raw_ctx,
        })
    }

    pub fn swap_buffers(&self) {
        self.raw_ctx.swap_buffers().unwrap()
    } 
}

impl Drop for GlContext {
    fn drop(&mut self) {
        
    }
}