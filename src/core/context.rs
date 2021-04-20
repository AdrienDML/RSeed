use ash::{Entry, Instance, version::{EntryV1_0, InstanceV1_0}, vk};
use consts::ENGINE_VERSION;
use rseed_log::time;
use super::{consts, utils::Version};

use raw_window_handle::{RawWindowHandle, HasRawWindowHandle};

#[derive(Clone, Debug)]
pub enum ContextError {
    LibLoadFail,
    NoInstance(ash::InstanceError),
    SurfaceCreation(vk::Result),
}

pub struct VkContext {
    instance: Instance,
}

impl VkContext {
    pub unsafe fn init(app_name : String, app_version : Version) -> Result<Self, ContextError> {
        let entry = Entry::new().map_err(|_| ContextError::LibLoadFail)?;
        
        let app_name = std::ffi::CString::new(app_name).unwrap();
        let engine_name = std::ffi::CString::new(consts::ENGINE_NAME).unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_version(vk::make_version(2, 0, 0))
            .engine_version(ENGINE_VERSION.into())
            .api_version(app_version.into())
            .application_name(&app_name)
            .engine_name(&engine_name);

        let layer_names:Vec<std::ffi::CString>  = Self::query_layers()?;
        let layer_pointer: Vec<*const i8> = layer_names.iter()
            .map(|l| l.as_ptr())
            .collect();

        let extension_names = Self::query_required_extentions()?;
        let extension_pointers : Vec<*const i8> = extension_names.iter()
            .map(|name| name.as_ptr())
            .collect();
        let mut debugcreateinfo = Self::create_debug_callback();
        let create_info = vk::InstanceCreateInfo::builder()
            .push_next(&mut debugcreateinfo)
            .application_info(&app_info)
            .enabled_extension_names(&extension_pointers)
            .enabled_layer_names(&layer_pointer);
        let instance = entry
            .create_instance(&create_info, None)
            .map_err(|e| ContextError::NoInstance(e))?;
        Ok(Self { instance })
    }

    fn query_layers() -> Result<Vec<std::ffi::CString>, ContextError> {
        let layers = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        Ok(layers)
    }

    fn query_required_extentions() -> Result<Vec<&'static std::ffi::CStr>, ContextError> {
        use ash::extensions as ext;
        let mut exts = Vec::new();
        exts.push(ext::ext::DebugUtils::name());
        exts.push(ext::khr::Surface::name());
        #[cfg(target_os = "windows")]
        exts.push(ext::khr::Win32Surface::name());
        
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        exts.push(ext::khr::XlibSurface::name());

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        exts.push(ext::khr::XcblibSurface::name());

        #[cfg(any(target_os = "android"))]
        exts.push(ext::khr::AndroidSurface::name());
            

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        exts.push(ext::ext::MetalSurface::name());
        return Ok(exts);
    }

    fn create_debug_callback<'a>() -> vk::DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        fn into_log_level(severity : vk::DebugUtilsMessageSeverityFlagsEXT) -> usize {
            if severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) { 3 }
            else if severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) { 2 }
            else if severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::INFO) { 1 }
            else {0}
        }
        unsafe extern "system" fn vulkan_debug_utils_callback(
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
            _p_user_data: *mut std::ffi::c_void,
        ) -> vk::Bool32 {
            let colorcodes= [
                        37, //WHITE   -> trace
                        32, //GREEN   -> info
                        33, //YELLOW  -> warn
                        31, //RED     -> error & fatal
                        0,  //RESET
                    ];
            let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
            let ty = format!("{:?}", message_type).to_lowercase();
            println!(
                "\x1B[{}m[{}] {}{:?}:{}> {:?} \x1B[{}m",
                colorcodes[into_log_level(message_severity)],
                time::get_time().unwrap(),
                "Vulkan",
                message_severity,
                ty,
                message,
                colorcodes[4]
            );
            vk::FALSE
        }
        
        vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            )
            .pfn_user_callback(Some(vulkan_debug_utils_callback))
    }

    unsafe fn create_surface<E, I>(entry : &E , instance : &I, window_handle : &dyn HasRawWindowHandle) -> Result<vk::SurfaceKHR, ContextError>
    where
        E : EntryV1_0,
        I : InstanceV1_0,
    {
        match window_handle.raw_window_handle()  {
            #[cfg(target_os = "windows")]
            RawWindowHandle::Windows(handle) => {
                let surface_desc = vk::Win32SurfaceCreateInfoKHR::builder()
                    .hinstance(handle.hinstance)
                    .hwnd(handle.hwnd);
                let surface_fn = ash::extensions::khr::Win32Surface::new(entry, instance);
                surface_fn.create_win32_surface(&surface_desc, None).map_err(|e| ContextError::SurfaceCreation(e))
            }
    
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawWindowHandle::Wayland(handle) => {
                let surface_desc = vk::WaylandSurfaceCreateInfoKHR::builder()
                    .display(handle.display)
                    .surface(handle.surface);
                let surface_fn = ash::extensions::khr::WaylandSurface::new(entry, instance);
                surface_fn.create_wayland_surface(&surface_desc, allocation_callbacks)
            }
    
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawWindowHandle::Xlib(handle) => {
                let surface_desc = vk::XlibSurfaceCreateInfoKHR::builder()
                    .dpy(handle.display as *mut _)
                    .window(handle.window);
                let surface_fn = ash::extensions::khr::XlibSurface::new(entry, instance);
                surface_fn.create_xlib_surface(&surface_desc, allocation_callbacks)
            }
    
            #[cfg(any(
                target_os = "linux",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "netbsd",
                target_os = "openbsd"
            ))]
            RawWindowHandle::Xcb(handle) => {
                let surface_desc = vk::XcbSurfaceCreateInfoKHR::builder()
                    .connection(handle.connection as *mut _)
                    .window(handle.window);
                let surface_fn = ash::extensions::khr::XcbSurface::new(entry, instance);
                surface_fn.create_xcb_surface(&surface_desc, allocation_callbacks)
            }
    
            #[cfg(any(target_os = "android"))]
            RawWindowHandle::Android(handle) => {
                let surface_desc =
                    vk::AndroidSurfaceCreateInfoKHR::builder().window(handle.a_native_window as _);
                let surface_fn = ash::extensions::khr::AndroidSurface::new(entry, instance);
                surface_fn.create_android_surface(&surface_desc, allocation_callbacks)
            }
    
            #[cfg(any(target_os = "macos"))]
            RawWindowHandle::MacOS(handle) => {
                use raw_window_metal::{macos, Layer};
    
                let layer = match macos::metal_layer_from_handle(handle) {
                    Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                    Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
                };
    
                let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
                let surface_fn = ash::extensions::ext::MetalSurface::new(entry, instance);
                surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
            }
    
            #[cfg(any(target_os = "ios"))]
            RawWindowHandle::IOS(handle) => {
                use raw_window_metal::{ios, Layer};
    
                let layer = match ios::metal_layer_from_handle(handle) {
                    Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                    Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
                };
    
                let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
                let surface_fn = ash::extensions::ext::MetalSurface::new(entry, instance);
                surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
            }
    
            _ => Err(ContextError::SurfaceCreation(vk::Result::ERROR_EXTENSION_NOT_PRESENT)),
        }
    }
}

impl Drop for VkContext {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}