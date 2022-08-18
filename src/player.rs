use crate::position::GameCubePosition;
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    serde::{Deserialize, Serialize},
};

/// The index of a player to whom the cube belongs (should be 1 or 2).
pub type GamePlayerIndex = i8;

pub fn win_position(player: GamePlayerIndex) -> GameCubePosition {
    match player {
        1 => GameCubePosition { x: 4, y: 2 },
        2 => GameCubePosition { x: 4, y: 6 },
        _ => unreachable!("Game Players only have indexes 1 and 2."),
    }
}
