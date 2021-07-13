use rseed_core::prelude::*;

pub mod debug;
pub mod device;
pub mod library;
pub mod pipeline;
pub mod surface;
pub mod swapchain;
pub mod window;

use self::{
    device::{Device, DeviceError},
    library::*,
    surface::{Surface, SurfaceError},
    swapchain::{Swapchain, SwapchainError},
    window::HasRawWindowHandle,
};

use rseed_core::{
    utils::Version,
};

#[derive(Debug, Error)]
pub enum ContextError {
    #[error(display = "There was an error while initiating vulkan : {:?}.", _0)]
    Library(LibraryError),
    #[error(display = "There was an error during surface creation : {:?}.", _0)]
    Surface(SurfaceError),
    #[error(display = "There was an error during device creation : {:?}.", _0)]
    Device(DeviceError),
    #[error(display = "There was an error during swapchain creation : {:?}.", _0)]
    Swapchain(SwapchainError),
}

pub type Result<T> = std::result::Result<T, ContextError>;

pub struct VkContext {
    library: Library,
    surface: Surface,
    device: Device,
    swapchain: Swapchain,
}

impl VkContext {
    pub unsafe fn init(
        app_name: String,
        app_version: Version,
        window_handle: &dyn HasRawWindowHandle,
    ) -> Result<Self> {
        let library = Library::init(app_name, app_version, window_handle)
            .map_err(|e| ContextError::Library(e))?;

        let surface =
            Surface::init(&library, window_handle).map_err(|e| ContextError::Surface(e))?;

        // Device creation
        let device = Device::init(&library, &surface).map_err(|e| ContextError::Device(e))?;
        // Swapchain
        let swapchain =
            Swapchain::init(&library, &device, &surface).map_err(|e| ContextError::Swapchain(e))?;

        Ok(Self {
            library,
            surface,
            device,
            swapchain,
        })
    }
}

impl Drop for VkContext {
    fn drop(&mut self) {
        self.swapchain.drop(&self.device);
    }
}
