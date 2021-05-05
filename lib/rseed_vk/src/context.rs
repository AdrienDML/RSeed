use ash::{
    self,
    vk,
    version::{EntryV1_0, InstanceV1_0},
    Entry, Instance,
};

use super::{
    window::{self, HasRawWindowHandle},
    debug::DebugMessenger,
    surface::{Surface, SurfaceError},
    device::{Device, DeviceError},
    swapchain::{Swapchain, SwapchainError},
};

use rseed_core::{
    consts::{
        ENGINE_VERSION,
        ENGINE_NAME,
    },
    utils::Version,
};

#[derive(Clone, Debug)]
pub enum ContextError {
    LibLoadFail,
    NoInstance(ash::InstanceError),
    Extention(window::WindowError),
    Surface(SurfaceError),
    Device(DeviceError),
    Swapchain(SwapchainError),
}

pub type Result<T> = std::result::Result<T, ContextError>;

pub struct VkContext {
    pub entry: Entry,
    pub instance: Instance,
    pub surface: Surface,
    pub device: Device,
    pub swapchain : Swapchain,
}


impl VkContext {
    pub unsafe fn init(
        app_name: String,
        app_version: Version,
        window_handle: &dyn HasRawWindowHandle,
    ) -> Result<Self> {
        let entry = Entry::new().map_err(|_| ContextError::LibLoadFail)?;
        let layer_names = Self::query_layers()?;
        //instance creation
        let instance = Self::create_instance(&entry, window_handle, app_name, app_version)?;

        let surface = Surface::init(&entry, &instance, window_handle)
            .map_err(|e| ContextError::Surface(e))?;

        // Device creation
        let device = Device::init(&instance, &surface, &layer_names).map_err(|e| ContextError::Device(e))?;

        let swapchain = Swapchain::init(&instance, &device, &surface).map_err(|e| ContextError::Swapchain(e))?;

        Ok(Self {
            entry,
            instance,
            surface,
            device,
            swapchain,
        })
    }

    fn create_instance(
        entry: &Entry,
        window_handle: &dyn HasRawWindowHandle,
        app_name: String,
        app_version: Version,
    ) -> Result<Instance> {
        // Instance creation
        let app_name = std::ffi::CString::new(app_name).unwrap();
        let engine_name = std::ffi::CString::new(ENGINE_NAME).unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_version(vk::make_version(2, 0, 0))
            .engine_version(ENGINE_VERSION.into())
            .api_version(app_version.into())
            .application_name(&app_name)
            .engine_name(&engine_name);

        let layer_names: Vec<std::ffi::CString> = Self::query_layers()?;
        let layer_pointer: Vec<*const i8> = layer_names.iter().map(|l| l.as_ptr()).collect();

        let extension_names = unsafe { window::query_surface_required_extentions(window_handle) }
            .map_err(|e| ContextError::Extention(e))?;
        let extension_pointers: Vec<*const i8> =
            extension_names.iter().map(|name| name.as_ptr()).collect();
        // Adding Debug call back
        let mut debugcreateinfo = DebugMessenger::create_debug_utils_messenger(rseed_log::LogLevel::INFO);
        let create_info = vk::InstanceCreateInfo::builder()
            .push_next(&mut debugcreateinfo)
            .application_info(&app_info)
            .enabled_extension_names(&extension_pointers)
            .enabled_layer_names(&layer_pointer);

        unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(|e| ContextError::NoInstance(e))
        }
    }

    fn query_layers() -> Result<Vec<std::ffi::CString>> {
        let layers = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        Ok(layers)
    }

    
}

impl Drop for VkContext {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
