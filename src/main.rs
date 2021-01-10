use std::{thread, time};

mod grid;
mod cell;

use grid::Grid;

fn main() {
    let grid = Grid::new(40, 40);

    let five_hundred_millis = time::Duration::from_millis(500);

    print!("\x1B[2J\x1B[1;1H");
    for i in 1..10 {
        println!("{}", i);
        print!("{}", grid.display());
        thread::sleep(five_hundred_millis);
        print!("\x1B[2J\x1B[1;1H");
    }
}
