use rseed_math::Vec3D;

pub trait VertexArrayT : Drop {

}


pub trait VertexBufferT : Drop {

    fn set_data(&self, data : &[Vec3D]);

    fn bind();
}

pub trait IndexBufferT : Drop {
    fn set_data(&self, data : &[usize]);
}