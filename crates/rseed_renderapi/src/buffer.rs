use rseed_math::Vec3D;

pub trait VertexArrayT {

}


pub trait VertexBufferT {

    fn set_data(&self, data : &[Vec3D]);

    fn bind();
}

pub trait IndexBufferT {
    fn set_data(&self, data : &[usize]);
}