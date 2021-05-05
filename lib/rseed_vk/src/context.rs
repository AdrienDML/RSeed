use ash::{
    self,
    vk,
    extensions::khr,
    version::{DeviceV1_0, EntryV1_0, InstanceV1_0},
    Entry, Instance,
};

use super::{
    window::{self, HasRawWindowHandle},
    debug::DebugMessenger,
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
    SurfaceCreation(window::WindowError),
    PhysicalDeviceCreation(vk::Result),
    LogicalDeviceCreation(vk::Result),
    SwapchainCreation(vk::Result),
}

pub struct VkContext {
    pub entry: Entry,
    pub instance: Instance,

    pub physical_device: vk::PhysicalDevice,
    pub logical_device: ash::Device,
    pub graphic_queue: vk::Queue,
    pub transfert_queue: vk::Queue,
    pub compute_queue: vk::Queue,

    pub swapchain_loader: khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,

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

        //instance creation
        let instance = Self::create_instance(&entry, window_handle, app_name, app_version)?;

        // Surface creation
        let surface =
            window::create_surface(&entry, &instance, window_handle).map_err(|e| match e {
                window::WindowError::ExtensionNotPresent(_) => ContextError::Extention(e),
                window::WindowError::SurfaceCreationFailed(_) => ContextError::SurfaceCreation(e),
            })?;
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        // Device creation
        let (physical_device, device_prop) = Self::chose_device(&instance)?;
        let (logical_device, graphic_queue, transfert_queue, compute_queue, qfam_inds) =
            Self::create_logical_device(&instance, &physical_device, &surface_loader, surface)?;

        let (swapchain_loader, swapchain) = Self::create_swapchain(
            &instance,
            &physical_device,
            &logical_device,
            &surface_loader,
            &surface,
            &qfam_inds.0,
        )?;

        Ok(Self {
            entry,
            instance,
            physical_device,
            logical_device,
            graphic_queue,
            transfert_queue,
            compute_queue,
            swapchain_loader,
            swapchain,
            surface,
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
        let mut debugcreateinfo = DebugMessenger::create_debug_utils_messenger(0);
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
        physical_device: &vk::PhysicalDevice,
        surface_loader: &khr::Surface,
        surface: vk::SurfaceKHR,
    ) -> Result<(
        ash::Device,
        vk::Queue,
        vk::Queue,
        vk::Queue,
        (u32, u32, u32),
    )> {
        let layer_names: Vec<std::ffi::CString> = Self::query_layers()?;
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
                        surface_loader
                            .get_physical_device_surface_support(
                                *physical_device,
                                index as u32,
                                surface,
                            )
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

        let device_extension_name_ptr: Vec<*const i8> = vec![khr::Swapchain::name().as_ptr()];

        let device_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queues_info)
            .enabled_extension_names(&device_extension_name_ptr)
            .enabled_layer_names(&enabled_layer_ptr);
        let device = unsafe {
            instance
                .create_device(*physical_device, &device_info, None)
                .map_err(|e| ContextError::LogicalDeviceCreation(e))
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

    fn create_swapchain(
        instance: &Instance,
        physical_device: &vk::PhysicalDevice,
        logical_device: &ash::Device,
        surface_loader: &ash::extensions::khr::Surface,
        surface: &vk::SurfaceKHR,
        graphic_queue_indice: &u32,
    ) -> Result<(khr::Swapchain, vk::SwapchainKHR)> {
        let surface_capabilities = unsafe {
            surface_loader
                .get_physical_device_surface_capabilities(*physical_device, *surface)
                .map_err(|e| ContextError::SwapchainCreation(e))?
        };
        // let surface_present_mode = unsafe {
        //     surface_loader.get_physical_device_surface_present_modes(*physical_device, surface)
        //         .map_err(|e| ContextError::SwapchainCreation(e))?
        // };
        let surface_formats = unsafe {
            surface_loader
                .get_physical_device_surface_formats(*physical_device, *surface)
                .map_err(|e| ContextError::SwapchainCreation(e))?
        };

        let queue_fam = [*graphic_queue_indice];
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(*surface)
            .min_image_count(
                3.max(surface_capabilities.min_image_count)
                    .min(surface_capabilities.max_image_count),
            )
            .image_format(surface_formats.first().unwrap().format)
            .image_color_space(surface_formats.first().unwrap().color_space)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .queue_family_indices(&queue_fam)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO);

        let swapchain_loader = khr::Swapchain::new(instance, logical_device);
        let swapchain = unsafe {
            swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .map_err(|e| ContextError::SwapchainCreation(e))?
        };
        Ok((swapchain_loader, swapchain))
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
            self.logical_device.destroy_device(None);
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }
}
