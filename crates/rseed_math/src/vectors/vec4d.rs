use super::super::fct::fast_isqrt;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4D {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
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

    pub fn normalize(&mut self) {
        let k = self.inv_norm();
        self.x *= k;
        self.y *= k;
        self.z *= k;
        self.w *= k;
    }

    pub fn normalized(&self) -> Self {
        let k = self.inv_norm();
        Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
            w: self.w * k,
        }
    }

    pub fn dot(&self, v2: &Self) -> f32 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z + self.w * v2.w
    }

    pub fn null() -> Self {
        Vec4D {
            x: 0_f32,
            y: 0_f32,
            z: 0_f32,
            w: 0_f32,
        }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z, self.w]
    }
}

impl From<(f32, f32, f32, f32)> for Vec4D {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Vec4D::new(other.0, other.1, other.2, other.3)
    }
}

impl std::ops::Add for Vec4D {
    type Output = Vec4D;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl std::ops::AddAssign for Vec4D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        };
    }
}
impl std::ops::Sub for Vec4D {
    type Output = Vec4D;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.z,
        }
    }
}
impl std::ops::Neg for Vec4D {
    type Output = Vec4D;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
impl std::ops::Mul<f32> for Vec4D {
    type Output = Vec4D;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}
impl std::ops::MulAssign<f32> for Vec4D {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        };
    }
}
impl std::ops::Mul<Vec4D> for f32 {
    type Output = Vec4D;
    fn mul(self, other: Vec4D) -> Vec4D {
        Vec4D {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
            w: self * other.w,
        }
    }
}
impl std::fmt::Display for Vec4D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{},{})", self.x, self.y, self.z, self.w)
    }
}
