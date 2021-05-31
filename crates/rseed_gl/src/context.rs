use gl;
use rseed_core::utils::Version;

use glutin::{ContextCurrentState, NotCurrent, PossiblyCurrent};

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
    pub fn init<>(
        raw_context : glutin::RawContext<NotCurrent>
    ) -> Result<Self> {

        let raw_ctx = match unsafe {raw_context.make_current()} {
            Ok(ctx) => ctx,
            Err((ctx,err)) => return Err(ContextError::ContextCurentError(err)),
        };
        let gl = gl::Gl::load_with(|s| raw_ctx.get_proc_address(s));
        Ok(Self {
            gl,
            raw_ctx,
        })
    }
}