use ash::{
    self,
    vk,
    extensions::khr,
    version::{InstanceV1_0, DeviceV1_0},
    Instance,
};

use super::surface::Surface;

#[derive(Clone, Debug)]
pub enum DeviceError {
    PhysicalCreation(vk::Result),
    LogicalCreation(vk::Result),
}

pub type Result<T> = std::result::Result<T, DeviceError>;

pub struct Device {
    pub physical_device : vk::PhysicalDevice,
    pub logical_device : ash::Device,
    pub graphic_queue: vk::Queue,
    pub transfert_queue: vk::Queue,
    pub compute_queue: vk::Queue,
    pub queue_inds: (u32, u32, u32),
}

impl Device {

    pub fn init(
        instance : &Instance,
        surface : &Surface,
        layer_names : &Vec<std::ffi::CString>,
    ) -> Result<Self> {
        let (physical_device, _device_prop) = Self::chose_device(&instance)?;
        let (logical_device, graphic_queue, transfert_queue, compute_queue, queue_inds) =
            Self::create_logical_device(&instance, &physical_device, surface, layer_names)?;

        Ok(Self{
            physical_device,
            logical_device,
            graphic_queue, 
            transfert_queue,
            compute_queue,
            queue_inds,
        })
    }

    fn chose_device(
        instance: &ash::Instance,
    ) -> Result<(vk::PhysicalDevice, vk::PhysicalDeviceProperties)> {
        let phys_devs = unsafe {
            instance
                .enumerate_physical_devices()
                .map_err(|e| DeviceError::PhysicalCreation(e))?
        };

        let mut chosen = None;
        for p in phys_devs {
            let props = unsafe { instance.get_physical_device_properties(p) };
            if props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
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
        chosen.ok_or(DeviceError::PhysicalCreation(
            vk::Result::ERROR_INITIALIZATION_FAILED,
        ))
    }

    fn create_logical_device(
        instance: &ash::Instance,
        physical_device: &vk::PhysicalDevice,
        surface : &Surface,
        layer_names : &Vec<std::ffi::CString>
    ) -> Result<(
        ash::Device,
        vk::Queue,
        vk::Queue,
        vk::Queue,
        (u32, u32, u32),
    )> {
        let enabled_layer_ptr: Vec<*const i8> = layer_names.iter().map(|l| l.as_ptr()).collect();
        let queue_fam_props =
            unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };
        let qfam_inds = {
            let mut g_q = None;
            let mut t_q = None;
            let mut c_q = None;
            for (index, qfam) in queue_fam_props.iter().enumerate() {
                if qfam.queue_count > 0
                    && qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                    && unsafe {
                        surface.loader
                            .get_physical_device_surface_support(
                                *physical_device,
                                index as u32,
                                surface.khr,
                            )
                            .map_err(|e| DeviceError::LogicalCreation(e))?
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

        let device_extension_name_ptr: Vec<*const i8> = vec![khr::Swapchain::name().as_ptr()];

        let device_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queues_info)
            .enabled_extension_names(&device_extension_name_ptr)
            .enabled_layer_names(&enabled_layer_ptr);
        let device = unsafe {
            instance
                .create_device(*physical_device, &device_info, None)
                .map_err(|e| DeviceError::LogicalCreation(e))
        }?;
        let graphic_queue = unsafe { device.get_device_queue(qfam_inds.0, 0) };
        let transfert_queue = unsafe { device.get_device_queue(qfam_inds.1, 0) };
        let compute_queue = unsafe { device.get_device_queue(qfam_inds.2, 0) };
        Ok((
            device,
            graphic_queue,
            transfert_queue,
            compute_queue,
            qfam_inds,
        ))
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {self.logical_device.destroy_device(None)};
    }
}


