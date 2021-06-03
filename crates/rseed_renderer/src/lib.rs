#[cfg(feature="vk")]
pub mod rexports {
    use rseed_vk;
    pub use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
}
#[cfg(feature="gl")]
use rseed_gl;

use glutin::{
    window::{
        Window,
        WindowBuilder,
    },
    event_loop::EventLoop,
};

use rseed_core::utils::Version;

#[derive(Debug)]
pub enum RendererError {

}

pub type Result<T> = std::result::Result<T,RendererError>;

pub struct Renderer {
    #[cfg(feature="vk")]
    context : rseed_vk::context::VkContext,
    #[cfg(feature="gl")]
    context : rseed_gl::context::GlContext,
}

impl Renderer {
    
    pub fn init(
        app_name : String,
        app_version : Version,
        w_width : u32,
        w_height : u32,
    ) -> Result<(Self, Window, EventLoop<()>)> {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(glutin::dpi::Size::Physical(glutin::dpi::PhysicalSize::new(
                w_width, w_height,
            )))
            .with_title(format!("{}: {}", app_name, app_version))
            .with_visible(true);



        #[cfg(feature="vk")]
        let (window, context) = (
            window_builder.build(&event_loop).unwrap(),
            rseed_vk::context::VkContext::init(app_name, app_version, window)
        );

        #[cfg(feature="gl")]
        let (context, window)  = {
            let (raw_context, window) = unsafe {
                glutin::ContextBuilder::new()
                    .build_windowed(window_builder, &event_loop)
                    .unwrap()
                    .split()
            };
            (rseed_gl::context::GlContext::init(raw_context).unwrap(), window)
        };


        Ok ((
            Self {
                context,
            },
            window,
            event_loop,
        ))

    }

    pub fn draw(&self) {
        self.context.swap_buffers()
    }

}