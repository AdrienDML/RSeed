use super::fct::fast_isqrt;
use super::Vec3D;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat {
    pub s: f32,
    pub v: Vec3D,
}

impl Quat {
    pub fn null() -> Self {
        Self {
            s: 0_f32,
            v: Vec3D::null(),
        }
    }

    pub fn sq_norm(&self) -> f32 {
        self.s * self.s + self.v.sq_norm()
    }

    pub fn norm(&self) -> f32 {
        self.sq_norm().sqrt()
    }
    pub fn inv_norm(&self) -> f32 {
        fast_isqrt(self.sq_norm())
    }

    pub fn normalize(&mut self) {
        let k = self.inv_norm();
        self.s *= k;
        self.v *= k;
    }

    pub fn conj(&self) -> Self {
        Self {
            s: self.s,
            v: -self.v,
        }
    }

    pub fn conjugate(&mut self) {
        self.v = -self.v;
    }

    pub fn inv(&self) -> Self {
        self.conj() * self.inv_norm()
    }

    pub fn inverse(&mut self) {
        *self = self.inv();
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.s, self.v.x, self.v.y, self.v.z]
    }
}

impl std::ops::Add for Quat {
    type Output = Quat;
    fn add(self, other: Self) -> Self {
        Self {
            s: self.s + other.s,
            v: self.v + other.v,
        }
    }
}
impl std::ops::AddAssign for Quat {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            s: self.s + other.s,
            v: self.v + other.v,
        };
    }
}
impl std::ops::Sub for Quat {
    type Output = Quat;
    fn sub(self, other: Self) -> Self {
        Self {
            s: self.s - other.s,
            v: self.v - other.v,
        }
    }
}
impl std::ops::Neg for Quat {
    type Output = Quat;
    fn neg(self) -> Self {
        Self {
            s: -self.s,
            v: -self.v,
        }
    }
}
impl std::ops::Mul<f32> for Quat {
    type Output = Quat;
    fn mul(self, other: f32) -> Self {
        Self {
            s: self.s * other,
            v: self.v * other,
        }
    }
}
impl std::ops::MulAssign<f32> for Quat {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            s: self.s * other,
            v: self.v * other,
        };
    }
}
impl std::ops::Mul<Quat> for Quat {
    type Output = Quat;
    fn mul(self, other: Quat) -> Self {
        Self {
            s: self.s * other.s - self.v.dot(&other.v),
            v: self.s * other.v + other.s * self.v + self.v.cross(&other.v),
        }
    }
}
impl std::ops::MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, other: Quat) {
        *self = Self {
            s: self.s * other.s - self.v.dot(&other.v),
            v: self.s * other.v + other.s * self.v + self.v.cross(&other.v),
        };
    }
}

impl std::fmt::Display for Quat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{};{}]", self.s, self.v)
    }
}
