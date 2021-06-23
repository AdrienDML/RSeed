use std::str::FromStr;

use rseed_log::Logger;
use rseed_core::utils::Version;
use rseed_renderer::{Renderer};
pub use rseed_renderapi::Backend;

use glutin::{
    self,
    event::{
        Event,
        WindowEvent
    },
    event_loop::{
        ControlFlow, 
        EventLoop
    }, 
    window::{
        WindowBuilder,
        Window
    }
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
    pub fn init(width: u32, height: u32, app_name : String, app_version : Version, backend : Backend) -> Result<Self> {
        let logger = Logger::new(String::from_str("RS-eed").unwrap());
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(glutin::dpi::Size::Physical(glutin::dpi::PhysicalSize::new(
                width, height,
            )))
            .with_title(format!("{}: {}", app_name, app_version))
            .with_visible(true);
        // Create Graphic pipeline
        let (renderer, window) = Renderer::init(
            window_builder,
            &event_loop,
            app_name,
            app_version,
            backend,
        ).unwrap();


        Ok(Self {
            logger,
            event_loop,
            renderer,
            window,
        })
    }

    pub fn run(self) {
        self.logger.info(&String::from("The app is running!"));
        let renderer = self.renderer;
        self.event_loop.run(move |event, _, cf| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *cf = ControlFlow::Exit,
            Event::RedrawRequested(_) => renderer.draw(),
            _ => (),
        })
    }
}


