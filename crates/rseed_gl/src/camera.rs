use crate::math::Vec3D;


pub struct Camera {
    pub pos : Vec3D,
    target : Vec3D,
    up : Vec3D,
}

impl Camera {
    pub fn new(pos : Vec3D, target : Vec3D, up : Vec3D) -> Self {
        Self {
            pos,
            target,
            up,
        }
    }

    pub fn look_at(&mut self, target : Vec3D) {
        self.target = target; 
    }
}
