use itertools::Itertools;

use crate::cell::Cell;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Grid {
        let mut cells: Vec<Vec<Cell>> = vec![];

        for x in 0..height {
            let mut row: Vec<Cell> = vec![];
            for y in 0..width {
                row.push(Cell::new(x, y));
            }
            cells.push(row);
        }

        return Grid { cells };
    }

    pub fn display(&self) -> String {
        let x: String = self
            .cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.display())
                    .intersperse(" ".to_string())
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
        let mut grid = Grid::new(10, 1);

        grid.cells = grid
            .cells
            .iter()
            .map(|row| row.iter().map(|cell| cell.alive()).collect())
            .collect();

        assert_eq!(grid.display(), "* * * * * * * * * *");
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
    fn display_square_grid_of_alive_cells() {
        let mut grid = Grid::new(10, 10);

        grid.cells = grid
            .cells
            .iter()
            .map(|row| row.iter().map(|cell| cell.alive()).collect())
            .collect();

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
}
