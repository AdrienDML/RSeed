use super::super::fct::fast_isqrt;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
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
    }

    pub fn dot(&self, v2: &Self) -> f32 {
        self.x * v2.x + self.y * v2.y
    }

    pub fn rot(&mut self, angle: &f32) {
        self.x *= angle.cos();
        self.y *= angle.sin();
    }

    pub fn det(&self, v2: &Self) -> f32 {
        self.x * v2.y - self.y * v2.x
    }

    pub fn angle(&self, v2: &Self) -> f32 {
        self.dot(v2) * self.inv_norm() * v2.inv_norm()
    }

    pub fn null() -> Self {
        Vec2D { x: 0_f32, y: 0_f32 }
    }
    pub fn up() -> Self {
        Self { x: 1_f32, y: 0_f32 }
    }
    pub fn down() -> Self {
        Self {
            x: -1_f32,
            y: 0_f32,
        }
    }
    pub fn right() -> Self {
        Self {
            x: 0_f32,
            y: -1_f32,
        }
    }
    pub fn left() -> Self {
        Self { x: 0_f32, y: 1_f32 }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y]
    }
}

impl From<(f32, f32)> for Vec2D {
    fn from(other: (f32, f32)) -> Self {
        Vec2D::new(other.0, other.1)
    }
}

impl std::ops::Add for Vec2D {
    type Output = Vec2D;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::AddAssign for Vec2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl std::ops::Sub for Vec2D {
    type Output = Vec2D;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Neg for Vec2D {
    type Output = Vec2D;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl std::ops::Add<f32> for Vec2D {
    type Output = Vec2D;
    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}
impl std::ops::Mul<f32> for Vec2D {
    type Output = Vec2D;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl std::ops::MulAssign<f32> for Vec2D {
    fn mul_assign(&mut self, other: f32) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl std::fmt::Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
