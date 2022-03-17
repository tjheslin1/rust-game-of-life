use clap::Parser;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp;
use std::env;
use std::process;
use std::{thread, time};

mod cell;
mod cli;
mod config;
mod example_worlds;
mod grid;
mod world;

use cli::Cli;
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
    let args = Cli::parse();

    // let default_world_def = Config::WorldDef {
    //     width: 40,
    //     height: 40,
    //     num_starting_cells: 40,
    //     seed: rand::thread_rng().gen_range(1, 10000),
    //     dead_char: Some(String::from(".")),
    //     alive_char: Some(String::from("*")),
    // };

    let presets = vec!["gosper"];

    // let config = Config::new(env::args(), default_world_def, presets).unwrap_or_else(|err| {
    //     eprintln!("{}", err);
    //     process::exit(1);
    // });

    let mut world = if let Some(ref key) = args.preset {
        match example_worlds::find(key) {
            Some(w) => w,
            _ => {
                eprintln!("Didn't find a match for preset: {}", key);
                process::exit(1)
            }
        }
    } else {
        let seed = args.seed.unwrap_or(rand::thread_rng().gen_range(1, 10000));

        println!("seed = {}", seed);

        let width = args.width.unwrap_or(40);
        let height = args.height.unwrap_or(40);
        let num_starting_cells = args.num_starting_cells.unwrap_or(40);

        World {
            grid: Grid::new_alive_grid(
                width,
                height,
                Some(args.dead_char.unwrap_or(".".to_owned())), // TODO: new_alive_grid should not take Option
                Some(args.alive_char.unwrap_or("#".to_owned())), // TODO: new_alive_grid should not take Option
                starting_cells(seed, width, height, num_starting_cells),
            ),
        }
    };

    let five_hundred_millis = time::Duration::from_millis(250); // TODO: rename?

    print!("\x1B[2J\x1B[1;1H");
    for i in 1..1000 {
        //         match config {
        //             Config::Preset { ref key } => println!("{}: key = {}", i, key),
        //             Config::WorldDef { seed, .. } => println!(
        //                 "for help: gol help
        // {}: seed = {}",
        //                 i, seed
        //             ),
        //         }

        print!("{}", world.grid.display());

        thread::sleep(five_hundred_millis);
        print!("\x1B[2J\x1B[1;1H");

        world = world.next();
    }

    fn starting_cells(
        seed: u32,
        width: u32,
        height: u32,
        num_starting_cells: u32,
    ) -> Vec<(u32, u32)> {
        let mut rng = StdRng::seed_from_u64(seed.into());

        let mut live_cells: Vec<(u32, u32)> = vec![];

        let width_offset = cmp::max(1, width / 8);
        let height_offset = cmp::max(1, height / 8);

        // TODO: refactor
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

        live_cells
    }
}
