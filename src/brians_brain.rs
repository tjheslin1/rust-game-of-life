use crate::cell::Cell;
use crate::grid::Grid;
use crate::ruleset::Ruleset;

#[derive(Clone, Debug, PartialEq)]
pub struct BriansBrain {
    pub grid: Grid,
    pub seed: u32,
}

impl Game<BriansBrain> for BriansBrain {
    fn next(&self) -> BriansBrain {}
}

impl BriansBrain {
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

    use crate::ruleset::Ruleset;

    #[test]
    fn world_preserves_seed() {
        let grid = Grid::new(1, 1);
        let world = BriansBrain { grid, seed: 55 };

        let updated_world = world.next();

        assert_eq!(updated_world.seed, 55);
    }
}
