use game_of_life::Grid;


use std::{thread, time};

fn main() {
    let grid = Grid::new(40, 40);

    let five_hundred_millis = time::Duration::from_millis(500);


    for i in 1..10 {
    	print!("{}{}", i, grid.display());
    	thread::sleep(five_hundred_millis);
    	print!("\x1B[2J\x1B[1;1H");
	}
}