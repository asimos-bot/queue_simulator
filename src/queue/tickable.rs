pub trait Tickable {

    /// return current tick valuable
    fn get_tick(&self) -> u64;
    /// increment tick and update entity
    fn tick(&mut self);
}
