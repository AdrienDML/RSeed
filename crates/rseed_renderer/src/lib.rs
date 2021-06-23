use std::ops::Deref;

use rseed_core::utils::Version;
use rseed_vk as vk;
use rseed_gl as gl;
use rseed_renderapi::{renderer::RendererT, Backend};
pub use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use glutin::{
    window::{
        Window,
        WindowBuilder,
    },
    event_loop::EventLoop,
};


#[derive(Debug)]
pub enum RendererError {

}


pub type Result<T> = std::result::Result<T,RendererError>;

pub struct Renderer {
    inner : Box<dyn RendererT>,
}

impl Deref for Renderer
{
    type Target = Box<dyn RendererT>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}




impl Renderer
{

    pub fn new(inner : Box<dyn RendererT>) -> Self {
        Self {
            inner : inner,
        }
    }

    pub fn init(
        window_builder : WindowBuilder,
        event_loop : &EventLoop<()>,
        app_name : String,
        app_version : Version,
        backend : Backend,
    ) -> Result<(Self, Window)> {
        match backend {
            Backend::VK => { 
                let window = window_builder.build(event_loop).unwrap();
                let ctx = unsafe {
                    vk::context::VkContext::init(app_name, app_version, &window)
                        .unwrap()
                };
                let renderer = Self::new(Box::new(vk::VkRenderer::new(ctx)));
                return Ok((renderer, window));
            }

            Backend::GL => {
                let (raw_context, window) = unsafe {
                    glutin::ContextBuilder::new()
                        .build_windowed(window_builder, &event_loop)
                        .unwrap()
                        .split()
                };
                let ctx = rseed_gl::context::GlContext::init(raw_context).unwrap();
                let renderer = Self::new(Box::new(gl::GlRenderer::new(ctx)));
                return Ok((renderer, window));
            }
        }
    }


    pub fn draw(&self) {
        self.inner.swap_buffers();
    }
}