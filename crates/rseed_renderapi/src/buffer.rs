use rseed_math::Vec3D;

pub trait VertexArrayT {
    fn bind();

    fn unbind();
}

pub trait VertexBufferT {
    fn set_data(&self, data: &[Vec3D]);

    fn bind();

    fn unbind();
}

pub trait IndexBufferT {
    fn set_data(&self, data: &[usize]);

    fn bind();

    fn unbind();
}
