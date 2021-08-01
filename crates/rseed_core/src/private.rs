// cool trick to make users not be able to use some functions from the API
pub struct Local;

pub trait IsLocal {}

impl IsLocal for Local {}