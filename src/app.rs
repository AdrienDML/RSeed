use std::str::FromStr;

use crate::core::context::*;
use rseed_log::Logger;

use winit::{
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
    pub context: VkContext,
    pub event_loop: EventLoop<()>,
    //pub window : Window,
    //pub surface: std::sync::Arc<vk::swapchain::Surface<Window>>,
}

impl App {
    pub fn init() -> Result<Self> {
        let _logger = Logger::new(String::from_str("RS-eed").unwrap());
        let context = unsafe { VkContext::init(String::from("Test"), (0,0,1).into()).unwrap() };
        let event_loop = EventLoop::new();
        //let surface = ash_window::;
        //    .unwrap();
        //let window = surface.build(&event_loop).unwrap();
        Ok(Self {
            context,
            event_loop,
            //window,
        })
    }

    pub fn run(self) {
        let mut should_quit = false;
        while !should_quit {
            self.event_loop.poll
        }
        self.event_loop.run(|event, _, cf| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *cf = ControlFlow::Exit,
            _ => (),
        })
    }
}
