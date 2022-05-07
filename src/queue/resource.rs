use super::tickable;

pub trait Resource : tickable::Tickable + Clone + Copy {
    fn new() -> Self;
    fn get_waiting_ticks(&self) -> u64;
    fn get_served_ticks(&self) -> u64;
}
