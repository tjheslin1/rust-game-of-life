use crate::grid::Grid;


#[derive(Clone, Debug)]
struct World {
	grid: Grid
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_empty_grid() {
        let grid = Grid::new(0, 0);
        let world = World::new(grid);

        let updated_world = world.next();

        assert_eq!(world.grid.cells.len(), updated_world.grid.cells.len());
    }
}