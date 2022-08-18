/// Game Block Faces, matching regular d6 dice scores.
/// The opposite faces are 7-complement.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameCubeFace {
    Star = 1,
    XHop = 2,
    Slide = 3,
    Hoops = 4,
    THop = 5,
    Stop = 6,
}

impl From<i8> for GameCubeFace {
    fn from(orig: i8) -> Self {
        match orig {
            1i8 => GameCubeFace::Star,
            2i8 => GameCubeFace::XHop,
            3i8 => GameCubeFace::Slide,
            4i8 => GameCubeFace::Hoops,
            5i8 => GameCubeFace::THop,
            6i8 => GameCubeFace::Stop,

            _ => panic!(
                "Should not refer to players other than 1 and 2! (requested: {})",
                orig
            ),
        }
    }
}

impl GameCubeFace {
    pub fn repr_char(&self) -> &str {
        match self {
            GameCubeFace::Star => "S",
            GameCubeFace::XHop => "X",
            GameCubeFace::Slide => "L",
            GameCubeFace::Hoops => "H",
            GameCubeFace::THop => "T",
            GameCubeFace::Stop => "P",
        }
    }
}

pub fn opposite_face(face: GameCubeFace) -> GameCubeFace {
    let face_index = face as i8;
    (7i8 - face_index).into()
}
