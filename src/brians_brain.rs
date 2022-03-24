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

    fn next(&mut self) {}
}

// a cell turns on if it was off but had exactly two neighbors that were on
// All cells that were "on" go into the "dying" state
// Cells that were in the dying state go into the off state
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

    #[test]
    fn world_preserves_seed() {
        let grid = Grid::new(1, 1);
        let mut world = BriansBrain { grid, seed: 55 };

        world.next();

        assert_eq!(world.seed, 55);
    }

    /*

       .  ->  .

    */
    #[test]
    fn update_empty_world() {
        let grid = Grid::new(1, 1);
        let mut world = BriansBrain {
            grid: grid.clone(),
            seed: 0,
        };

        world.next();

        let expected_world = BriansBrain {
            grid: grid.clone(),
            seed: 0,
        };

        assert_eq!(world, expected_world);
    }

    /*

       *  ->  x

    */
    #[test]
    fn update_tiny_alive_world() {
        let grid = Grid::new_alive_grid(
            1,
            1,
            String::new(),
            String::new(),
            String::new(),
            vec![(1, 1)],
            vec![],
        );

        let mut world = BriansBrain {
            grid: grid.clone(),
            seed: 0,
        };

        world.next();

        let expected_world = BriansBrain {
            grid: grid.clone(),
            seed: 0,
        };

        assert_eq!(world, expected_world);
    }

    /*

       x  ->  .

    */
    #[test]
    fn update_tiny_dying_world() {
        let grid = Grid::new_alive_grid(
            1,
            1,
            String::new(),
            String::new(),
            String::new(),
            vec![],
            vec![(1, 1)],
        );

        let mut world = BriansBrain { grid, seed: 0 };

        world.next();

        let expected_grid = Grid::new_alive_grid(
            1,
            1,
            String::new(),
            String::new(),
            String::new(),
            vec![],
            vec![],
        );

        let expected_world = BriansBrain {
            grid: expected_grid,
            seed: 0,
        };

        assert_eq!(world, expected_world);
    }

    /*

       . . . . .      . . . * .
       . . * * .      . * x x *
       . * . * .  ->  . x . x .
       . . * . .      . * x * .
       . . . . .      . . . . .

    */
    #[test]
    #[rustfmt::skip]
    fn update_alive_world() {
        let grid = Grid::new_alive_grid(
        	5, 5,
            String::new(), String::new(), String::new(),
        	vec![
        		(2, 1), (3, 1),
        		(1, 2),  (3, 2),
        		(2, 3),
        	],
            vec![],
    	);

        let mut world = BriansBrain { grid, seed: 0 };

        let expected_grid = Grid::new_alive_grid(
        	5, 5,
            String::new(), String::new(), String::new(),
            vec![
                (3, 0),
                (1, 1), (4, 1),
                (1, 3), (3, 3),
            ],
        	vec![
        		(2, 1), (3, 1),
        		(1, 2),  (3, 2),
        		(2, 3),
        	],
    	);
        let expected_world = BriansBrain { grid: expected_grid, seed: 0 };

        world.next();

        assert_eq!(expected_world, world);
    }

    /*

       . . . .      . . . .
       . * * .      . x x .
       . . . .  ->  . * * .
       . . . .      . . . .

    */
    #[test]
    #[rustfmt::skip]
    fn update_world_dead_cells_to_set_alive() {
        let grid = Grid::new_alive_grid(
        	4, 4,
            String::new(), String::new(), String::new(),
        	vec![
        		(1, 1), (2, 1),
        	],
            vec![],
    	);
        let mut world = BriansBrain { grid, seed: 0 };

        let expected_grid = Grid::new_alive_grid(
        	4, 4,
            String::new(), String::new(), String::new(),
        	vec![
        		(1, 2), (2, 2),
        	],
            vec![
                (1, 1), (2, 1),
            ],
    	);
        let expected_world = BriansBrain { grid: expected_grid, seed: 0 };

        world.next();

        assert_eq!(expected_world, world);
    }

    // #[test]
    // fn alive_cell_no_longer_alive_for_all_dead_neighbours() {
    //     let alive_cell = Cell::new(0, 0).set_alive();
    //
    //     let cell_one = &Cell::new(0, 1);
    //     let cell_two = &Cell::new(1, 0);
    //     let cell_three = &Cell::new(1, 1);
    //
    //     let neighbours = vec![cell_one, cell_two, cell_three];
    //
    //     let is_alive = BriansBrain::is_alive(&alive_cell, neighbours);
    //
    //     assert_eq!(is_alive, false);
    // }
}
