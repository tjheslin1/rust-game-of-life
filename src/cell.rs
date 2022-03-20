#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub dead_character: String,
    pub alive_character: String,
    pub alive: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell {
            x,
            y,
            dead_character: String::from("."),
            alive_character: String::from("*"),
            alive: false,
        }
    }

    pub fn new_with_characters(
        x: u32,
        y: u32,
        dead_character: String,
        alive_character: String,
    ) -> Cell {
        Cell {
            x,
            y,
            dead_character,
            alive_character,
            alive: false,
        }
    }

    #[allow(dead_code)]
    pub fn set_dead(&self) -> Cell {
        Cell {
            alive: false,
            dead_character: self.dead_character.clone(),
            alive_character: self.alive_character.clone(),
            ..*self
        }
    }

    pub fn set_alive(&self) -> Cell {
        Cell {
            alive: true,
            dead_character: self.dead_character.clone(),
            alive_character: self.alive_character.clone(),
            ..*self
        }
    }

    pub fn display(&self) -> &str {
        if self.alive {
            &self.alive_character[..]
        } else {
            &self.dead_character[..]
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
