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
