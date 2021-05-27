use super::super::fct::fast_isqrt;
use super::super::quat::Quat;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn sq_norm(&self) -> f32 {
        self.dot(self)
    }

    pub fn norm(&self) -> f32 {
        self.sq_norm().sqrt()
    }

    pub fn inv_norm(&self) -> f32 {
        fast_isqrt(self.sq_norm())
    }

    pub fn normalized(&self) -> Self {
        let k = self.inv_norm();
        Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    pub fn dot(&self, v2: &Self) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }

    pub fn cross(&self, v2: &Self) -> Self {
        Self {
            x: self.y * v2.z - self.z * v2.y,
            y: self.x * v2.z - self.z * v2.x,
            z: self.x * v2.y - self.y * v2.x,
        }
    }

    pub fn angle(&self, v2: &Self) -> f32 {
        let cnorm = self.cross(v2).norm();
        cnorm * self.inv_norm() * v2.inv_norm()
    }

    pub fn null() -> Self {
        Vec3D {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
        }
    }
    pub fn up() -> Self {
        Self {
            x: 0_f32,
            y: 0_f32,
            z: 1_f32,
        }
    }
    pub fn down() -> Self {
        Self {
            x: 0_f32,
            y: 0_f32,
            z: -1_f32,
        }
    }
    pub fn forward() -> Self {
        Self {
            x: 1_f32,
            y: 0_f32,
            z: 0_f32,
        }
    }
    pub fn back() -> Self {
        Self {
            x: -1_f32,
            y: 0_f32,
            z: 0_f32,
        }
    }
    pub fn right() -> Self {
        Self {
            x: 0_f32,
            y: -1_f32,
            z: 0_f32,
        }
    }
    pub fn left() -> Self {
        Self {
            x: 0_f32,
            y: 1_f32,
            z: 0_f32,
        }
    }

    pub fn rotate(&mut self, angle: f32, axis: &Vec3D) -> Vec3D {
        let qvec = Quat { s: 0_f32, v: *self };
        let qrot = Quat {
            s: (angle * 0.5).cos(),
            v: (angle * 0.5).sin() * axis.normalized(),
        };

        return (qrot * qvec * qrot.inv()).v;
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }
}

impl std::ops::Add for Vec3D {
    type Output = Vec3D;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl std::ops::AddAssign for Vec3D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}
impl std::ops::Sub for Vec3D {
    type Output = Vec3D;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl std::ops::Neg for Vec3D {
    type Output = Vec3D;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl std::ops::Mul<f32> for Vec3D {
    type Output = Vec3D;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
impl std::ops::MulAssign<f32> for Vec3D {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}
impl std::ops::Mul<Vec3D> for f32 {
    type Output = Vec3D;
    fn mul(self, other: Vec3D) -> Vec3D {
        Vec3D {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}
impl std::fmt::Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
