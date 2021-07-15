use std::str::FromStr;

use rseed_log::Logger;
use rseed_core::{
    prelude::*,
    utils::Version,
};
use rseed_renderer::{Renderer};
pub use rseed_renderapi::Backend;

use super::ProjectInfo;

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

#[derive(Debug, Error)]
pub enum AppError {
    #[error(display="Project file not found.")]
    ProjectFileNotFound,
    #[error(display="Project file invalid: {}", _0)]
    ProjectFileInvalid(rseed_core::serialization::toml::de::Error),
    #[error(display="Io error: {}", _0)]
    IoError(std::io::Error)

}

pub struct App {
    pub logger: Logger,
    pub event_loop: EventLoop<()>,
    pub renderer: Renderer,
    pub window: Window,
}

impl App {

    pub fn from_toml_config() -> Result<Self> {
        let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str());
        let md_contents = std::fs::read_dir(manifest_dir).unwrap();

        let ppf = {
        md_contents.filter_map(
                |e| {
                    e.ok()
                    .map(
                        |dir| {
                            dir.path()
                        }
                    )
                }
            ).filter(
                |path| {
                    Some("toml") == path.extension().map(|e| e.to_str().unwrap())
                }
            ).filter(
                |path| {
                    path.file_stem().map(|e| e.to_str().unwrap().find(".proj")).flatten().is_some()
                }
            )}.next()
            .ok_or(AppError::ProjectFileNotFound)?;
        
        let mut file = std::fs::File::open(ppf).unwrap();
        let mut contents = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut contents)
            .map_err(|e| AppError::IoError(e))?;
        let toml = String::from_utf8(contents).unwrap();
        let proj : ProjectInfo = from_str(toml.as_str())
            .map_err(|e| AppError::ProjectFileInvalid(e))?;
        Self::init(proj.window.width, proj.window.height, proj.name, proj.version, proj.window.render_backend)
    }

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
