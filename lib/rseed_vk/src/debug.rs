use ash::{self, vk, Entry, Instance};

pub enum DebugMessengerError {
    CreationError(vk::Result),
}

pub type Result<T> = std::result::Result<T, DebugMessengerError>;



pub unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    use rseed_core::time;
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

pub struct DebugMessenger {
    loader : ash::extensions::ext::DebugUtils,
    messenger : vk::DebugUtilsMessengerEXT,
}

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



impl DebugMessenger {

    fn init(entry : &Entry, instance : &Instance, log_level : usize) -> Result<Self> {
        let debug_create_info = Self::create_debug_utils_messenger(log_level);
        
        let loader = ash::extensions::ext::DebugUtils::new(entry, instance);
        let messenger = unsafe {
            loader
                .create_debug_utils_messenger(&debug_create_info, None)
                .map_err(|e| DebugMessengerError::CreationError(e))?
        };
        Ok(Self {
            loader,
            messenger
        })
    }

    pub fn create_debug_utils_messenger<'a>(log_level : usize) -> vk::DebugUtilsMessengerCreateInfoEXTBuilder<'a>
    {
        let message_severity = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
        | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
        | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR;
        let message_type = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION;
        
        vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(message_severity)
            .message_type(message_type)
            .pfn_user_callback(Some(vulkan_debug_utils_callback))
    }

}

