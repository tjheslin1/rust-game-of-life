#[derive(Debug)]
struct World {
	grid: Grid
}

impl World {
	pub fun new(grid: Grid) -> World {
		World { grid }
	}

	pub fn next(&self) -> World {
		// for each cell find its neigbours
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn empty_grid() {
    //     let grid = Grid::new(0, 0);

    //     assert_eq!(grid.cells.len(), 0);
    // }
}