use near_sdk::near_bindgen;

use crate::{game::Game, player::Player, Contract, ContractExt};
use near_sdk::serde::{Deserialize, Serialize};

#[near_bindgen]
impl Contract {
    pub fn status(&self) -> Status {
        Status {
            first_player: self.first.clone(),
            second_player: self.second.clone(),
        }
    }

    pub fn get_game(&self) -> Option<Game> {
        self.game.clone()
    }
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Status {
    first_player: Option<Player>,
    second_player: Option<Player>,
}
