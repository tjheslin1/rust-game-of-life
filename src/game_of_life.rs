use crate::cell::Cell;
use crate::grid::Grid;
use crate::neighbours::find_neighbours;
use crate::world::Simulation;

#[derive(Clone, Debug, PartialEq)]
pub struct GameOfLife {
    pub grid: Grid,
    pub seed: u32,
}

impl Simulation for GameOfLife {
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

                if GameOfLife::is_alive(cell, neighbours) {
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

impl GameOfLife {
    pub fn is_alive(cell: &Cell, neighbours: Vec<&Cell>) -> bool {
        let alive_neighbours_count = neighbours.iter().filter(|&c| c.alive).count();

        if cell.alive {
            if alive_neighbours_count < 2 || alive_neighbours_count > 3 {
                false
            } else {
                true
            }
        } else {
            alive_neighbours_count == 3
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_preserves_seed() {
        let grid = Grid::new(1, 1);
        let mut world = GameOfLife { grid, seed: 55 };

        world.next();

        assert_eq!(world.seed, 55);
    }

    /*

       .  ->  .

    */
    #[test]
    fn update_empty_world() {
        let grid = Grid::new(1, 1);
        let mut world = GameOfLife {
            grid: grid.clone(),
            seed: 0,
        };

        world.next();

        let expected_world = GameOfLife {
            grid: grid.clone(),
            seed: 0,
        };

        assert_eq!(world, expected_world);
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
            String::new(), String::new(), String::new(),
        	vec![
        		(2, 1), (3, 1),
        		(1, 2),  (3, 2),
        		(2, 3),
        	],
            vec![],
    	);

        let mut world = GameOfLife { grid, seed: 0 };

        let expected_grid = Grid::new_alive_grid(
        	5, 5,
            String::new(), String::new(), String::new(),
        	vec![
        		(2, 1), (3, 1),
        		(1, 2),  (3, 2),
        		(2, 3),
        	],
            vec![],
    	);
        let expected_world = GameOfLife { grid: expected_grid, seed: 0 };

        world.next();

        assert_eq!(expected_world, world);
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
            String::new(), String::new(), String::new(),
        	vec![
        		(1, 1), (2, 1),
        		(1, 2),
        	],
            vec![],
    	);
        let mut world = GameOfLife { grid, seed: 0 };

        let expected_grid = Grid::new_alive_grid(
        	4, 4,
            String::new(), String::new(), String::new(),
        	vec![
        		(1, 1), (2, 1),
        		(1, 2), (2, 2),
        	],
            vec![],
    	);
        let expected_world = GameOfLife { grid: expected_grid, seed: 0 };

        world.next();

        assert_eq!(expected_world, world);
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
            String::new(), String::new(), String::new(),
        	vec![
        		        (1, 0),
        		(0, 1), (1, 1), (2, 1),
        		        (1, 2), (2, 2),
        	],
            vec![],
    	);
        let mut world = GameOfLife { grid, seed: 0 };

        let expected_grid = Grid::new_alive_grid(
        	4, 4,
            String::new(), String::new(), String::new(),
        	vec![
        		(0, 0), (1, 0), (2, 0),
        		(0, 1),
        		(0, 2),         (2, 2),
        	],
            vec![],
    	);
        let expected_world = GameOfLife { grid: expected_grid, seed: 0 };

        world.next();

        assert_eq!(expected_world, world);
    }

    #[test]
    fn alive_cell_no_longer_alive_for_all_dead_neighbours() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1);
        let cell_two = &Cell::new(1, 0);
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = GameOfLife::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, false);
    }

    #[test]
    fn alive_cell_no_longer_alive_for_one_alive_neighbour() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0);
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = GameOfLife::is_alive(&alive_cell, neighbours);

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

        let is_alive = GameOfLife::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, false);
    }

    #[test]
    fn alive_cell_stays_alive_for_two_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = GameOfLife::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, true);
    }

    #[test]
    fn alive_cell_stays_alive_for_three_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_alive();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1).set_alive();

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = GameOfLife::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, true);
    }

    #[test]
    fn dead_cell_becomes_alive_for_three_alive_neighbours() {
        let alive_cell = Cell::new(0, 0);

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1).set_alive();

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = GameOfLife::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, true);
    }

    #[test]
    fn dead_cell_stays_dead_for_fewer_than_three_alive_neighbours() {
        let alive_cell = Cell::new(0, 0).set_dead();

        let cell_one = &Cell::new(0, 1).set_alive();
        let cell_two = &Cell::new(1, 0).set_alive();
        let cell_three = &Cell::new(1, 1);

        let neighbours = vec![cell_one, cell_two, cell_three];

        let is_alive = GameOfLife::is_alive(&alive_cell, neighbours);

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

        let is_alive = GameOfLife::is_alive(&alive_cell, neighbours);

        assert_eq!(is_alive, false);
    }
}
