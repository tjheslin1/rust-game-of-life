use clap::Parser;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::cmp;
use std::process;
use std::{thread, time};

mod brians_brain;
mod cell;
mod cli;
mod example_worlds;
mod game_of_life;
mod grid;
mod world;

use brians_brain::BriansBrain;
use cli::Cli;
use game_of_life::GameOfLife;
use grid::Grid;
use world::World;

/*
    memorable seeds:
    - 4045 (starting cells = 40)
*/
fn main() {
    let args = Cli::parse();

    let mut world: World = if let Some(ref key) = args.preset {
        match example_worlds::find(key) {
            Some(w) => w,
            _ => {
                eprintln!("Didn't find a match for preset: {}", key);
                process::exit(1)
            }
        }
    } else {
        let seed = args
            .seed
            .unwrap_or_else(|| rand::thread_rng().gen_range(1, 10000));

        println!("seed = {}", seed);

        let width = args.width.unwrap_or(40);
        let height = args.height.unwrap_or(40);
        let num_starting_cells = args.num_starting_cells.unwrap_or(40);

        let grid = Grid::new_alive_grid(
            width,
            height,
            args.dead_char.unwrap_or_else(|| ".".to_owned()),
            args.dying_char.unwrap_or_else(|| "x".to_owned()),
            args.alive_char.unwrap_or_else(|| "#".to_owned()),
            starting_cells(seed, width, height, num_starting_cells),
        );

        args.ruleset
            .map(|r| match r {
                rule if rule.starts_with("game_of") || rule.starts_with("gameof") => World {
                    game: Box::new(GameOfLife {
                        grid: grid.clone(),
                        seed,
                    }),
                },
                rule if rule.starts_with("brian") => World {
                    game: Box::new(BriansBrain {
                        grid: grid.clone(),
                        seed,
                    }),
                },
                rule => panic!("Unknown ruleset: {}", rule),
            })
            .unwrap_or(World {
                game: Box::new(GameOfLife {
                    grid: grid.clone(),
                    seed,
                }),
            })
    };

    let gen_length = time::Duration::from_millis(args.gen_length.unwrap_or(250));

    clear_screen();

    for i in 1..1000 {
        if let Some(ref preset) = args.preset {
            println!("{}: key = {}", i, preset)
        } else {
            println!("for help: --help");
            println!("seed = {}; generation = {}", world.game.seed(), i)
        }

        print!("{}", world.game.grid().display());

        thread::sleep(gen_length);

        clear_screen();

        world.game.next();
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

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H")
}
