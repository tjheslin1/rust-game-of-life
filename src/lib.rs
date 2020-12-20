use itertools::Itertools;

#[derive(Debug)]
pub struct Cell {
    x: u32,
    y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell { x, y }
    }
}

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<Cell>>,
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
        let x: String = self.cells.iter()
            .map(|row| row.iter().map(|cell| ".".to_string()).intersperse(" ".to_string()).collect::<String>()).intersperse("\n".to_string()).collect();

        // x.intersperse(" ").collect()

        // String::from("")

        format!("{}\r", x)
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

        assert_eq!(grid.display(), "\r");
    }

    #[test]
    fn display_one_row_grid() {
        let grid = Grid::new(10, 1);

        assert_eq!(grid.display(), ". . . . . . . . . .\r");
    }

    #[test]
    fn display_square_grid() {
        let grid = Grid::new(10, 10);

        assert_eq!(grid.display(), ". . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .
. . . . . . . . . .\r");
    }
}
