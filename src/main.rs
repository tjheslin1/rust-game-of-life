use std::{thread, time};

mod cell;
mod grid;
mod world;

use grid::Grid;
use world::World;

#[rustfmt::skip]
fn main() {
    let mut world = World { grid: Grid::new_alive_grid(40, 40, 
	    	vec![
	    		(3, 2),
	    		(3, 4), //(4, 4),
	    		(3, 5),
	    		(3, 6), (4, 6), (6, 7),
	    		(3, 8), (4, 8), (6, 8),
	    	]
		)
	};

    let five_hundred_millis = time::Duration::from_millis(500);

    print!("\x1B[2J\x1B[1;1H");
    for i in 1..100 {
        println!("{}", i);
        print!("{}", world.grid.display());

        thread::sleep(five_hundred_millis);
        print!("\x1B[2J\x1B[1;1H");

        world = world.next();
    }
}

/*
stuck:

. * *
* . *
. * .


*/
