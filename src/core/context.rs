use ash::{
    self,
    version::{DeviceV1_0, EntryV1_0, InstanceV1_0},
    vk, Entry, Instance,
};

use super::{
    consts,
    utils::Version,
    window::{self, HasRawWindowHandle},
};
use consts::ENGINE_VERSION;

#[derive(Clone, Debug)]
pub enum ContextError {
    LibLoadFail,
    NoInstance(ash::InstanceError),
    Extention(window::WindowError),
    SurfaceCreation(window::WindowError),
    PhysicalDeviceCreation(vk::Result),
    LogicalDeviceCreation(vk::Result),
}

pub struct VkContext {
    pub entry: Entry,
    pub instance: Instance,

    pub phys_device: vk::PhysicalDevice,
    pub log_device: ash::Device,
    pub graphic_queue: vk::Queue,
    pub transfert_queue: vk::Queue,
    pub compute_queue: vk::Queue,
    pub surface: vk::SurfaceKHR,
}

pub type Result<T> = std::result::Result<T, ContextError>;

impl VkContext {
    pub unsafe fn init(
        app_name: String,
        app_version: Version,
        window_handle: &dyn HasRawWindowHandle,
    ) -> Result<Self> {
        let entry = Entry::new().map_err(|_| ContextError::LibLoadFail)?;

        // Instance creation
        let app_name = std::ffi::CString::new(app_name).unwrap();
        let engine_name = std::ffi::CString::new(consts::ENGINE_NAME).unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_version(vk::make_version(2, 0, 0))
            .engine_version(ENGINE_VERSION.into())
            .api_version(app_version.into())
            .application_name(&app_name)
            .engine_name(&engine_name);

        let layer_names: Vec<std::ffi::CString> = Self::query_layers()?;
        let layer_pointer: Vec<*const i8> = layer_names.iter().map(|l| l.as_ptr()).collect();

        let extension_names = window::query_surface_required_extentions(window_handle)
            .map_err(|e| ContextError::Extention(e))?;
        let extension_pointers: Vec<*const i8> =
            extension_names.iter().map(|name| name.as_ptr()).collect();
        // Adding Debug call back
        let mut debugcreateinfo = Self::create_debug_callback();
        let create_info = vk::InstanceCreateInfo::builder()
            .push_next(&mut debugcreateinfo)
            .application_info(&app_info)
            .enabled_extension_names(&extension_pointers)
            .enabled_layer_names(&layer_pointer);
        let instance = entry
            .create_instance(&create_info, None)
            .map_err(|e| ContextError::NoInstance(e))?;

        // Surface creation
        let surface =
            window::create_surface(&entry, &instance, window_handle).map_err(|e| match e {
                window::WindowError::ExtensionNotPresent(_) => ContextError::Extention(e),
                window::WindowError::SurfaceCreationFailed(_) => ContextError::SurfaceCreation(e),
            })?;
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
        
        
        // Device creation
        let (phys_device, device_prop) = Self::chose_device(&instance)?;
        let (log_device, graphic_queue, transfert_queue, compute_queue) =
            Self::create_logical_device(&instance, &phys_device, &layer_pointer, surface_loader, surface)?;

        Ok(Self {
            entry,
            instance,
            phys_device,
            log_device,
            graphic_queue,
            transfert_queue,
            compute_queue,
            surface,
        })
    }

