use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// options: gosper
    #[clap(short, long)]
    pub preset: Option<String>,
    /// options: game_of_life (default), brians_brain
    #[clap(short, long)]
    pub ruleset: Option<String>,
    /// Width, in characters, of the world
    /// Default is 40
    #[clap(short, long)]
    pub width: Option<u32>,
    /// Height, in characters, of the world
    /// Default is 40
    #[clap(short, long)]
    pub height: Option<u32>,
    /// Number of cells that start alive
    #[clap(short, long)]
    pub num_starting_cells: Option<u32>,
    /// A starting seed, to reproduce a previous world
    #[clap(short, long)]
    pub seed: Option<u32>,
    /// Character which represents a dead cell
    #[clap(short, long)]
    pub dead_char: Option<String>,
    /// Character which represents a dying cell
    #[clap(short = 'x', long)]
    pub dying_char: Option<String>,
    /// Character which represents an alive cell
    #[clap(short, long)]
    pub alive_char: Option<String>,
    /// Time, in milliseconds, that each generation lives for (lower to speed up simulation).
    /// Default is 250
    #[clap(short, long)]
    pub gen_length: Option<u64>,
}
