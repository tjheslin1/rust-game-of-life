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
        dead_char: Option<String>,
        alive_char: Option<String>,
    },
}

impl Config {
    pub fn new(
        args: env::Args,
        default_world_def: Config,
        presets: Vec<&'static str>,
    ) -> Result<Config, String> {
        let args2 = args;
        let args_vec = args2.collect::<Vec<String>>();
        let args_slice = args_vec.as_slice();

        let result = match args_slice {
            [_, preset] => {
                if preset == "help" {
                    Err(format!(
                        "try: gol
try: gol {:?}
try: gol [width height num_starting_cells seed display_dead display_alive] (e.g: gol 40 40 40 4045)
When providing your own display characters 

$ gol 40 40 40 4045 . #

You may need to place your characters in double-quotes.

$ gol 40 40 40 4045 \"-\" \"#\"",
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
                    dead_char: None,
                    alive_char: None,
                })
            }
            [_, w, h, n, s, d, a] => {
                let (width, height, num_starting_cells, seed) = Config::parse_args(&w, &h, &n, &s)?;

                Ok(Config::WorldDef {
                    width,
                    height,
                    num_starting_cells,
                    seed,
                    dead_char: Some(d.to_owned()),
                    alive_char: Some(a.to_owned()),
                })
            }
            args => Err(format!(
                "Expected at least 4 or 6 args but got {}",
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
