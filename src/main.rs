use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp;
use std::env;
use std::process;
use std::{thread, time};

mod cell;
mod config;
mod example_worlds;
mod grid;
mod world;

use config::Config;
use grid::Grid;
use world::World;

/*
    memorable seeds:
    - 43 (starting cells = 40)
    - 4045 (starting cells = 40)
    - 3658 (starting cells = 40)
*/
fn main() {
    let default_world_def = Config::WorldDef {
        width: 40,
        height: 40,
        num_starting_cells: 40,
        seed: rand::thread_rng().gen_range(1, 10000),
        dead_char: Some(String::from(".")),
        alive_char: Some(String::from("*")),
    };

    let presets = vec!["gosper"];

    let config = Config::new(env::args(), default_world_def, presets).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let mut world = World {
        grid: Grid::new(1, 1),
    };

    match config {
        Config::Preset { ref key } => {
            println!("preset = {}", key);

            // presets
            world = match example_worlds::find(key) {
                Some(world) => world,
                _ => {
                    eprintln!("Didn't find a match for preset: {}", key);
                    process::exit(1)
                }
            }
        }
        Config::WorldDef {
            width,
            height,
            num_starting_cells,
            seed,
            ref dead_char,
            ref alive_char,
        } => {
            println!("seed = {}", seed);

            let mut rng = StdRng::seed_from_u64(seed.into());

            let mut live_cells: Vec<(u32, u32)> = vec![];

            let width_offset = cmp::max(1, width / 8);
            let height_offset = cmp::max(1, height / 8);

            for _ in 1..num_starting_cells {
                live_cells.push((
                    rng.gen_range(
                        cmp::max(0, (width / 2) - width_offset),
                        cmp::min(width, (width / 2) + width_offset),
                    ),
                    rng.gen_range(
                        cmp::max(0, (height / 2) - height_offset),
                        cmp::min(height, (height / 2) + height_offset),
                    ),
                ));
            }

            world = World {
                grid: Grid::new_alive_grid(
                    width,
                    height,
                    dead_char.clone(),
                    alive_char.clone(),
                    live_cells,
                ),
            };
        }
    }

    let five_hundred_millis = time::Duration::from_millis(250);

    print!("\x1B[2J\x1B[1;1H");
    for i in 1..1000 {
        match config {
            Config::Preset { ref key } => println!("{}: key = {}", i, key),
            Config::WorldDef { seed, .. } => println!(
                "for help: gol help
{}: seed = {}",
                i, seed
            ),
        }

        print!("{}", world.grid.display());

        thread::sleep(five_hundred_millis);
        print!("\x1B[2J\x1B[1;1H");

        world = world.next();
    }
}
