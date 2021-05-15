use ash::{self, extensions::khr, vk};

use super::library::*;
use super::window::{self, HasRawWindowHandle};

#[derive(Clone, Debug)]
pub enum SurfaceError {
    CreationFailed(vk::Result),
    Extension(window::WindowError),
}

pub(crate) type Result<T> = std::result::Result<T, SurfaceError>;

pub(crate) struct Surface {
    pub khr: vk::SurfaceKHR,
    pub loader: khr::Surface,
}

impl Surface {
    pub fn init(lib: &Library, window_handle: &dyn HasRawWindowHandle) -> Result<Self> {
        // Surface creation
        let surface_khr = unsafe {
            window::create_surface(&lib.entry, &lib.instance, window_handle).map_err(
                |e| match e {
                    window::WindowError::ExtensionNotPresent(_) => SurfaceError::Extension(e),
                    window::WindowError::SurfaceCreationFailed(ee) => {
                        SurfaceError::CreationFailed(ee)
                    }
                },
            )?
        };
        let surface_loader = khr::Surface::new(&lib.entry, &lib.instance);
        Ok(Self {
            khr: surface_khr,
            loader: surface_loader,
        })
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        unsafe { self.loader.destroy_surface(self.khr, None) };
    }
}
