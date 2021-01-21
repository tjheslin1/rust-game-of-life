#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    alive_character: String,
    dead_character: String,
    pub alive: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell { x, y, 
            alive_character: String::from("*"), 
            dead_character: String::from("."), 
            alive: false }
    }


    pub fn new_with_characters(x: u32, y: u32, alive_character: String, dead_character: String) -> Cell {
        Cell { x, y, alive_character, dead_character, alive: false }
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

    pub fn display(&self) -> String {
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
        let cell = Cell::new(0, 0).set_alive();

        assert_eq!(cell.display(), "*");
    }
}
