use ash::{
    self,
    vk,
    extensions::khr,
    Entry, Instance,
};

use super::{
    window::{self, HasRawWindowHandle},
};


#[derive(Clone, Debug)]
pub enum SurfaceError {
    CreationFailed(vk::Result),
    Extension(window::WindowError)
}

pub type Result<T> = std::result::Result<T, SurfaceError>;

pub struct Surface {
    pub khr : vk::SurfaceKHR,
    pub loader : khr::Surface,
}

impl Surface {
    pub fn init(entry : &Entry, instance : &Instance, window_handle: &dyn HasRawWindowHandle) -> Result<Self> {
        // Surface creation
        let surface_khr = unsafe {
            window::create_surface(entry, instance, window_handle).map_err(|e| match e {
                window::WindowError::ExtensionNotPresent(_) => SurfaceError::Extension(e),
                window::WindowError::SurfaceCreationFailed(ee) => SurfaceError::CreationFailed(ee),
            })?};
        let surface_loader = khr::Surface::new(entry, instance);
        Ok(Self {
            khr : surface_khr,
            loader : surface_loader,
        })
    }

}