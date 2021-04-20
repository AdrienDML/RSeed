
pub struct Version {
    pub major : u32,
    pub minor : u32,
    pub patch : u32,
}

impl Into<u32> for Version {
    fn into(self) -> u32 {
        use ash::vk::make_version;
        make_version(self.major, self.minor, self.patch)
    }
}

impl From<(u32,u32,u32)> for Version {
    fn from(tuple: (u32,u32,u32)) -> Self {
        Self {
            major : tuple.0,
            minor : tuple.1,
            patch : tuple.2,
        }
    }
}