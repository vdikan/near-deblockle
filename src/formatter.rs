use crate::face::GameCubeFace;
use crate::game::Game;
use crate::player::{win_position, GamePlayerIndex};
use crate::position::GameCubePosition;

impl Game {
    fn format_board_row(&self, y: i8) -> String {
        assert!((1..=7).contains(&y));
        let v = vec![1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8];
        let vs: Vec<String> = v
            .iter()
            .map(|x| {
                let pos = &GameCubePosition::from(*x, y).unwrap();
                match self.get_cube_at(*pos) {
                    Some(cube) => {
                        let face: GameCubeFace = cube.direction.up.into();
                        let glyph = face.repr_char();
                        let player = cube.player;
                        format!("{}{}", glyph, player)
                    }
                    None => {
                        if *pos == win_position(1) || *pos == win_position(2) {
                            String::from("__")
                        } else {
                            String::from("..")
                        }
                    }
                }
            })
            .collect();

        format!(
            "{y} |{a}|{b}|{c}|{d}|{e}|{f}|{g}|",
            y = y,
            a = vs[0],
            b = vs[1],
            c = vs[2],
            d = vs[3],
            e = vs[4],
            f = vs[5],
            g = vs[6],
        )
    }

    pub fn format_board(&self) -> String {
        let v = vec![1i8, 2i8, 3i8, 4i8, 5i8, 6i8, 7i8];
        let rows: Vec<String> = v.iter().map(|y| self.format_board_row(*y)).collect();
        format!("  :a  b  c  d  e  f  g\n  :1  2  3  4  5  6  7  \n{r1}\n{r2}\n{r3}\n{r4}\n{r5}\n{r6}\n{r7}",
        r1 = rows[0],
        r2 = rows[1],
        r3 = rows[2],
        r4 = rows[3],
        r5 = rows[4],
        r6 = rows[5],
        r7 = rows[6],
        )
    }
}
