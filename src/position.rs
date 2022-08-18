use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

/// Coordinates of the Game Cube position on a board.
#[derive(
    Clone, Copy, Debug, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize,
)]
#[serde(crate = "near_sdk::serde")]
pub struct GameCubePosition {
    pub x: i8,
    pub y: i8,
}

impl GameCubePosition {
    pub fn from(x: i8, y: i8) -> Option<GameCubePosition> {
        if (1..=7).contains(&x) && (1..=7).contains(&y) {
            Some(GameCubePosition {
                x: x as i8,
                y: y as i8,
            })
        } else {
            None
        }
    }
}
