use vulkano as vk;
use vulkano_win;
#[derive(Clone, Debug)]
pub enum ContextError {
    NoInstance(vk::instance::InstanceCreationError),
    NoPhysicalDevice,
    PhysicalDeviceCreation(vk::device::DeviceCreationError)
}

pub struct VkContext {
    pub instance : std::sync::Arc<vk::instance::Instance>,
    pub device : std::sync::Arc<vk::device::Device>,
    pub queues : vk::device::QueuesIter, 
}

impl VkContext {
    pub fn init() -> Result<Self, ContextError> {
        let instance = match vk::instance::Instance::new(None, &vulkano_win::required_extensions(), None) {
            Ok(i) => i,
            Err(err) => return Err(ContextError::NoInstance(err)),
        };
        let mut phys_devs = vk::instance::PhysicalDevice::enumerate(&instance);

        // TODO : better choice of the device.
        let chosen_device = match phys_devs.next() {
            Some(dev) => dev,
            None => return Err(ContextError::NoPhysicalDevice),
        };
        
        // TODO : Get a good choice of queues
        let (device, queues) = {
            let queue_family = chosen_device.queue_families().next().unwrap();
            let feature = vk::device::Features::none();
            let ext = vk::device::DeviceExtensions::none();
            match vk::device::Device::new(chosen_device, &feature, &ext, Some((queue_family, 1.0))) {
                Ok(d) => d,
                Err(err) => return Err(ContextError::PhysicalDeviceCreation(err)),
            }
        };
        Ok( Self {
                instance : instance.clone(),
                device,
                queues,
            }
        )
    }
}