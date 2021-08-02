use serde::{Deserialize, Serialize, de::Visitor};
use super::bit::bit_range_mask;


#[derive(Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

struct StringVisitor;

impl<'de> Visitor<'de> for StringVisitor {
    type Value = Version;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a version of the form : major.minor.patch")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let mut it = v.split('.');
        let major = str::parse::<u32>(it.next().unwrap_or("0"))
            .map_err(|_| E::custom(format!("eror wile parsing major")))?;
        let minor = str::parse::<u32>(it.next().unwrap_or("0"))
            .map_err(|_| E::custom(format!("eror wile parsing minor")))?;
        let patch = str::parse::<u32>(it.next().unwrap_or("0"))
            .map_err(|_| E::custom(format!("eror wile parsing patch")))?;
        Ok(Self::Value {
            major,
            minor,
            patch,
        })
    }

  
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        deserializer.deserialize_str(StringVisitor)
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let s = format!("{}.{}.{}", self.major, self.minor, self.patch);
        serializer.serialize_str(&s)
    }
}



impl Into<u32> for Version {
    fn into(self) -> u32 {
        (self.major << 22) | (self.minor << 12) | self.patch
    }
}

impl From<u32> for Version {
    fn from(other : u32) -> Self {
        Self {
            major : (other & bit_range_mask(22, 31)) >> 22,
            minor : (other & bit_range_mask(12, 21)) >> 12,
            patch : other & bit_range_mask(0, 11),
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}