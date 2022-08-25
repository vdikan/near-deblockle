use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    json_types::U128,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::position::GameCubePosition;

/// The index of a player to whom the cube belongs (should be 1 or 2).
pub type GamePlayerIndex = i8;

pub fn win_position(player: GamePlayerIndex) -> GameCubePosition {
    match player {
        1 => GameCubePosition { x: 4, y: 2 },
        2 => GameCubePosition { x: 4, y: 6 },
        _ => unreachable!("Game Players only have indexes 1 and 2."),
    }
}

#[near_bindgen]
#[derive(Debug, PartialEq, BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Player {
    index: GamePlayerIndex,
    account: AccountId,
    deposit: U128,
    pub stream: Option<String>,
}

impl Player {
    pub fn new(account: AccountId, deposit: U128, index: GamePlayerIndex) -> Self {
        Self {
            index,
            account,
            deposit,
            stream: None,
        }
    }

    pub fn account(&self) -> &AccountId {
        &self.account
    }

    pub fn deposit(&self) -> U128 {
        self.deposit
    }

    pub fn stream(&self) -> Option<&String> {
        self.stream.as_ref()
    }
}
