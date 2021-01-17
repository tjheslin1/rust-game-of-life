use std::io;
use std::cmp;
use std::{thread, time};
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;

mod cell;
mod grid;
mod world;

use grid::Grid;
use world::World;

/*
	memorable seeds:
	- 43 (starting cells = 40)
	- 4045 (starting cells = 40)
	- 3658 (starting cells = 40)
*/
#[rustfmt::skip]
fn main() {
	let width = 40;
	let height = 40;
	let num_starting_cells = 40;
    let random_seed: u32 = rand::thread_rng().gen_range(1, 10000);

	let mut input_seed = String::new();
	println!("Input a seed or leave blank for random:");
	io::stdin()
		.read_line(&mut input_seed)
		.expect("Failed to read line");

	let seed: u64 = match input_seed.trim().parse() {
		Ok(user_seed) => user_seed,
		Err(_) => random_seed.into(),
	};

	let mut rng = StdRng::seed_from_u64(seed);

	println!("seed = {}", seed);

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

	// gosper glider gun
	// let mut world = World { grid: Grid::new_alive_grid(40, 40, 
	//     	vec![
	//     		(25, 1),
	//     		(23, 2), (25, 2),
	//     		(13, 3), (14, 3), (21, 3), (22, 3), (35, 3), (36, 3),
	//     		(12, 4), (16, 4), (21, 4), (22, 4), (35, 4), (36, 3),
	//     		(1, 5), (2, 5), (11, 5), (17, 5), (21, 5), (22, 5), 
	//     		(1, 6), (2, 6), (11, 6), (15, 6), (17, 6), (18, 6), (23, 6), (25, 6),
	//     		(11, 7), (17, 7), (25, 7),
	//     		(12, 8), (16, 8),
	//     		(13, 9), (14, 9),
	//     	]
	// 	)
	// };

    let five_hundred_millis = time::Duration::from_millis(250);

    print!("\x1B[2J\x1B[1;1H");
    for i in 1..1000 {
        println!("{}: seed = {}", i, seed);
        print!("{}", world.grid.display());

        thread::sleep(five_hundred_millis);
        print!("\x1B[2J\x1B[1;1H");

        world = world.next();
    }
}
