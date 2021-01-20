use std::env;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub num_starting_cells: u32,
    pub seed: u32,
}

impl Config {
    pub fn new(args: env::Args, default_config: Config) -> Result<Config, String> {
        let argss = args.collect::<Vec<String>>();

        let result: Result<(u32, u32, u32, u32), String> = match argss.as_slice() {
            [w, h, n, s] =>
                parse(w, h, n, s),
            [args] => 
                Err(format!("Expected 4 args but got {}", args.len())),
            [] => 
                Err("Didn't get a query string".to_owned()),
            _ =>
                Err("Unknown error :(".to_owned()),
        };

        let (width, height, num_starting_cells, seed) = result?;

        Ok(Config {
            width,
            height,
            num_starting_cells,
            seed,
        })
    }

    fn parse(w: String, h: String, n: String, s: String) -> Result<[u32; 4], String> {
        let width: u32 = w.parse()?;
        let height: u32 = w.parse()?;
        let num_starting_cells: u32 = w.parse()?;
        let seed: u32 = w.parse()?;

        Ok([width, height, num_starting_cells, seed])
    }
}