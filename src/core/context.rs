use ash::{vk, Entry, version::EntryV1_0};
#[derive(Clone, Debug)]
pub enum ContextError {
    NoInstance(vk::instance::InstanceCreationError),
    NoPhysicalDevice,
    PhysicalDeviceCreation(vk::device::DeviceCreationError)
}

pub struct VkContext {

}

impl VkContext {
    pub fn init() {
        let entry = unsafe {Entry::new().expect("Library loading failed!")};
        let app_info = vk::ApplicationInfo::builder()
            .application_version(vk::make_version(2,0,0))
            .engine_version(vk::make_version(0, 0, 1))
            .api_version(vk::make_version(1, 0, 106))
            .application_name(&std::ffi::CString::new("Unknown app.").unwrap())
            .engine_name(&std::ffi::CString::new("rseed.").unwrap());
    }
}