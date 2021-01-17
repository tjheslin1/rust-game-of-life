use std::cmp;
use std::{thread, time};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

mod cell;
mod grid;
mod world;

use grid::Grid;
use world::World;

#[rustfmt::skip]
fn main() {
	let width = 40;
	let height = 40;
	let num_starting_cells = 40;
	let seed = 43;

	println!("seed = {}", seed);

	let mut rng = StdRng::seed_from_u64(seed);

    let mut live_cells: Vec<(u32, u32)> = vec![];
    for _ in 1..num_starting_cells {
    	live_cells.push(
    		(
    			rng.gen_range(cmp::max(0, (width/2)-10), cmp::min(width, (width/2)+10)),
    			rng.gen_range(cmp::max(0, (height/2)-10), cmp::min(height, (height/2)+10))
			)
		);
    }

    let mut world = World { grid: Grid::new_alive_grid(
    	width, 
    	height, 
    	live_cells,
		)
	};

	// glider
	// let mut world = World { grid: Grid::new_alive_grid(40, 40, 
	//     	vec![
	//     		(2, 3), (4, 3),
	//     		(3, 4), (4, 4),
	//     		(3, 5),
	//     	]
	// 	)
	// };

    let five_hundred_millis = time::Duration::from_millis(500);

    print!("\x1B[2J\x1B[1;1H");
    for i in 1..100 {
        println!("{}: seed = {}", i, seed);
        print!("{}", world.grid.display());

        thread::sleep(five_hundred_millis);
        print!("\x1B[2J\x1B[1;1H");

        world = world.next();
    }
}
