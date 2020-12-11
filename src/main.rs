use game_of_life::Cell;

fn main() {
	let cells = (0..40).map(|x|
		(0..80).map(move |y|
			Cell::new(x, y)
		)
	);
}
