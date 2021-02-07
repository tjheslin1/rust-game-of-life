use std::env;

pub enum Config {
    Preset {
        key: String,
    },
    WorldDef {
        width: u32,
        height: u32,
        num_starting_cells: u32,
        seed: u32,
        dead_char: Option<&str>,
        alive_char: Option<&str>,
    },
}

impl Config {
    pub fn new(
        args: env::Args,
        default_world_def: Config,
        presets: Vec<&str>,
    ) -> Result<Config, String> {
        let arg_strs = args.collect::<Vec<String>>();

        let result = match arg_strs.as_slice() {
            [_, preset] => {
                if preset == "help" {
                    Err(format!(
                        "try: gol
try: gol {:?}
try: gol [width height num_starting_cells seed dead_cell live_cell] (e.g: gol 40 40 40 4045)",
                        presets
                    ))
                } else if presets.contains(&&preset[..]) {
                    Ok(Config::Preset {
                        key: preset.to_string(),
                    })
                } else {
                    Err(format!("Unknown preset, choose from {:?}.", presets))
                }
            }
            [_] => Ok(default_world_def),
            [_, w, h, n, s] => {
                let (width, height, num_starting_cells, seed) = Config::parse_args(&w, &h, &n, &s)?;

                Ok(Config::WorldDef {
                    width,
                    height,
                    num_starting_cells,
                    seed,
                    None,
                    None,
                })
            }
            [_, w, h, n, s, d, a] => {
                let (width, height, num_starting_cells, seed, dead_char, alive_char) =
                    Config::parse_args(&w, &h, &n, &s, &d, &a)?;

                Ok(Config::WorldDef {
                    width,
                    height,
                    num_starting_cells,
                    seed,
                    dead_char,
                    alive_char,
                })
            }
            args => Err(format!(
                "Expected at least 4 args but got {}",
                args.len() - 1
            )),
        };

        result
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
