pub mod mat3;
pub mod quat;
pub mod vectors;
//pub mod noise; // TODO make it work with f32
pub mod fct;

// exposing structs and fct
pub use mat3::Mat3;
pub use quat::Quat;
pub use vectors::Vec2D;
pub use vectors::Vec3D;
pub use vectors::Vec4D;
