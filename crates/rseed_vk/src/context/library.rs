use rseed_core::prelude::*;

use ash::{self, vk, Entry, Instance};

pub(crate) use ash::version::{EntryV1_0, InstanceV1_0};

use super::debug::*;
use super::window::{self, HasRawWindowHandle};

use rseed_core::{
    consts::{ENGINE_NAME, ENGINE_VERSION},
    utils::Version,
};

#[derive(Debug, Error)]
pub enum LibraryError {
    #[error(display = "Error loading vulkan {:?}", _0)]
    LibLoadFail(#[error(source)] ash::LoadingError),
    #[error(display = "Error creating vulkan instance {:?}", _0)]
    NoInstance(#[error(source)] ash::InstanceError),
    #[error(display = "Error loading vulkan extension: {:?}", _0)]
    Extention(#[error(source)] window::WindowError),
}

pub(crate) type Result<T> = std::result::Result<T, LibraryError>;

pub(crate) struct Library {
    pub(crate) entry: Entry,
    pub(crate) instance: Instance,
    pub(crate) enabled_layers: Vec<std::ffi::CString>,
}

impl Library {
    pub(crate) fn init(
        app_name: String,
        app_version: Version,
        window_handle: &dyn HasRawWindowHandle,
    ) -> Result<Self> {
        let entry = unsafe { Entry::new().map_err(|e| LibraryError::LibLoadFail(e))? };
        let layer_names = Self::query_layers()?;
        //instance creation
        let instance = Self::create_instance(&entry, app_name, app_version, window_handle)?;
        Ok(Self {
            entry,
            instance,
            enabled_layers: layer_names,
        })
    }

    fn create_instance(
        entry: &Entry,
        app_name: String,
        app_version: Version,
        window_handle: &dyn HasRawWindowHandle,
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
            .map_err(|e| LibraryError::Extention(e))?;
        let extension_pointers: Vec<*const i8> =
            extension_names.iter().map(|name| name.as_ptr()).collect();
        // Adding Debug call back
        let mut debugcreateinfo =
            DebugMessenger::create_debug_utils_messenger(rseed_log::LogLevel::WARN);
        let create_info = vk::InstanceCreateInfo::builder()
            .push_next(&mut debugcreateinfo)
            .application_info(&app_info)
            .enabled_extension_names(&extension_pointers)
            .enabled_layer_names(&layer_pointer);

        unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(|e| LibraryError::NoInstance(e))
        }
    }

    fn query_layers() -> Result<Vec<std::ffi::CString>> {
        let layers = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        Ok(layers)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
