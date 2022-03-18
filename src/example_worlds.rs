use crate::grid::Grid;
use crate::world::World;

pub fn find(key: &String) -> Option<World> {
    match &key[..] {
        "gosper" => Some(World {
            grid: Grid::new_alive_grid(
                40,
                40,
                Some(String::from(".")),
                Some(String::from("#")),
                vec![
                    (25, 1),
                    (23, 2),
                    (25, 2),
                    (13, 3),
                    (14, 3),
                    (21, 3),
                    (22, 3),
                    (35, 3),
                    (36, 3),
                    (12, 4),
                    (16, 4),
                    (21, 4),
                    (22, 4),
                    (35, 4),
                    (36, 3),
                    (1, 5),
                    (2, 5),
                    (11, 5),
                    (17, 5),
                    (21, 5),
                    (22, 5),
                    (1, 6),
                    (2, 6),
                    (11, 6),
                    (15, 6),
                    (17, 6),
                    (18, 6),
                    (23, 6),
                    (25, 6),
                    (11, 7),
                    (17, 7),
                    (25, 7),
                    (12, 8),
                    (16, 8),
                    (13, 9),
                    (14, 9),
                ],
            ),
            seed: 0,
        }),
        _ => None,
    }
}
// }
