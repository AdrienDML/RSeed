use std::str::FromStr;

use rseed_log::Logger;
use rseed_core::utils::Version;
use rseed_renderer::Renderer;


use glutin::{
    self,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
}

pub struct App {
    pub logger: Logger,
    pub event_loop: EventLoop<()>,
    pub renderer: Renderer,
    pub window: Window,
}

impl App {
    pub fn init(width: u32, height: u32, app_name : String, app_version : Version) -> Result<Self> {
        let logger = Logger::new(String::from_str("RS-eed").unwrap());
        
        

        // Create Graphic pipeline
        let (renderer, window, event_loop) = Renderer::init(
            app_name,
            app_version,
            width,
            height,
        ).unwrap();


        Ok(Self {
            logger,
            event_loop,
            renderer,
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
