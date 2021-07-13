use rseed_core::prelude::*;

use super::asset::Asset;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum CacheError {

}

pub type Result<T> = std::result::Result<T, CacheError>;

pub struct AssetCache<T>
where
    T : Asset
{
    cache : HashMap<String, T>
}

impl<T> AssetCache<T>
where
    T : Asset
{
    pub fn new() -> Self {
        Self {
            cache : HashMap::new(),
        }
    }

    pub fn insert(name : String, value : T) -> Result<()> {
        todo!()
    }

    pub fn get_asset(name : String) -> Result<T> {
        todo!()
    }
}


