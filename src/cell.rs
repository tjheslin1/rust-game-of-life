#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub alive: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Cell {
        Cell { x, y, alive: false }
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
            String::from("#")
        } else {
            String::from("-")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_period_when_dead() {
        let cell = Cell {
            x: 0,
            y: 0,
            alive: false,
        };

        assert_eq!(cell.display(), ".");
    }

    #[test]
    fn print_asterisk_when_alive() {
        let cell = Cell {
            x: 0,
            y: 0,
            alive: true,
        };

        assert_eq!(cell.display(), "*");
    }
}
