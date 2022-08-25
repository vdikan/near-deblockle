#![allow(unused_imports)]

pub mod direction;
// mod external;
pub mod face;
pub mod formatter;
pub mod game;
pub mod game_setup;
// mod interface;
pub mod move_pattern;
pub mod player;
pub mod position;

use direction::GameCubeDirection;
use game::{Game, GameCube, GamePhase};
use player::GamePlayerIndex;
use position::GameCubePosition;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};

// pub const MIN_BOOKING_TIME: u64 = 30_000_000_000; // half a minute
// pub const MIN_DEPOSIT: u128 = 1_000_000_000_000_000_000_000_000; // 1 NEAR
// pub const EXTRA_FACTOR: u128 = 20_000_000_000_000; // divisor for deposit->time conversion above minimal

pub type GameIndex = u64;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    Games,
    Field { game_id: GameIndex },
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct GameWithData {
    pub game: Game,
    pub is_finished: bool,
    pub first_player: AccountId,
    pub second_player: AccountId,
}

impl GameWithData {
    pub fn new(
        first_player: AccountId,
        second_player: AccountId,
        num_cubes: Option<usize>,
    ) -> Self {
        GameWithData {
            game: Game::game_setup(num_cubes),
            is_finished: false,
            first_player,
            second_player,
        }
    }
}

/// Contract state definition.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub games: Vector<GameWithData>,
}

// impl Default for Contract {
//     fn default() -> Self {
//         Self::new()
//     }
// }

/// Contract functions implementations.
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            games: Vector::new(StorageKey::Games),
        }
    }

    pub fn create_game(
        &mut self,
        first_player: AccountId,
        second_player: AccountId,
        num_cubes: Option<usize>,
    ) -> GameIndex {
        let index = self.games.len();
        self.games
            .push(&GameWithData::new(first_player, second_player, num_cubes));
        log!(
            "Created Game setup with {} cubes per player",
            num_cubes.unwrap_or(4)
        );
        index
    }

    pub fn game_state(&self, index: GameIndex) -> Option<GameWithData> {
        let game_with_data = self.games.get(index);
        if game_with_data.is_some() {
            log!("Game board:");
            log!("{}", game_with_data.as_ref().unwrap().game.format_board());
        }
        game_with_data
    }

    pub fn cube_state(&self, index: GameIndex, x: i8, y: i8) -> Option<GameCube> {
        let game_with_data = self.games.get(index).expect("Game doesn't exist.");

        let pos: Option<GameCubePosition> = GameCubePosition::from(x, y);
        match pos {
            Some(pos) => {
                let cube = game_with_data.game.get_cube_at(pos);
                if cube.is_some() {}
                match cube {
                    Some(..) => {
                        log!("{}", cube.unwrap().direction.format_layout());
                        cube
                    }
                    None => {
                        log!("no cube at position ({},{})", pos.x, pos.y);
                        None
                    }
                }
            }
            None => {
                log!("illegal position ({},{})", x, y);
                None
            }
        }
    }

    pub fn pass_move(&mut self, index: GameIndex) -> GameWithData {
        let mut game_with_data = self.games.get(index).expect("Game doesn't exist.");

        assert!(
            (env::predecessor_account_id() == game_with_data.first_player
                && game_with_data.game.active_player == 1)
                || (env::predecessor_account_id() == game_with_data.second_player
                    && game_with_data.game.active_player == 2),
            "Wrong player's turn! "
        );

        let phase = game_with_data.game.phase;
        let active_player_ind = game_with_data.game.active_player;
        let other_player_ind = 3 - active_player_ind;
        match phase {
            GamePhase::End => log!("Game is finished, no moves allowed"),
            _ => {
                game_with_data.game.phase = GamePhase::Roll;
                game_with_data.game.active_player = other_player_ind;
                log!(
                    "Player {} passed the turn. It is player {} Roll phase.",
                    active_player_ind,
                    other_player_ind
                );
                self.games.replace(index, &game_with_data);
            }
        }
        game_with_data
    }

    pub fn make_move(
        &mut self,
        index: GameIndex,
        from_x: i8,
        from_y: i8,
        to_x: i8,
        to_y: i8,
    ) -> GameWithData {
        let mut game_with_data = self.games.get(index).expect("Game doesn't exist.");

        assert!(
            (env::predecessor_account_id() == game_with_data.first_player
                && game_with_data.game.active_player == 1)
                || (env::predecessor_account_id() == game_with_data.second_player
                    && game_with_data.game.active_player == 2),
            "Wrong player's turn! "
        );

        let from = GameCubePosition::from(from_x, from_y);
        assert!(
            from.is_some(),
            "Illegal move attempt from ({},{}). Coordinates must be in range 1..7",
            from_x,
            from_y
        );

        let to = GameCubePosition::from(to_x, to_y);
        assert!(
            to.is_some(),
            "Illegal move attempt to ({},{}). Coordinates must be in range 1..7",
            to_x,
            to_y
        );
        let from = from.unwrap();
        let to = to.unwrap();

        let move_report = game_with_data.game.try_make_move(from, to);

        log!("{}", move_report);
        log!("{}", game_with_data.game.format_board());

        //NOTE: as a result of the move a game can end!
        // That means the all contract is finished, and rewards must be processed.

        if game_with_data.game.phase == GamePhase::End {
            game_with_data.is_finished = true;
            //TODO: stream shutdown logic takes place here...
        };

        self.games.replace(index, &game_with_data);

        game_with_data
    }
}

