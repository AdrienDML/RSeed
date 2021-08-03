use crate::vec2d::Vec2D;
use crate::vec3d::Vec3D;

pub struct NoiseParam2 {
    center : Vec2D,
    angle : f64,
    seed : i8,
    octaves : u8,
    pub persistance : f64,
    pub lacunarity : f64,
}

impl Default for NoiseParam2 {
    fn default() -> Self {
        NoiseParam2 {
            center : Vec2D::null(),
            angle : 0_f64,
            seed : 0_i8,
            octaves : 1_u8,
            persistance : 0_f64,
            lacunarity : 0_f64,
        }
    }
}

impl NoiseParam2 {
    pub fn new(center : Vec2D, angle : f64) -> Self {
        Self {
            center : center,
            angle : angle,
            ..Self::default()
        }
    }
    pub fn translate(&mut self, vec :Vec2D) {
        self.center += vec;
    }
    pub fn rotate(&mut self, angle: f64) {
        self.angle += angle;
    }
}

pub struct NoiseParam3{
    center : Vec3D,
    angle : Vec3D,
    seed : i8,
    octaves : u8,
    persistance : f64,
    lacunarity : f64,
}

impl Default for NoiseParam3 {
    fn default() -> Self {
        Self {
            center : Vec3D::null(),
            angle : Vec3D::null(),
            seed : 0_i8,
            octaves : 1_u8,
            persistance : 0_f64,
            lacunarity : 0_f64,
        }
    }
}

impl NoiseParam3{
    pub fn new(center : Vec3D, angle : Vec3D) -> Self {
        Self {
            center : center,
            angle : angle,
            ..Self::default()
        }
    }
    pub fn translate(&self, vec: Vec3D) {
        self.center += vec;
    }
    pub fn rotate(&self, angle: Vec3D) {
        self.angle += angle;
    }
}