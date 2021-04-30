use ash::{
    vk,
    extensions::khr,
    version::{EntryV1_0, InstanceV1_0},
};
pub use raw_window_handle::{RawWindowHandle, HasRawWindowHandle};

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::extensions::ext; // portability extensions

#[derive(Clone, Debug)]
pub enum Error {
    ExtensionNotPresent(vk::Result),
    SurfaceCreationFailed(vk::Result),
}



/// Returns all the vulkan extension to load for each platform
pub unsafe fn query_surface_required_extentions(
    window_handle: &dyn HasRawWindowHandle,
) -> Result<Vec<&'static std::ffi::CStr>, Error> {
    let extensions = match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(_) => vec![khr::Surface::name(), khr::Win32Surface::name()],

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => vec![khr::Surface::name(), khr::WaylandSurface::name()],

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(_) => vec![khr::Surface::name(), khr::XlibSurface::name()],

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(_) => vec![khr::Surface::name(), khr::XcbSurface::name()],

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(_) => vec![khr::Surface::name(), khr::AndroidSurface::name()],

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(_) => vec![khr::Surface::name(), ext::MetalSurface::name()],

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(_) => vec![khr::Surface::name(), ext::MetalSurface::name()],

        _ => return Err(Error::ExtensionNotPresent(vk::Result::ERROR_EXTENSION_NOT_PRESENT)),
    };

    Ok(extensions)
}

pub unsafe fn create_surface<E, I>(
    entry: &E,
    instance: &I,
    window_handle: &dyn HasRawWindowHandle,
) -> Result<vk::SurfaceKHR, Error>
where
    E: EntryV1_0,
    I: InstanceV1_0,
{
    match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(handle.hinstance)
                .hwnd(handle.hwnd);
            let surface_fn = ash::extensions::khr::Win32Surface::new(entry, instance);
            surface_fn
                .create_win32_surface(&surface_desc, None)
                .map_err(|e| Error::SurfaceCreationFailed(e))
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(handle) => {
            let surface_desc = vk::WaylandSurfaceCreateInfoKHR::builder()
                .display(handle.display)
                .surface(handle.surface);
            let surface_fn = ash::extensions::khr::WaylandSurface::new(entry, instance);
            surface_fn
                .create_wayland_surface(&surface_desc, allocation_callbacks)
                .map_err(|e| Error::SurfaceCreationFailed(e))
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(handle) => {
            let surface_desc = vk::XlibSurfaceCreateInfoKHR::builder()
                .dpy(handle.display as *mut _)
                .window(handle.window);
            let surface_fn = ash::extensions::khr::XlibSurface::new(entry, instance);
            surface_fn
                .create_xlib_surface(&surface_desc, allocation_callbacks)
                .map_err(|e| Error::SurfaceCreationFailed(e))

        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(handle) => {
            let surface_desc = vk::XcbSurfaceCreateInfoKHR::builder()
                .connection(handle.connection as *mut _)
                .window(handle.window);
            let surface_fn = ash::extensions::khr::XcbSurface::new(entry, instance);
            surface_fn
                .create_xcb_surface(&surface_desc, allocation_callbacks)
                .map_err(|e| Error::SurfaceCreationFailed(e))
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            let surface_desc =
                vk::AndroidSurfaceCreateInfoKHR::builder().window(handle.a_native_window as _);
            let surface_fn = ash::extensions::khr::AndroidSurface::new(entry, instance);
            surface_fn
            .create_android_surface(&surface_desc, allocation_callbacks)
            .map_err(|e| Error::SurfaceCreationFailed(e))
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(handle) => {
            use raw_window_metal::{macos, Layer};

            let layer = match macos::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
            let surface_fn = ash::extensions::ext::MetalSurface::new(entry, instance);
            surface_fn
                .create_metal_surface(&surface_desc, allocation_callbacks)
                .map_err(|e| Error::SurfaceCreationFailed(e))
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(handle) => {
            use raw_window_metal::{ios, Layer};

            let layer = match ios::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
            let surface_fn = ash::extensions::ext::MetalSurface::new(entry, instance);
            surface_fn
                .create_metal_surface(&surface_desc, allocation_callbacks)
                .map_err(|e| Error::SurfaceCreationFailed(e))
        }

        _ => Err(Error::ExtensionNotPresent(
            vk::Result::ERROR_EXTENSION_NOT_PRESENT,
        )),
    }
}
