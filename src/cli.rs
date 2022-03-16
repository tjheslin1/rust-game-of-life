use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Width, in characters, of the world
    #[clap(short, long)]
    width: u32,
    /// Height, in characters, of the world
    #[clap(short, long)]
    height: u32,
    /// Number of cells that start alive
    #[clap(short, long)]
    num_starting_cells: u32,
    /// A starting seed, to reproduce a previous world
    #[clap(short, long)]
    seed: Option<u32>,
    /// Character which represents a dead cell
    #[clap(short, long)]
    dead_char: Option<String>,
    /// Character which represents an alive cell
    #[clap(short, long)]
    alive_char: Option<String>,
}
