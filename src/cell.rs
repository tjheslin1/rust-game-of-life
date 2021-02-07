#[derive(Clone, Debug, PartialEq)]
pub struct Cell<'a> {
    pub x: u32,
    pub y: u32,
    alive_character: &'a str,
    dead_character: &'a str,
    pub alive: bool,
}

impl Cell<'_> {
    pub fn new(x: u32, y: u32) -> Cell<'static> {
        Cell {
            x,
            y,
            alive_character: "*",
            dead_character: ".",
            alive: false,
        }
    }

    pub fn new_with_characters<'a>(
        x: u32,
        y: u32,
        alive_character: &'a str,
        dead_character: &'a str,
    ) -> Cell<'a> {
        Cell {
            x,
            y,
            alive_character,
            dead_character,
            alive: false,
        }
    }

    pub fn set_dead(&self) -> Cell {
        Cell {
            alive: false,
            ..*self
        }
    }

    pub fn set_alive(&self) -> Cell {
        Cell {
            alive: true,
            ..*self
        }
    }

    pub fn display(&self) -> &str {
        if self.alive {
            self.alive_character
        } else {
            self.dead_character
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_dead_character_when_dead() {
        let cell = Cell::new(0, 0);

        assert_eq!(cell.display(), ".");
    }

    #[test]
    fn print_alive_character_when_alive() {
        let cell = Cell::new(0, 0);
        let alive_cell = cell.set_alive();

        assert_eq!(alive_cell.display(), "*");
    }
}
