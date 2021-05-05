use std::str::FromStr;

use rseed_vk::context::*;
use rseed_log::Logger;

use winit::{
    self,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Context(ContextError),
}

pub struct App {
    pub logger: Logger,
    pub context: VkContext,
    pub event_loop: EventLoop<()>,
    pub window: Window,
    //pub surface: std::sync::Arc<vk::swapchain::Surface<Window>>,
}

impl App {
    pub fn init(width: u32, height: u32) -> Result<Self> {
        let logger = Logger::new(String::from_str("RS-eed").unwrap());
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize::new(
                width, height,
            )))
            .build(&event_loop)
            .unwrap();

        // Create the Vulkan context
        let context =
            unsafe { VkContext::init(String::from("Test"), (0, 0, 1).into(), &window).unwrap() };

        Ok(Self {
            logger,
            context,
            event_loop,
            window,
        })
    }

    pub fn run(self) {
        self.event_loop.run(|event, _, cf| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *cf = ControlFlow::Exit,
            _ => (),
        })
    }
}
