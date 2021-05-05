use ash::{
    self,
    extensions::khr,
    vk,
    Instance,
};

use super::{
    device::Device,
    surface::Surface,
};

#[derive(Clone, Debug)]
pub enum SwapchainError {
    Creation(vk::Result),
}

pub type Result<T> = std::result::Result<T, SwapchainError>;


pub struct Swapchain {
    pub loader : khr::Swapchain,
    pub khr : vk::SwapchainKHR,
}

impl Swapchain {

    pub fn init(
        instance : &Instance,
        device : &Device,
        surface : &Surface,
    ) -> Result<Self> {

        let surface_capabilities = unsafe {
            surface.loader
                .get_physical_device_surface_capabilities(device.physical_device, surface.khr)
                .map_err(|e| SwapchainError::Creation(e))?
        };

        let surface_formats = unsafe {
            surface.loader
                .get_physical_device_surface_formats(device.physical_device, surface.khr)
                .map_err(|e| SwapchainError::Creation(e))?
        };

        let queue_fam = [device.queue_inds.0];
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface.khr)
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

        let loader = khr::Swapchain::new(instance, &device.logical_device);
        let khr = unsafe {
            loader
                .create_swapchain(&swapchain_create_info, None)
                .map_err(|e| SwapchainError::Creation(e))?
        };
        Ok(Self {
            loader,
            khr,
        })
    }

}