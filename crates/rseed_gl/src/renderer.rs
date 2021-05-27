use super::Camera;
use super::{VArray, IndexBuffer, ShaderProgram};
use gl::typed::*;

struct Renderer {

}

impl Renderer {
    pub fn init(gl : &gl::Gl) {
        gl.Enable(gl::BLEND)
    }
}