use ash::{Entry, Instance, version::{EntryV1_0, InstanceV1_0}, vk};

use rseed_log::time;

use consts::ENGINE_VERSION;
use super::{
    consts,
    utils::Version,
    window::{
        self,
        HasRawWindowHandle,
    }
};


#[derive(Clone, Debug)]
pub enum ContextError {
    LibLoadFail,
    NoInstance(ash::InstanceError),
    Extention(window::Error),
    SurfaceCreation(window::Error),
    Physical(vk::Result),
}

pub struct VkContext {
    entry : Entry,
    instance: Instance,

}

impl VkContext {
    pub unsafe fn init(app_name : String, app_version : Version, window_handle: &dyn HasRawWindowHandle) -> Result<Self, ContextError> {
        let entry = Entry::new().map_err(|_| ContextError::LibLoadFail)?;
        
        let app_name = std::ffi::CString::new(app_name).unwrap();
        let engine_name = std::ffi::CString::new(consts::ENGINE_NAME).unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_version(vk::make_version(2, 0, 0))
            .engine_version(ENGINE_VERSION.into())
            .api_version(app_version.into())
            .application_name(&app_name)
            .engine_name(&engine_name);

        let layer_names : Vec<std::ffi::CString>  = Self::query_layers()?;
        let layer_pointer : Vec<*const i8> = layer_names.iter()
            .map(|l| l.as_ptr())
            .collect();

        let extension_names = window::query_surface_required_extentions(window_handle)
            .map_err(|e | ContextError::Extention(e))?;
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
        

        let surface = window::create_surface(&entry, &instance, window_handle).map_err(|e|
                match e {
                    window::Error::ExtensionNotPresent(_) => ContextError::Extention(e),
                    window::Error::SurfaceCreationFailed(_) => ContextError::SurfaceCreation(e),
                }
            )?;
        
        let (device, device_prop) = Self::chose_device(&instance)?;
        let device_name = std::ffi::CStr::from_ptr(device_prop.device_name.as_ptr()).to_str().unwrap();
        println!("{}", device_name);
        Ok(Self { 
            entry,
            instance,
        })
    }

    fn chose_device(instance : &ash::Instance) 
        -> Result<(vk::PhysicalDevice, vk::PhysicalDeviceProperties), ContextError>
    {
        let phys_devs = unsafe {
            instance.enumerate_physical_devices()
                .map_err(|e| ContextError::Physical(e))?
        };
    
        let mut chosen = None;
        for p in phys_devs {
            let props = unsafe{instance.get_physical_device_properties(p)};
            if props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                let device_name = unsafe {std::ffi::CStr::from_ptr(props.device_name.as_ptr()).to_str().unwrap()};
                println!("{}", device_name);
                chosen = Some((p, props));
            }
            else if props.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU {
                match chosen {
                    Some((_, prop)) => if prop.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
                        chosen = Some((p, props))
                    },
                    None => chosen = Some((p, props)),
                }
            }
        }
        chosen.ok_or(ContextError::Physical(vk::Result::ERROR_INITIALIZATION_FAILED))
    }

    fn create_logical_device(instance : &ash::Instance, device : &vk::PhysicalDevice) {
        
    }

    fn query_layers() -> Result<Vec<std::ffi::CString>, ContextError> {
        let layers = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        Ok(layers)
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

}

impl Drop for VkContext {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}