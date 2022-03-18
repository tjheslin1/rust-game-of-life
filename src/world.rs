use crate::cell::Cell;
use crate::grid::Grid;

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub grid: Grid,
    pub seed: u32,
}

impl World {
    pub fn next(&self) -> World {
        let width = self.grid.cells[0].len();
        let height = self.grid.cells.len();

        let mut updated_cells: Vec<Vec<Cell>> = vec![];

        for y in 0..height {
            let mut row: Vec<Cell> = vec![];
            for x in 0..width {
                let cell = &self.grid.cells[y][x];
                let neighbours = self.neighbours(cell);

                if World::is_alive(cell, neighbours) {
                    row.push(
                        Cell::new_with_characters(
                            x as u32,
                            y as u32,
                            cell.dead_character.clone(),
                            cell.alive_character.clone(),
                        )
                        .set_alive(),
                    );
                } else {
                    row.push(Cell::new_with_characters(
                        x as u32,
                        y as u32,
                        cell.dead_character.clone(),
                        cell.alive_character.clone(),
                    ));
                }
            }
            updated_cells.push(row);
        }

        World {
            grid: Grid {
                cells: updated_cells,
            },
            ..*self
        }
    }

    pub fn is_alive(cell: &Cell, neighbours: Vec<&Cell>) -> bool {
        let alive_neighbours_count = neighbours.iter().filter(|&c| c.alive).count();

        if cell.alive {
            if alive_neighbours_count < 2 {
                return false;
            } else if alive_neighbours_count > 3 {
                return false;
            } else {
                return true;
            }
        } else {
            if alive_neighbours_count == 3 {
                return true;
            } else {
                return false;
            }
        }
    }

