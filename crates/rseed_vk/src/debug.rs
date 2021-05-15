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
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message)
        .to_str()
        .expect("Vulkan Debug call back message tranlation failed!");
    let ty = format!("{:?}", message_type).to_lowercase();
    println!(
        "\x1B[{}m[{}] {} {:?}:{}> {} \x1B[{}m",
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
    pub loader: ash::extensions::ext::DebugUtils,
    pub messenger: vk::DebugUtilsMessengerEXT,
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
    pub fn init(entry: &Entry, instance: &Instance, _log_level: impl Into<usize>) -> Result<Self> {
        let debug_create_info = Self::create_debug_utils_messenger(_log_level);

        let loader = ash::extensions::ext::DebugUtils::new(entry, instance);
        let messenger = unsafe {
            loader
                .create_debug_utils_messenger(&debug_create_info, None)
                .map_err(|e| DebugMessengerError::CreationError(e))?
        };
        Ok(Self { loader, messenger })
    }

    pub fn create_debug_utils_messenger<'a>(
        _log_level: impl Into<usize>,
    ) -> vk::DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
        let mut message_severity = vk::DebugUtilsMessageSeverityFlagsEXT::ERROR;
        let log_level: usize = _log_level.into();
        if log_level < 3usize {
            message_severity |= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING;
        }
        if log_level < 2usize {
            message_severity |= vk::DebugUtilsMessageSeverityFlagsEXT::INFO;
        }
        if log_level < 1usize {
            message_severity |= vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE;
        }

        let message_type = vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION;

        vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(message_severity)
            .message_type(message_type)
            .pfn_user_callback(Some(vulkan_debug_utils_callback))
    }
}
