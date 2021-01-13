use crate::cell::Cell;
use crate::grid::Grid;

#[derive(Clone, Debug, PartialEq)]
struct World {
    grid: Grid,
}

impl World {
    pub fn new(grid: Grid) -> World {
        World { grid }
    }

    pub fn next(&self) -> World {
        // for each cell find its neigbours
        // populate cell in new grid based on non-updated neighbours
        // let grid = self.grid.clone();
        self.clone()
    }

    pub fn neighbours(&self, cell: &Cell) -> Vec<&Cell> {
        let width = self.grid.cells[0].len();
        let height = self.grid.cells.len();

        let mut neighbours: Vec<&Cell> = vec![];

        // if cell.x > 0 {
        // neighours.push()
        // }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_empty_world() {
        let grid = Grid::new(0, 0);
        let world = World::new(grid);

        let updated_world = world.next();

        assert_eq!(world, updated_world);
    }

    #[test]
    fn find_all_neighbours() {
        let grid = Grid::new(2, 2);
        let world = World::new(grid);

        let below_cell = &Cell::new(0, 1);
        let right_cell = &Cell::new(1, 0);
        let below_right_cell = &Cell::new(1, 1);

        let expected_neightbours = vec![below_cell, right_cell, below_right_cell];

        let actual_neighbours = world.neighbours(&world.grid.cells[0][0]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    #[test]
    fn find_neighbours_top_left_corner() {
        let grid = Grid::new(4, 4);
        let world = World::new(grid);

        let below_cell = &Cell::new(0, 1);
        let right_cell = &Cell::new(1, 0);
        let below_right_cell = &Cell::new(1, 1);

        let expected_neightbours = vec![below_cell, right_cell, below_right_cell];

        let actual_neighbours = world.neighbours(&world.grid.cells[0][0]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }

    #[test]
    fn find_neighbours_in_centre() {
        let grid = Grid::new(10, 10);
        let world = World::new(grid);

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

    #[test]
    fn find_neighbours_bottom_right_corner() {
        let grid = Grid::new(4, 4);
        let world = World::new(grid);

        let above_cell = &Cell::new(3, 2);
        let left_cell = &Cell::new(2, 3);
        let above_left_cell = &Cell::new(2, 2);

        let expected_neightbours = vec![above_cell, left_cell, above_left_cell];

        let actual_neighbours = world.neighbours(&world.grid.cells[3][3]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }
}