/// Tests for contract.
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, AccountId, VMContext};

    // fn get_context(is_view: bool) -> VMContext {
    //     VMContextBuilder::new().is_view(is_view).build()
    // }
    fn get_context(account: AccountId) -> near_sdk::VMContext {
        VMContextBuilder::new()
            .predecessor_account_id(account)
            .build()
    }

    #[test]
    fn test_new_get() {
        testing_env!(get_context(accounts(4)));

        let mut contract = Contract::new();

        contract.create_game(accounts(1), accounts(2), Some(3));
        contract.create_game(accounts(4), accounts(3), Some(4));

        let game_state = contract.game_state(0);
        assert!(game_state.is_some());
        assert_eq!(game_state.unwrap().first_player, accounts(1));
        let game_state = contract.game_state(1);
        assert!(game_state.is_some());
        assert_eq!(game_state.as_ref().unwrap().second_player, accounts(3));
        println!("{}", game_state.unwrap().game.format_board());
        let cube = contract.cube_state(1, 3, 1);
        assert!(cube.is_some());
    }

    #[test]
    fn test_pass_call() {
        testing_env!(get_context(accounts(4)));

        let mut contract = Contract::new();

        contract.create_game(accounts(1), accounts(3), Some(2));

        testing_env!(get_context(accounts(1)));
        contract.pass_move(0);
        testing_env!(get_context(accounts(3)));
        contract.pass_move(0);
        testing_env!(get_context(accounts(1)));
        contract.pass_move(0);
        assert_eq!(contract.games.get(0).unwrap().game.active_player, 2);
    }

    #[test]
    fn test_make_move_call() {
        testing_env!(get_context(accounts(4)));

        let mut contract = Contract::new();

        contract.create_game(accounts(1), accounts(3), Some(3));
        contract.create_game(accounts(4), accounts(2), Some(4));

        testing_env!(get_context(accounts(4)));
        contract.make_move(1, 5, 5, 5, 5);

        testing_env!(get_context(accounts(4)));
        contract.make_move(1, 5, 5, 4, 5);

        testing_env!(get_context(accounts(2)));
        contract.make_move(1, 3, 3, 3, 4);
        contract.make_move(1, 3, 4, 3, 7);
        contract.make_move(1, 3, 4, 3, 6);

        testing_env!(get_context(accounts(4)));
        contract.make_move(1, 3, 7, 4, 7);
        contract.make_move(1, 4, 7, 4, 6);
        contract.make_move(1, 4, 7, 3, 7);

        testing_env!(get_context(accounts(2)));
        contract.make_move(1, 3, 6, 4, 6);

        println!("{}", contract.game_state(1).unwrap().game.format_board());
        println!("{}", contract.game_state(0).unwrap().game.format_board());
    }

    #[test]
    fn test_finish_game() {
        testing_env!(get_context(accounts(1)));

        let mut contract = Contract::new();

        let game_with_data = GameWithData {
            first_player: accounts(1),
            second_player: accounts(2),
            is_finished: false,
            game: Game {
                phase: GamePhase::Roll,
                active_player: 2,
                board: vec![
                    GameCube {
                        player: 1,
                        position: GameCubePosition { x: 4, y: 7 },
                        direction: GameCubeDirection {
                            up: 2,
                            front: 3,
                            right: 6,
                        },
                    },
                    GameCube {
                        player: 2,
                        position: GameCubePosition { x: 3, y: 6 },
                        direction: GameCubeDirection {
                            up: 2,
                            front: 3,
                            right: 6,
                        },
                    },
                ],
            },
        };

        contract.games.push(&game_with_data);
        testing_env!(get_context(accounts(2)));
        contract.make_move(0, 3, 6, 4, 6);
        println!("{}", contract.game_state(0).unwrap().game.format_board());
        assert!(contract.games.get(0).unwrap().is_finished);
    }
}
