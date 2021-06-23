
pub trait ContextT : Drop {
    fn swap_buffers(&self);

}
