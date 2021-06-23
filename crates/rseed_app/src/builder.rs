use rseed_core::utils::Version;
use rseed_renderapi::Backend;

pub type Result<T> = std::result::Result<T, BuilderError>;

pub enum BuilderError {
    NoBackendSelecetd,
}

pub struct AppInfo {
    width: u32,
    height: u32,
    name : String,
    version : Version,
    backend : Option<Backend>,
}

impl AppInfo {
    pub fn new() -> Self {
        Self {
            width : 300,
            height : 300,
            name : String::from("Default App"),
            version : Version::new(),
            backend : None,
        }
    }

    pub fn width(self, w : u32) -> Self {
        self.width = w;
        self
    }

    pub fn height(self, h : u32) -> Self {
        self.hight = h;
        self
    }

    pub fn name(self, name : impl Into<String>) -> Self {
        self.name = name;
        self
    }

    pub fn backend(self, be : Backend) -> Self {
        self.backend = be;
        self
    }

    pub fn build(self) -> Result<App> {
        match self.backend {
            None => Err(BuilderError::NoBackendSelecetd),
            Some(_) => Ok(App::new(self.width, self.height, self.name, self.version, self.backend)),
        }
    }
}