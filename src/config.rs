use std::env;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub num_starting_cells: u32,
    pub seed: u32,
}

impl Config {

    pub fn new(args: env::Args, default_config: Config) -> Result<Config, String> {
        match args.collect::<Vec<String>>().as_slice() {
            [_] => 
                Ok(default_config),
            [_, w, h, n, s] => {
                let (width, height, num_starting_cells, seed) = 
                    Config::parse_args(w, h, n, s)?;

                Ok(Config {
                    width,
                    height,
                    num_starting_cells,
                    seed,
                })
            }
            args => {
                println!("{:?}", args[0]);

                Err(format!("Expected 4 args but got {}", args.len()))
            }
        }
    }

    fn parse_args(w: &String, h: &String, n: &String, s: &String) -> Result<(u32, u32, u32, u32), String> {
        match (w.parse(), h.parse(), n.parse(), s.parse()) {
            (Ok(wu), Ok(hu), Ok(nu), Ok(su)) =>
                Ok((wu, hu, nu, su)),
            _ => 
                Err("Unable to parse parameters, please try again.".to_owned()),
        }
    }
}