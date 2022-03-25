use crate::cell::Cell;
use crate::grid::Grid;

pub fn find_neighbours<'a>(grid: &'a Grid, cell: &'a Cell) -> Vec<&'a Cell> {
    let width = grid.cells[0].len();
    let height = grid.cells.len();

    let mut neighbours: Vec<&Cell> = vec![];

    if (cell.x > 0) & (cell.y > 0) {
        let left = (cell.x - 1) as usize;
        let above = (cell.y - 1) as usize;

        neighbours.push(&grid.cells[above][left]);
    }

    if cell.x > 0 {
        let left = (cell.x - 1) as usize;

        neighbours.push(&grid.cells[cell.y as usize][left]);
    }

    if (cell.x > 0) & (cell.y < (height - 1) as u32) {
        let left = (cell.x - 1) as usize;
        let below = (cell.y + 1) as usize;

        neighbours.push(&grid.cells[below][left]);
    }

    if cell.y > 0 {
        let above = (cell.y - 1) as usize;

        neighbours.push(&grid.cells[above][cell.x as usize]);
    }

    if cell.y < (height - 1) as u32 {
        let below = (cell.y + 1) as usize;

        neighbours.push(&grid.cells[below][cell.x as usize]);
    }

    if (cell.x < (width - 1) as u32) & (cell.y > 0) {
        let right = (cell.x + 1) as usize;
        let above = (cell.y - 1) as usize;

        neighbours.push(&grid.cells[above][right]);
    }

    if cell.x < (width - 1) as u32 {
        let right = (cell.x + 1) as usize;

        neighbours.push(&grid.cells[cell.y as usize][right]);
    }

    if (cell.x < (width - 1) as u32) & (cell.y < (height - 1) as u32) {
        let right = (cell.x + 1) as usize;
        let below = (cell.y + 1) as usize;

        neighbours.push(&grid.cells[below][right]);
    }

    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;

    /*

       * *
       * *

    */
    #[test]
    fn find_all_neighbours() {
        let grid = Grid::new(2, 2);

        let below_cell = &Cell::new(0, 1);
        let right_cell = &Cell::new(1, 0);
        let below_right_cell = &Cell::new(1, 1);

        let expected_neightbours = vec![below_cell, right_cell, below_right_cell];

        let actual_neighbours = find_neighbours(&grid, &grid.cells[0][0]);

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

        let below_cell = &Cell::new(0, 1);
        let right_cell = &Cell::new(1, 0);
        let below_right_cell = &Cell::new(1, 1);

        let expected_neightbours = vec![below_cell, right_cell, below_right_cell];

        let actual_neighbours = find_neighbours(&grid, &grid.cells[0][0]);

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

        let actual_neighbours = find_neighbours(&grid, &grid.cells[2][2]);

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

        let above_left_cell = &Cell::new(2, 2);
        let left_cell = &Cell::new(2, 3);
        let above_cell = &Cell::new(3, 2);

        let expected_neightbours = vec![above_left_cell, left_cell, above_cell];

        let actual_neighbours = find_neighbours(&grid, &grid.cells[3][3]);

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

        let actual_neighbours = find_neighbours(&grid, &grid.cells[1][0]);

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

        let actual_neighbours = find_neighbours(&grid, &grid.cells[2][3]);

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

        let actual_neighbours = find_neighbours(&grid, &grid.cells[0][2]);

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

        let actual_neighbours = find_neighbours(&grid, &grid.cells[3][1]);

        assert_eq!(actual_neighbours, expected_neightbours);
    }
}
