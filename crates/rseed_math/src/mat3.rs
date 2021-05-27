use super::Vec3D;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3 {
    pub(crate) m_data: [f32; 9],
}

impl Mat3 {
    pub fn new() -> Self {
        Self { m_data: [0f32; 9] }
    }

    pub fn from_data(data: [f32; 9]) -> Self {
        Self { m_data: data }
    }

    pub fn id() -> Self {
        let mut m = Self::new();
        m.m_data[0] = 1f32;
        m.m_data[4] = 1f32;
        m.m_data[7] = 1f32;
        m
    }
    pub fn get(&self, i: usize, j: usize) -> f32 {
        self.m_data[3 * i + j]
    }
    pub fn set(&mut self, i: usize, j: usize, value: &f32) {
        self.m_data[3 * i + j] = *value;
    }

    pub fn traspose(&mut self) {
        let c = self.m_data[1];
        self.m_data[1] = self.m_data[3];
        self.m_data[3] = c;
        let c = self.m_data[2];
        self.m_data[2] = self.m_data[6];
        self.m_data[6] = c;
        let c = self.m_data[5];
        self.m_data[5] = self.m_data[7];
        self.m_data[7] = c;
    }

    pub fn inverse(&mut self) {
        let a = self.get(1, 1) * self.get(2, 2) - self.get(1, 2) * self.get(2, 1);
        let b = self.get(1, 2) * self.get(2, 0) - self.get(1, 0) * self.get(2, 2);
        let c = self.get(1, 0) * self.get(2, 1) - self.get(1, 1) * self.get(2, 0);
        let d = self.get(2, 1) * self.get(0, 2) - self.get(0, 1) * self.get(2, 2);
        let e = self.get(0, 0) * self.get(2, 2) - self.get(0, 2) * self.get(2, 0);
        let f = self.get(0, 1) * self.get(2, 0) - self.get(0, 0) * self.get(2, 1);
        let g = self.get(0, 1) * self.get(1, 2) - self.get(1, 1) * self.get(0, 2);
        let h = self.get(1, 0) * self.get(0, 2) - self.get(0, 0) * self.get(1, 2);
        let i = self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0);
        let k = self.get(0, 0) * a + self.get(0, 1) * b + self.get(0, 2) * c;
        *self = Self {
            m_data: [
                a * k,
                d * k,
                g * k,
                b * k,
                e * k,
                h * k,
                c * k,
                f * k,
                i * k,
            ],
        }
    }

    pub fn dot(&self, vec: Vec3D) -> Vec3D {
        Vec3D {
            y: self.get(1, 0) * vec.x + self.get(1, 1) * vec.y + self.get(1, 2) * vec.z,
            z: self.get(2, 0) * vec.x + self.get(2, 1) * vec.y + self.get(2, 2) * vec.z,
            x: self.get(0, 0) * vec.x + self.get(0, 1) * vec.y + self.get(0, 2) * vec.z,
        }
    }
}