    pub fn neighbours(&self, cell: &Cell) -> Vec<&Cell> {
        let width = self.grid.cells[0].len();
        let height = self.grid.cells.len();

        let mut neighbours: Vec<&Cell> = vec![];

        if (cell.x > 0) & (cell.y > 0) {
            let left = (cell.x - 1) as usize;
            let above = (cell.y - 1) as usize;

            neighbours.push(&self.grid.cells[above][left]);
        }

        if cell.x > 0 {
            let left = (cell.x - 1) as usize;

            neighbours.push(&self.grid.cells[cell.y as usize][left]);
        }

        if (cell.x > 0) & (cell.y < (height - 1) as u32) {
            let left = (cell.x - 1) as usize;
            let below = (cell.y + 1) as usize;

            neighbours.push(&self.grid.cells[below][left]);
        }

        if cell.y > 0 {
            let above = (cell.y - 1) as usize;

            neighbours.push(&self.grid.cells[above][cell.x as usize]);
        }

        if cell.y < (height - 1) as u32 {
            let below = (cell.y + 1) as usize;

            neighbours.push(&self.grid.cells[below][cell.x as usize]);
        }

        if (cell.x < (width - 1) as u32) & (cell.y > 0) {
            let right = (cell.x + 1) as usize;
            let above = (cell.y - 1) as usize;

            neighbours.push(&self.grid.cells[above][right]);
        }

        if cell.x < (width - 1) as u32 {
            let right = (cell.x + 1) as usize;

            neighbours.push(&self.grid.cells[cell.y as usize][right]);
        }

        if (cell.x < (width - 1) as u32) & (cell.y < (height - 1) as u32) {
            let right = (cell.x + 1) as usize;
            let below = (cell.y + 1) as usize;

            neighbours.push(&self.grid.cells[below][right]);
        }

        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_preserves_seed() {
        let grid = Grid::new(1, 1);
        let world = World { grid, seed: 55 };

        let updated_world = world.next();

        assert_eq!(world.seed, 55);
    }

    /*

       .  ->  .

    */
    #[test]
    fn update_tiny_world() {
        let grid = Grid::new(1, 1);
        let world = World { grid, seed: 0 };

        let updated_world = world.next();

        assert_eq!(world, updated_world);
    }

    /*

       . . . . .      . . . . .
       . . * * .      . . * * .
       . * . * .  ->  . * . * .
       . . * . .      . . * . .
       . . . . .      . . . . .

    */
    #[test]
    #[rustfmt::skip]
    fn update_static_world() {
        let grid = Grid::new_alive_grid(
        	5, 5,
            String::new(), String::new(),
        	vec![
        		(2, 1), (3, 1),
        		(1, 2),  (3, 2),
        		(2, 3),
        	],
    	);
        let world = World { grid, seed: 0 };

        let expected_grid = Grid::new_alive_grid(
        	5, 5,
            String::new(), String::new(),
        	vec![
        		(2, 1), (3, 1),
        		(1, 2),  (3, 2),
        		(2, 3),
        	],
    	);
        let expected_world = World { grid: expected_grid, seed: 0 };

        let actual_world = world.next();

        assert_eq!(expected_world, actual_world);
    }

    /*

       . . . .      . . . .
       . * * .      . * * .
       . * . .  ->  . * * .
       . . . .      . . . .

    */
    #[test]
    #[rustfmt::skip]
    fn update_world_one_dead_cell_to_set_alive() {
        let grid = Grid::new_alive_grid(
        	4, 4,
            String::new(), String::new(),
        	vec![
        		(1, 1), (2, 1),
        		(1, 2),
        	],
    	);
        let world = World { grid, seed: 0 };

        let expected_grid = Grid::new_alive_grid(
        	4, 4,
            String::new(), String::new(),
        	vec![
        		(1, 1), (2, 1),
        		(1, 2), (2, 2),
        	],
    	);
        let expected_world = World { grid: expected_grid, seed: 0 };

        let actual_world = world.next();

        assert_eq!(expected_world, actual_world);
    }

    /*

       . * . .      * * * .
       * * * .      * . . .
       . * * .  ->  * . * .
       . . . .      . . . .

    */
    #[test]
    #[rustfmt::skip]
    fn update_world_one_alive_cell_to_set_dead() {
        let grid = Grid::new_alive_grid(
        	4, 4,
            String::new(), String::new(),
        	vec![
        		        (1, 0),
        		(0, 1), (1, 1), (2, 1),
        		        (1, 2), (2, 2),
        	],
    	);
        let world = World { grid, seed: 0 };

        let expected_grid = Grid::new_alive_grid(
        	4, 4,
            String::new(), String::new(),
        	vec![
        		(0, 0), (1, 0), (2, 0),
        		(0, 1),
        		(0, 2),         (2, 2),
        	],
    	);
        let expected_world = World { grid: expected_grid, seed: 0 };

        let actual_world = world.next();

        assert_eq!(expected_world, actual_world);
    }

    /*

       * *
       * *

    */
    #[test]
    fn find_all_neighbours() {
        let grid = Grid::new(2, 2);
        let world = World { grid, seed: 0 };

        let below_cell = &Cell::new(0, 1);
        let right_cell = &Cell::new(1, 0);
        let below_right_cell = &Cell::new(1, 1);

        let expected_neightbours = vec![below_cell, right_cell, below_right_cell];

        let actual_neighbours = world.neighbours(&world.grid.cells[0][0]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    /*

       * . . .
       . . . .
       . . . .
       . . . .

    */
    #[test]
    fn find_neighbours_top_left_corner() {
        let grid = Grid::new(4, 4);
        let world = World { grid, seed: 0 };

        let below_cell = &Cell::new(0, 1);
        let right_cell = &Cell::new(1, 0);
        let below_right_cell = &Cell::new(1, 1);

        let expected_neightbours = vec![below_cell, right_cell, below_right_cell];

        let actual_neighbours = world.neighbours(&world.grid.cells[0][0]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    /*

       . . . . . . . . . .
       . . . . . . . . . .
       . . * . . . . . . .
       . . . . . . . . . .
       . . . . . . . . . .
       . . . . . . . . . .
       . . . . . . . . . .
       . . . . . . . . . .
       . . . . . . . . . .
       . . . . . . . . . .

    */
    #[test]
    fn find_neighbours_in_centre() {
        let grid = Grid::new(10, 10);
        let world = World { grid, seed: 0 };

        let above_left_cell = &Cell::new(1, 1);
        let left_cell = &Cell::new(1, 2);
        let below_left_cell = &Cell::new(1, 3);
        let above_cell = &Cell::new(2, 1);
        let below_cell = &Cell::new(2, 3);
        let above_right_cell = &Cell::new(3, 1);
        let right_cell = &Cell::new(3, 2);
        let below_right_cell = &Cell::new(3, 3);

        let expected_neightbours = vec![
            above_left_cell,
            left_cell,
            below_left_cell,
            above_cell,
            below_cell,
            above_right_cell,
            right_cell,
            below_right_cell,
        ];

        let actual_neighbours = world.neighbours(&world.grid.cells[2][2]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    /*

       . . . .
       . . . .
       . . . .
       . . . *

    */
    #[test]
    fn find_neighbours_bottom_right_corner() {
        let grid = Grid::new(4, 4);
        let world = World { grid, seed: 0 };

        let above_left_cell = &Cell::new(2, 2);
        let left_cell = &Cell::new(2, 3);
        let above_cell = &Cell::new(3, 2);

        let expected_neightbours = vec![above_left_cell, left_cell, above_cell];

        let actual_neighbours = world.neighbours(&world.grid.cells[3][3]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    /*

       . . . .
       * . . .
       . . . .
       . . . .

    */
    #[test]
    fn find_neighbours_left_edge() {
        let grid = Grid::new(4, 4);
        let world = World { grid, seed: 0 };

        let above_cell = &Cell::new(0, 0);
        let below_cell = &Cell::new(0, 2);
        let above_right_cell = &Cell::new(1, 0);
        let right_cell = &Cell::new(1, 1);
        let below_right_cell = &Cell::new(1, 2);

        let expected_neightbours = vec![
            above_cell,
            below_cell,
            above_right_cell,
            right_cell,
            below_right_cell,
        ];

        let actual_neighbours = world.neighbours(&world.grid.cells[1][0]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    /*

       . . . .
       . . . .
       . . . *
       . . . .

    */
    #[test]
    fn find_neighbours_right_edge() {
        let grid = Grid::new(4, 4);
        let world = World { grid, seed: 0 };

        let above_left_cell = &Cell::new(2, 1);
        let left_cell = &Cell::new(2, 2);
        let below_left_cell = &Cell::new(2, 3);
        let above_cell = &Cell::new(3, 1);
        let below_cell = &Cell::new(3, 3);

        let expected_neightbours = vec![
            above_left_cell,
            left_cell,
            below_left_cell,
            above_cell,
            below_cell,
        ];

        let actual_neighbours = world.neighbours(&world.grid.cells[2][3]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    /*

       . . * .
       . . . .
       . . . .
       . . . .

    */
    #[test]
    fn find_neighbours_above_edge() {
        let grid = Grid::new(4, 4);
        let world = World { grid, seed: 0 };

        let left_cell = &Cell::new(1, 0);
        let below_left_cell = &Cell::new(1, 1);
        let below_cell = &Cell::new(2, 1);
        let right_cell = &Cell::new(3, 0);
        let below_right_cell = &Cell::new(3, 1);

        let expected_neightbours = vec![
            left_cell,
            below_left_cell,
            below_cell,
            right_cell,
            below_right_cell,
        ];

        let actual_neighbours = world.neighbours(&world.grid.cells[0][2]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    /*

       . . . .
       . . . .
       . . . .
       . * . .

    */
    #[test]
    fn find_neighbours_below_edge() {
        let grid = Grid::new(4, 4);
        let world = World { grid, seed: 0 };

        let above_left_cell = &Cell::new(0, 2);
        let left_cell = &Cell::new(0, 3);
        let aobve_cell = &Cell::new(1, 2);
        let above_right_cell = &Cell::new(2, 2);
        let right_cell = &Cell::new(2, 3);

        let expected_neightbours = vec![
            above_left_cell,
            left_cell,
            aobve_cell,
            above_right_cell,
            right_cell,
        ];

        let actual_neighbours = world.neighbours(&world.grid.cells[3][1]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    #[test]
    fn alive_cell_no_longer_alive_for_all_dead_neighbours() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1);
        let cell_two = &Cell::new(1, 0);
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = World::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, false);
    }

    #[test]
    fn alive_cell_no_longer_alive_for_one_alive_neighbour() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0);
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = World::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, false);
    }

    #[test]
    fn alive_cell_no_longer_alive_for_more_thhan_three_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1).set_alive();
        let cell_four = &Cell::new(1, 2).set_alive();

        let neighbours = vec![cell_one, cell_two, cell_three, cell_four];

        let is_alive = World::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, false);
    }

    #[test]
    fn alive_cell_stays_alive_for_two_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = World::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, true);
    }

    #[test]
    fn alive_cell_stays_alive_for_three_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1).set_alive();

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = World::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, true);
    }

    #[test]
    fn dead_cell_becomes_alive_for_three_alive_neighbours() {
        let alive_cell = Cell::new(0, 0);

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1).set_alive();

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = World::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, true);
    }

    #[test]
    fn dead_cell_stays_dead_for_fewer_than_three_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_dead();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = World::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, false);
    }

    #[test]
    fn dead_cell_stays_dead_for_more_than_three_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_dead();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1).set_alive();
        let cell_four = &Cell::new(1, 2).set_alive();

        let neighbours = vec![cell_one, cell_two, cell_three, cell_four];

        let is_alive = World::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, false);
    }
}
