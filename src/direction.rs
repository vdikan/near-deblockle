use crate::face::{opposite_face, GameCubeFace};
use crate::position::GameCubePosition;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

/// General directions of movement on the game board.
pub enum Direction {
    Forward,
    Right,
    Backward,
    Left,
}

/// Directions of the Game Cube in the ordoer:
/// Up (facing the player), Forward, Right.
#[derive(
    Clone, Copy, Debug, PartialEq, BorshDeserialize, BorshSerialize, Serialize, Deserialize,
)]
#[serde(crate = "near_sdk::serde")]
pub struct GameCubeDirection {
    pub up: i8,
    pub front: i8,
    pub right: i8,
}

impl GameCubeDirection {
    pub fn from(up: i8, front: i8, right: i8) -> Option<GameCubeDirection> {
        if (1..=6).contains(&up)
            && (1..=6).contains(&front)
            && (1..=6).contains(&right)
            && (up + front != 7)
            && (up + right != 7)
            && (front + right != 7)
            && (up != front && front != right && up != right)
        {
            Some(GameCubeDirection { up, front, right })
        } else {
            None
        }
    }

    /// Get a permuted direction of game block faces after a would-be roll
    /// in a specified direction.
    pub fn direction_after_roll(&self, roll_direction: Direction) -> Self {
        let up: &GameCubeFace = &self.up.into();
        let front: &GameCubeFace = &self.front.into();
        let right: &GameCubeFace = &self.right.into();
        let down = opposite_face(*up);
        let back = opposite_face(*front);
        let left = opposite_face(*right);

        match roll_direction {
            Direction::Forward => GameCubeDirection {
                up: back as i8,
                front: *up as i8,
                right: *right as i8,
            },
            Direction::Right => GameCubeDirection {
                up: left as i8,
                front: *front as i8,
                right: *up as i8,
            },
            Direction::Backward => GameCubeDirection {
                up: *front as i8,
                front: down as i8,
                right: *right as i8,
            },
            Direction::Left => GameCubeDirection {
                up: *right as i8,
                front: *front as i8,
                right: down as i8,
            },
        }
    }

    pub fn format_layout(&self) -> String {
        let up: &GameCubeFace = &self.up.into();
        let front: &GameCubeFace = &self.front.into();
        let right: &GameCubeFace = &self.right.into();
        let down = opposite_face(*up);
        let back = opposite_face(*front);
        let left = opposite_face(*right);

        format!(
            "  :-:  \n  |{}|  \n:-:-:-:\n|{}|{}|{}|\n:-:-:-:\n  |{}|  \n  :-:  \n  |{}|  \n  :-:  ",
            front.repr_char(),
            left.repr_char(),
            up.repr_char(),
            right.repr_char(),
            back.repr_char(),
            down.repr_char()
        )
    }
}

pub fn get_roll_direction(from: GameCubePosition, to: GameCubePosition) -> Direction {
    if to.x < from.x {
        Direction::Left
    } else if to.x > from.x {
        Direction::Right
    } else if to.y < from.y {
        Direction::Forward
    } else {
        Direction::Backward
    }
}
