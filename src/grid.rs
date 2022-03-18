use itertools::Itertools;

use crate::cell::Cell;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let mut cells: Vec<Vec<Cell>> = vec![];

        for y in 0..height {
            let mut row: Vec<Cell> = vec![];
            for x in 0..width {
                row.push(Cell::new(x, y));
            }
            cells.push(row);
        }

        Grid { cells }
    }

    pub fn new_alive_grid(
        width: u32,
        height: u32,
        dead_char: String,
        alive_char: String,
        alive_cells: Vec<(u32, u32)>,
    ) -> Grid {
        let mut cells: Vec<Vec<Cell>> = vec![];

        for y in 0..height {
            let mut row: Vec<Cell> = vec![];
            for x in 0..width {
                let cell = Cell::new_with_characters(x, y, dead_char.clone(), alive_char.clone());
                let alive_cell = cell.set_alive();

                if alive_cells.contains(&(x, y)) {
                    row.push(alive_cell);
                } else {
                    row.push(cell);
                }
            }
            cells.push(row);
        }

        Grid { cells }
    }

    pub fn display(&self) -> String {
        let x: String = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.display())
                    .intersperse(" ")
                    .collect::<String>()
            })
            .intersperse("\n".to_string())
            .collect();

        format!("{}", x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_grid() {
        let grid = Grid::new(0, 0);

        assert_eq!(grid.cells.len(), 0);
    }

    #[test]
    fn one_row_grid() {
        let grid = Grid::new(10, 1);

        assert_eq!(grid.cells.len(), 1);
        assert_eq!(grid.cells[0].len(), 10);
    }

    #[test]
    fn square_grid() {
        let grid = Grid::new(5, 5);

        assert_eq!(grid.cells.len(), 5);
        assert_eq!(grid.cells[0].len(), 5);
        assert_eq!(grid.cells[1].len(), 5);
        assert_eq!(grid.cells[2].len(), 5);
        assert_eq!(grid.cells[3].len(), 5);
        assert_eq!(grid.cells[4].len(), 5);
    }

    #[test]
    fn display_empty_grid() {
        let grid = Grid::new(0, 0);

        assert_eq!(grid.display(), "");
    }

    #[test]
    fn display_one_row_grid_of_dead_cells() {
        let grid = Grid::new(10, 1);

        assert_eq!(grid.display(), ". . . . . . . . . .");
    }

    #[test]
    fn display_one_row_grid_of_alive_cells() {
        let grid = Grid::new_alive_grid(
            10,
            1,
            ".".to_owned(),
            "*".to_owned(),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 0),
            ],
        );

        assert_eq!(grid.display(), "* * * * * * * * * *");
    }

    #[test]
    fn display_one_row_grid_of_alive_cells_with_custom_characters() {
        let grid = Grid::new_alive_grid(
            10,
            1,
            String::from("_"),
            String::from("#"),
            vec![
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0),
                (9, 0),
            ],
        );

        assert_eq!(grid.display(), "# # # # # # # # # #");
    }

    #[test]
    fn display_square_grid_of_dead_cells() {
        let grid = Grid::new(10, 10);

        assert_eq!(
            grid.display(),
            ". . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . ."
        );
    }

    #[test]
    #[rustfmt::skip]
    fn display_square_grid_of_alive_cells() {

        let grid = Grid::new_alive_grid(
            10, 10,
            ".".to_owned(), "*".to_owned(),
            vec![
                (0, 0),(0, 1),(0, 2),(0, 3),(0, 4),(0, 5),(0, 6),(0, 7),(0, 8),(0, 9),
                (1, 0),(1, 1),(1, 2),(1, 3),(1, 4),(1, 5),(1, 6),(1, 7),(1, 8),(1, 9),
                (2, 0),(2, 1),(2, 2),(2, 3),(2, 4),(2, 5),(2, 6),(2, 7),(2, 8),(2, 9),
                (3, 0),(3, 1),(3, 2),(3, 3),(3, 4),(3, 5),(3, 6),(3, 7),(3, 8),(3, 9),
                (4, 0),(4, 1),(4, 2),(4, 3),(4, 4),(4, 5),(4, 6),(4, 7),(4, 8),(4, 9),
                (5, 0),(5, 1),(5, 2),(5, 3),(5, 4),(5, 5),(5, 6),(5, 7),(5, 8),(5, 9),
                (6, 0),(6, 1),(6, 2),(6, 3),(6, 4),(6, 5),(6, 6),(6, 7),(6, 8),(6, 9),
                (7, 0),(7, 1),(7, 2),(7, 3),(7, 4),(7, 5),(7, 6),(7, 7),(7, 8),(7, 9),
                (8, 0),(8, 1),(8, 2),(8, 3),(8, 4),(8, 5),(8, 6),(8, 7),(8, 8),(8, 9),
                (9, 0),(9, 1),(9, 2),(9, 3),(9, 4),(9, 5),(9, 6),(9, 7),(9, 8),(9, 9),
            ],
        );

        assert_eq!(
            grid.display(),
            "* * * * * * * * * *
* * * * * * * * * *
* * * * * * * * * *
* * * * * * * * * *
* * * * * * * * * *
* * * * * * * * * *
* * * * * * * * * *
* * * * * * * * * *
* * * * * * * * * *
* * * * * * * * * *"
        );
    }

    #[test]
    fn display_square_grid_with_one_alive_cell() {
        let grid = Grid::new_alive_grid(10, 10, ".".to_owned(), "*".to_owned(), vec![(2, 3)]);

        assert_eq!(
            grid.display(),
            ". . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . * . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . ."
        );
    }
}
