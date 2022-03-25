use crate::grid::Grid;

pub trait Simulation {
    fn seed(&self) -> &u32;
    fn grid(&self) -> &Grid;

    fn next(&mut self);
}

pub struct World {
    pub game: Box<dyn Simulation>,
}
