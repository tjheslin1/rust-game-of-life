use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp;
use std::env;
use std::process;
use std::{thread, time};

mod cell;
mod config;
mod grid;
mod world;
mod example_worlds;

use config::Config;
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
	let default_world_def = Config::WorldDef {
    	width: 40,
    	height: 40,
    	num_starting_cells: 40,
    	seed: rand::thread_rng().gen_range(1, 10000)
    };

    let presets = vec!["gosper"];

    let config = Config::new(env::args(), default_world_def, presets)
    	.unwrap_or_else(|err| {
    		eprintln!("{}", err);
        	process::exit(1);
    });

	let mut world = World { grid: Grid::new(1, 1) };

	match config {
		Config::Preset { ref key } => {
			println!("preset = {}", key);

			// presets
			world = match example_worlds::find(key) {
				Some(world) => 
					world,
				_ => {
					eprintln!("Didn't find a match for preset: {}", key);
					process::exit(1)
				}
			}
		},
		Config::WorldDef { width, height, num_starting_cells, seed } => {
			println!("seed = {}", seed);

			let mut rng = StdRng::seed_from_u64(seed.into());

		    let mut live_cells: Vec<(u32, u32)> = vec![];

		    let width_offset = cmp::max(1, width/8);
		    let height_offset = cmp::max(1, height/8);

		    for _ in 1..num_starting_cells {
		    	live_cells.push(
		    		(
						rng.gen_range(cmp::max(0, (width/2) - width_offset), cmp::min(width, (width/2) + width_offset)),
		    			rng.gen_range(cmp::max(0, (height/2) - height_offset), cmp::min(height, (height/2) + height_offset))
					)
				);
		    }

		    world = World { grid: Grid::new_alive_grid(
		    	width, 
		    	height, 
		    	live_cells,
				)
			};
		},
	}

    let five_hundred_millis = time::Duration::from_millis(250);

    print!("\x1B[2J\x1B[1;1H");
    for i in 1..1000 {
    	match config {
    		Config::Preset { ref key } =>
    			println!("{}: key = {}", i, key),
			Config::WorldDef { seed, .. } =>
				println!("for help: cargo run help
{}: seed = {}", i, seed),
    	}
        
        print!("{}", world.grid.display());

        thread::sleep(five_hundred_millis);
        print!("\x1B[2J\x1B[1;1H");

        world = world.next();
    }
}

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
