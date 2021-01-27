use std::env;

pub enum Config {
    Preset { key: &'static str },
    WorldDef {
        width: u32,
        height: u32,
        num_starting_cells: u32,
        seed: u32,
    }
}

impl Config {
    pub fn new(args: env::Args, default_world_def: Config, presets: Vec<&'static str>) -> Result<Config, String> {
        let arg_strs = args.collect::<Vec<String>>();

        match arg_strs[..] {
            [_, preset] => {
                if presets.contains(&&preset[..]) {
                    Ok(Config::Preset { key: &preset[..] })
                } else {
                    Err(format!("Unknown preset, choose from {:?}.", presets))
                }
            }
            [_] => Ok(default_world_def),
            [_, w, h, n, s] => {
                let (width, height, num_starting_cells, seed) = Config::parse_args(w, h, n, s)?;

                Ok(Config::WorldDef {
                    width,
                    height,
                    num_starting_cells,
                    seed,
                    }
                )
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
