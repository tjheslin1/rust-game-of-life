use std::env;

pub struct Config {
    pub worldDef: Option<WorldDef>,
    pub preset: Option<&<'static> str>
}

pub struct WorldDef {
    pub width: u32,
    pub height: u32,
    pub num_starting_cells: u32,
    pub seed: u32,
}

impl Config {
    pub fn new(args: env::Args, default_config: Config, presets: Vec<&str>) -> Result<Either<Config>, String> {
        match args.collect::<Vec<String>>().as_slice() {
            [_, preset] => {
                if presets.contains(preset) {
                    Ok(Config { worldDef: None, preset: Some(preset) }),
                } else {
                    Err(format!("Unknown preset, choose from {}.", presets))
                }
            }
            [_] => Ok(default_config),
            [_, w, h, n, s] => {
                let (width, height, num_starting_cells, seed) = Config::parse_args(w, h, n, s)?;

                Ok(Config {
                    WorldDef {
                    width,
                    height,
                    num_starting_cells,
                    seed,
                    },
                    preset: None,
                })
            }
            args => {
                println!("{:?}", args[0]);

                Err(format!("Expected 4 args but got {}", args.len() - 1))
            }
        }
    }

    fn parse_args(
        w: &String,
        h: &String,
        n: &String,
        s: &String,
    ) -> Result<(u32, u32, u32, u32), String> {
        match (w.parse(), h.parse(), n.parse(), s.parse()) {
            (Ok(wu), Ok(hu), Ok(nu), Ok(su)) => Ok((wu, hu, nu, su)),
            _ => Err("Unable to parse parameters, please try again.".to_owned()),
        }
    }
}
