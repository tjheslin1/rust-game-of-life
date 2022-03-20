#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub dead_character: String,
    pub dying_character: String,
    pub alive_character: String,
    pub dying: bool,
    pub alive: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell {
            x,
            y,
            dead_character: String::from("."),
            dying_character: String::from("x"),
            alive_character: String::from("*"),
            dying: false,
            alive: false,
        }
    }

    pub fn new_with_characters(
        x: u32,
        y: u32,
        dead_character: String,
        dying_character: String,
        alive_character: String,
    ) -> Cell {
        Cell {
            x,
            y,
            dead_character,
            dying_character,
            alive_character,
            dying: false,
            alive: false,
        }
    }

    #[allow(dead_code)]
    pub fn set_dead(&self) -> Cell {
        Cell {
            dying: false,
            alive: false,
            dead_character: self.dead_character.clone(),
            dying_character: self.dying_character.clone(),
            alive_character: self.alive_character.clone(),
            ..*self
        }
    }

    #[allow(dead_code)]
    pub fn set_dying(&self) -> Cell {
        Cell {
            dying: true,
            alive: false,
            dead_character: self.dead_character.clone(),
            dying_character: self.dying_character.clone(),
            alive_character: self.alive_character.clone(),
            ..*self
        }
    }

    pub fn set_alive(&self) -> Cell {
        Cell {
            dying: false,
            alive: true,
            dead_character: self.dead_character.clone(),
            dying_character: self.dying_character.clone(),
            alive_character: self.alive_character.clone(),
            ..*self
        }
    }

    pub fn display(&self) -> &str {
        if self.alive {
            &self.alive_character[..]
        } else if self.dying {
            &self.dying_character[..]
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
    fn print_dying_character_when_dying() {
        let cell = Cell::new(0, 0);
        let alive_cell = cell.set_dying();

        assert_eq!(alive_cell.display(), "x");
    }

    #[test]
    fn print_alive_character_when_alive() {
        let cell = Cell::new(0, 0);
        let alive_cell = cell.set_alive();

        assert_eq!(alive_cell.display(), "*");
    }
}
