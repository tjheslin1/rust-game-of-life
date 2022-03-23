use crate::cell::Cell;
use crate::grid::Grid;
use crate::world::Simulation;

#[derive(Clone, Debug, PartialEq)]
pub struct BriansBrain {
    pub grid: Grid,
    pub seed: u32,
}

impl Simulation for BriansBrain {
    fn seed(&self) -> &u32 {
        &self.seed
    }

    fn grid(&self) -> &Grid {
        &self.grid
    }

    fn next(&mut self) {
        unimplemented!("todo")
    }
}

impl BriansBrain {
    pub fn is_dying(cell: &Cell, neighbours: Vec<&Cell>) -> bool {
        unimplemented!("todo")
    }

    pub fn is_alive(cell: &Cell, neighbours: Vec<&Cell>) -> bool {
        unimplemented!("todo")
    }

    pub fn neighbours(&self, cell: &Cell) -> Vec<&Cell> {
        unimplemented!("todo")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn world_preserves_seed() {
    //     let grid = Grid::new(1, 1);
    //     let mut world = BriansBrain { grid, seed: 55 };
    //
    //     world.next();
    //
    //     assert_eq!(world.seed, 55);
    // }
}
