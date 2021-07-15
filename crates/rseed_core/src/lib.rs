pub mod consts;
pub mod utils;
mod log;



// --------------------------
// all the macros imported from external crates
// --------------------------

pub mod error {
    pub use err_derive::*;
}

pub mod builder {
    pub use derive_builder::{self,*};
}

pub mod serialization {
    pub use serde::{self, Serialize, Deserialize};
    pub use toml::{self,from_str, to_string, to_string_pretty};
}

pub mod prelude {
    pub use crate::{
        error::*,
        builder::*,
        serialization::*,
    };
}