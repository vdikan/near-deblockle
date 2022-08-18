use crate::position::GameCubePosition;

pub fn coord_pattern_to_positions(v: Vec<(i8, i8)>) -> Vec<GameCubePosition> {
    v.into_iter()
        .filter_map(|p| GameCubePosition::from(p.0, p.1))
        .collect()
}

pub fn t_move_pattern(x: i8, y: i8) -> Vec<(i8, i8)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

pub fn x_move_pattern(x: i8, y: i8) -> Vec<(i8, i8)> {
    vec![
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ]
}

pub fn h_move_pattern(x: i8, y: i8) -> Vec<(i8, i8)> {
    vec![
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 3, y),
        (x + 3, y),
        (x, y - 3),
        (x, y + 3),
        (x - 2, y + 1),
        (x - 2, y - 1),
        (x + 2, y + 1),
        (x + 2, y - 1),
        (x + 1, y - 2),
        (x - 1, y - 2),
        (x + 1, y + 2),
        (x - 1, y + 2),
    ]
}

pub fn l_move_pattern(x: i8, y: i8) -> Vec<(i8, i8)> {
    let mut col: Vec<(i8, i8)> = (1i8..=7).map(|i| (x, i)).collect();
    let mut row: Vec<(i8, i8)> = (1i8..=7).map(|i| (i, y)).collect();
    row.append(&mut col);
    row
}
