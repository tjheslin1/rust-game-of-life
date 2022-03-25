use crate::cell::Cell;
use crate::grid::Grid;
use crate::neighbours::find_neighbours;
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
        let width = self.grid.cells[0].len();
        let height = self.grid.cells.len();

        let mut updated_cells: Vec<Vec<Cell>> = vec![];

        for y in 0..height {
            let mut row: Vec<Cell> = vec![];
            for x in 0..width {
                let cell = &self.grid.cells[y][x];
                let neighbours = find_neighbours(&self.grid, cell);

                if BriansBrain::is_alive(cell, neighbours) {
                    row.push(
                        Cell::new_with_characters(
                            x as u32,
                            y as u32,
                            cell.dead_character.clone(),
                            cell.dying_character.clone(),
                            cell.alive_character.clone(),
                        )
                        .set_alive(),
                    )
                } else if BriansBrain::is_dying(cell) {
                    row.push(
                        Cell::new_with_characters(
                            x as u32,
                            y as u32,
                            cell.dead_character.clone(),
                            cell.dying_character.clone(),
                            cell.alive_character.clone(),
                        )
                        .set_dying(),
                    )
                } else {
                    row.push(Cell::new_with_characters(
                        x as u32,
                        y as u32,
                        cell.dead_character.clone(),
                        cell.dying_character.clone(),
                        cell.alive_character.clone(),
                    ))
                }
            }
            updated_cells.push(row);
        }

        self.grid.cells = updated_cells;
    }
}

// A cell turns on if it was off but had exactly two neighbors that were on.
// All cells that were "on" go into the "dying" state.
// Cells that were in the dying state go into the off state.
impl BriansBrain {
    pub fn is_dying(cell: &Cell) -> bool {
        cell.alive
    }

    pub fn is_alive(cell: &Cell, neighbours: Vec<&Cell>) -> bool {
        let alive_neighbours_count = neighbours.iter().filter(|&c| c.alive).count();

        cell.alive == false && cell.dying == false && alive_neighbours_count == 2
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

       . . . . .      . . * * .
       . . * * .      . * x x *
       . * . * .  ->  . x . x *
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
                (2, 0), (3, 0),
                (1, 1), (4, 1),
                (4, 2),
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

       . . . .      . * * .
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
                (1, 0), (2, 0),
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

    #[test]
    fn dead_cell_set_to_alive_for_two_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_dead();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = BriansBrain::is_alive(&alive_cell, neighbours);
        assert_eq!(is_alive, true);
    }

    #[test]
    fn alive_cell_set_to_dying() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1);
        let cell_two = &Cell::new(1, 0);
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = BriansBrain::is_alive(&alive_cell, neighbours);
        assert_eq!(is_alive, false);

        let is_dying = BriansBrain::is_dying(&alive_cell);
        assert_eq!(is_dying, true);
    }

    #[test]
    fn dying_cell_set_to_dead() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1);
        let cell_two = &Cell::new(1, 0);
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = BriansBrain::is_alive(&alive_cell, neighbours);
        assert_eq!(is_alive, false);

        let is_dying = BriansBrain::is_dying(&alive_cell);
        assert_eq!(is_dying, true);
    }
}
