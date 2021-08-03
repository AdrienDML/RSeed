use rseed_core::{prelude::*, utils::version::Version};
pub use rseed_renderapi::Backend;
use rseed_renderer::Renderer;

use super::config::{self, ProjectInfo, WindowConf};

use glutin::{
    self,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowBuilder},
};

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(display = "Project file not found.")]
    ProjectFileNotFound,
    #[error(display = "Project file invalid: {}", _0)]
    ProjectFileInvalid(rseed_core::serialization::toml::de::Error),
    #[error(display = "Io error: {}", _0)]
    IoError(std::io::Error),
    #[error(display = "Log init error : {}", _0)]
    LogError(rseed_core::log::InitError),
    #[error(display = "Renderer init error : {}", _0)]
    RendererError(rseed_renderer::RendererError),
}

pub struct App {
    pub event_loop: EventLoop<()>,
    pub renderer: Renderer,
    pub window: Window,
}

impl App {
    fn get_toml_config() -> Result<ProjectInfo> {
        let manifest_dir =
            std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str());
        let md_contents = std::fs::read_dir(manifest_dir).unwrap();
        let config_path = md_contents
            .filter_map(|e| e.ok().map(|dir| dir.path()))
            .filter(|path| Some("toml") == path.extension().map(|e| e.to_str().unwrap()))
            .filter(|path| {
                path.file_stem()
                    .map(|e| e.to_str().unwrap().find(".proj"))
                    .flatten()
                    .is_some()
            })
            .next()
            .ok_or(AppError::ProjectFileNotFound)?;
        let mut file = std::fs::File::open(config_path).unwrap();
        let mut contents = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut contents).map_err(|e| AppError::IoError(e))?;
        let toml = String::from_utf8(contents).unwrap();
        from_str(toml.as_str()).map_err(|e| AppError::ProjectFileInvalid(e))
    }

    pub fn configure_window_builder(
        wb: WindowBuilder,
        conf: &WindowConf,
        el: &EventLoop<()>,
    ) -> WindowBuilder {
        use config::{Mode, Resolution};
        let mut wb = wb
            .with_title(conf.title.to_owned())
            .with_visible(conf.visible);
        match conf.mode {
            Mode::FullScreen => {
                let handle = el.primary_monitor();
                wb =
                    wb.with_fullscreen(Some(Fullscreen::Borderless(handle)))
                        .with_inner_size(glutin::dpi::Size::Physical(
                            glutin::dpi::PhysicalSize::new(conf.res.width, conf.res.height),
                        ));
            }
            Mode::Windowed {
                size: Resolution { width, height },
                resizable,
            } => {
                wb = wb
                    .with_resizable(resizable)
                    .with_inner_size(glutin::dpi::Size::Physical(glutin::dpi::PhysicalSize::new(
                        width, height,
                    )))
            }
        };
        wb
    }

    pub fn from_config(config: &mut Option<ProjectInfo>) -> Result<Self> {
        let project_conf = config.get_or_insert(Self::get_toml_config()?);
        let event_loop = EventLoop::new();
        let window_builder =
            Self::configure_window_builder(WindowBuilder::new(), &project_conf.window, &event_loop);
        let (renderer, window) = Renderer::init(
            window_builder,
            &event_loop,
            project_conf.name.to_owned(),
            project_conf.version,
            project_conf.window.render_backend,
        )
        .map_err(|e| AppError::RendererError(e))?;
        Ok(Self {
            event_loop,
            renderer,
            window,
        })
    }

    pub fn run(self) {
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