    fn chose_device(
        instance: &ash::Instance,
    ) -> Result<(vk::PhysicalDevice, vk::PhysicalDeviceProperties)> {
        let phys_devs = unsafe {
            instance
                .enumerate_physical_devices()
                .map_err(|e| ContextError::PhysicalDeviceCreation(e))?
        };

        let mut chosen = None;
        for p in phys_devs {
            let props = unsafe { instance.get_physical_device_properties(p) };
            if props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                let device_name = unsafe {
                    std::ffi::CStr::from_ptr(props.device_name.as_ptr())
                        .to_str()
                        .unwrap()
                };
                println!("{}", device_name);
                chosen = Some((p, props));
            } else if props.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU {
                match chosen {
                    Some((_, prop)) => {
                        if prop.device_type != vk::PhysicalDeviceType::DISCRETE_GPU {
                            chosen = Some((p, props))
                        }
                    }
                    None => chosen = Some((p, props)),
                }
            }
        }
        chosen.ok_or(ContextError::PhysicalDeviceCreation(
            vk::Result::ERROR_INITIALIZATION_FAILED,
        ))
    }

    fn create_logical_device(
        instance: &ash::Instance,
        phys_device: &vk::PhysicalDevice,
        enabled_layer_ptr: &Vec<*const i8>,
        surface_loader : ash ::extensions::khr::Surface,
        surface : vk::SurfaceKHR,
    ) -> Result<(ash::Device, vk::Queue, vk::Queue, vk::Queue)> {
        let queue_fam_props =
            unsafe { instance.get_physical_device_queue_family_properties(*phys_device) };
        let qfam_inds = {
            let mut g_q = None;
            let mut t_q = None;
            let mut c_q = None;
            for (index, qfam) in queue_fam_props.iter().enumerate() {
                if qfam.queue_count > 0 && qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS) 
                    && unsafe{
                        surface_loader
                            .get_physical_device_surface_support(*phys_device, index as u32, surface)
                            .map_err(|e| ContextError::LogicalDeviceCreation(e))?
                    }
                {
                    g_q = Some(index as u32);
                }
                if qfam.queue_count > 0 && qfam.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                    if t_q.is_none()
                        && !(qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                            && qfam.queue_flags.contains(vk::QueueFlags::COMPUTE))
                    {
                        t_q = Some(index as u32);
                    }
                }
                if qfam.queue_count > 0 && qfam.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                    if c_q.is_none()
                        && !(qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                            && qfam.queue_flags.contains(vk::QueueFlags::TRANSFER))
                    {
                        c_q = Some(index as u32);
                    }
                }
            }
            (g_q.unwrap(), t_q.unwrap(), c_q.unwrap())
        };

        let priorities = [1.0f32];
        let queues_info = [
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(qfam_inds.0)
                .queue_priorities(&priorities)
                .build(),
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(qfam_inds.1)
                .queue_priorities(&priorities)
                .build(),
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(qfam_inds.2)
                .queue_priorities(&priorities)
                .build(),
        ];
        let device_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queues_info)
            .enabled_layer_names(enabled_layer_ptr);
        let device = unsafe {
            instance
                .create_device(*phys_device, &device_info, None)
                .map_err(|e| ContextError::LogicalDeviceCreation(e))
        }?;
        let graphic_queue = unsafe { device.get_device_queue(qfam_inds.0, 0) };
        let transfert_queue = unsafe { device.get_device_queue(qfam_inds.1, 0) };
        let compute_queue = unsafe { device.get_device_queue(qfam_inds.2, 0) };
        Ok((device, graphic_queue, transfert_queue, compute_queue))
    }

    fn query_layers() -> Result<Vec<std::ffi::CString>> {
        let layers = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
        Ok(layers)
    }

    fn create_debug_callback<'a>() -> vk::DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        use rseed_log::time;
        fn into_log_level(severity: vk::DebugUtilsMessageSeverityFlagsEXT) -> usize {
            if severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR) {
                3
            } else if severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING) {
                2
            } else if severity.intersects(vk::DebugUtilsMessageSeverityFlagsEXT::INFO) {
                1
            } else {
                0
            }
        }
        unsafe extern "system" fn vulkan_debug_utils_callback(
            message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
            message_type: vk::DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
            _p_user_data: *mut std::ffi::c_void,
        ) -> vk::Bool32 {
            let colorcodes = [
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
            self.log_device.destroy_device(None);
        }
    }
}
