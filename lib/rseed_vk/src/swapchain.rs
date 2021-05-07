use ash::{
    self,
    extensions::khr,
    vk,
    version::DeviceV1_0,
    Instance,
};

use super::{
    device::Device,
    surface::Surface,
};

#[derive(Clone, Debug)]
pub enum SwapchainError {
    Creation(vk::Result),
    ImageCreation(vk::Result),
}

pub type Result<T> = std::result::Result<T, SwapchainError>;


pub struct Swapchain {
    pub loader : khr::Swapchain,
    pub khr : vk::SwapchainKHR,
    pub images : Vec<vk::Image>,
    pub views : Vec<vk::ImageView>,

}

impl Swapchain {

    pub fn init(
        instance : &Instance,
        device : &Device,
        surface : &Surface,
    ) -> Result<Self> {

        let surface_capabilities = unsafe {
            surface.loader
                .get_physical_device_surface_capabilities(device.physical, surface.khr)
                .map_err(|e| SwapchainError::Creation(e))?
        };

        let surface_formats = unsafe {
            surface.loader
                .get_physical_device_surface_formats(device.physical, surface.khr)
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
            .image_extent(surface_capabilities.current_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .queue_family_indices(&queue_fam)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO);

        let loader = khr::Swapchain::new(instance, &device.logical);
        let khr = unsafe {
            loader
                .create_swapchain(&swapchain_create_info, None)
                .map_err(|e| SwapchainError::Creation(e))?
        };

        let (images, views) = Self::create_images(&loader, &khr, device)?;
        Ok(Self {
            loader,
            khr,
            images,
            views,
        })
    }

    fn create_images(loader : &khr::Swapchain, khr : &vk::SwapchainKHR, device : &Device) -> Result<(Vec<vk::Image>, Vec<vk::ImageView>)> {
        let images = unsafe {
            loader.get_swapchain_images(*khr)
                .map_err(|e| SwapchainError::ImageCreation(e))?
        };
        let mut views = Vec::with_capacity(images.len());
        for img in &images {
            let subressource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);
            let view_create_info = vk::ImageViewCreateInfo::builder()
                .image(*img)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(vk::Format::B8G8R8A8_UNORM)
                .subresource_range(*subressource_range);
            let view = unsafe {
                device.logical
                    .create_image_view(&view_create_info, None)
                    .map_err(|e|  SwapchainError::ImageCreation(e))?
                };
            views.push(view);
        } 
        Ok((images, views))
    }

}