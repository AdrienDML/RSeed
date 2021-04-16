use crate::core::context::*;
use vulkano as vk;
use vulkano_win::VkSurfaceBuild;
use winit::{event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Window, WindowBuilder}};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Context(ContextError),
}

pub struct App {
    pub context : VkContext,
    pub event_loop : EventLoop<()>,
    pub surface : std::sync::Arc<vk::swapchain::Surface<Window>>,
}

impl App {

    pub fn init() -> Result<Self> {
        let context = VkContext::init().unwrap();
        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new().build_vk_surface(&event_loop, context.instance.clone()).unwrap();
        //let window = WindowBuilder::new().build(&event_loop).unwrap();
        Ok(Self {
            context,
            event_loop,
            surface,
        })
    }

    pub fn run(self) {
        self.event_loop.run( |event, _, cf|
            match event {
            Event::WindowEvent{ event: WindowEvent::CloseRequested, ..} => 
                *cf = ControlFlow::Exit,
            _ => (),
        })
    }
}
