#[cfg(all(feature = "vk", feature = "gl"))]
compile_error!("The vk and gl features cannot be enabled at the same time");

mod app;
pub use app::*;
