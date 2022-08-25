#![allow(unused_imports)]

pub mod direction;
mod external;
pub mod face;
pub mod formatter;
pub mod game;
pub mod game_setup;
mod interface;
pub mod move_pattern;
pub mod player;
pub mod position;
mod views;

use std::collections::HashMap;

use external::{
    streaming_roketo::streaming_roketo::{self, StreamingRoketoExt},
    token::token,
};

use direction::GameCubeDirection;
use game::{Game, GameCube, GamePhase};
use near_sdk::json_types::U128;
use player::{GamePlayerIndex, Player};
use position::GameCubePosition;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::Value;
use near_sdk::{
    env, log, near_bindgen, require, AccountId, BorshStorageKey, Gas, PanicOnDefault, Promise,
    PromiseError, PromiseOrValue,
};

use crate::external::TGAS;

/// Contract state definition.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]

pub struct Contract {
    game: Option<Game>,
    is_finished: bool,
    first: Option<Player>,
    second: Option<Player>,
    token_id: Option<AccountId>,
    deposit: u128,
    tokens_per_sec: String,
    streaming_id: Option<AccountId>,
    num_cubes: Option<usize>,
}

/// Contract functions implementations.
#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn new(num_cubes: Option<usize>) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            game: None,
            is_finished: false,
            first: None,
            second: None,
            deposit: 0,
            streaming_id: None,
            token_id: None,
            tokens_per_sec: String::new(),
            num_cubes,
        }
    }

    pub fn reset(&mut self, num_cubes: Option<usize>) {
        *self = Self {
            game: None,
            is_finished: false,
            first: None,
            second: None,
            deposit: 0,
            token_id: None,
            streaming_id: None,
            tokens_per_sec: String::new(),
            num_cubes,
        };
    }

    fn streaming_id(&self) -> &AccountId {
        self.streaming_id
            .as_ref()
            .expect("streaming id should be connected by now")
    }

    fn first_player(&self) -> &Player {
        self.first.as_ref().expect("first player is not registered")
    }

    fn second_player(&self) -> &Player {
        self.second
            .as_ref()
            .expect("second player is not registered")
    }

    pub fn connect_streaming_contract(&mut self, streaming_id: AccountId) {
        assert!(
            self.streaming_id.is_none(),
            "streaming contract is already connected"
        );
        self.streaming_id = Some(streaming_id);
    }

    fn register_first_player(
        &mut self,
        account: AccountId,
        token_id: AccountId,
        deposit: U128,
        tokens_per_sec: String,
    ) -> PromiseOrValue<U128> {
        assert!(self.token_id.is_none(), "somehow token ID is already set");

        log!("game token set to: {}", token_id);
        self.token_id = Some(token_id);

        log!("deposit set to: {}", deposit.0);
        self.deposit = deposit.0;

        log!("tokens streaming rate set to: {}/sec", tokens_per_sec);
        self.tokens_per_sec = tokens_per_sec;

        log!("first player registered: {} ", account,);
        self.first = Some(Player::new(account, deposit, 1));
        PromiseOrValue::Value(U128::from(0))
    }

    pub fn start(&mut self) -> Promise {
        let first_player_stream = self
            .first_player()
            .stream()
            .expect("first players stream was not registered")
            .clone();
        let second_player_stream = self
            .second_player()
            .stream()
            .expect("second players stream was not registered")
            .clone();

        let streaming_id = self.streaming_id().clone();

        log!(
            "first stream: {}, second: {}",
            first_player_stream,
            second_player_stream
        );
        let start_first_player_stream =
            start_stream(streaming_id.clone(), first_player_stream.clone());
        let pause_first_player_stream = pause_stream(streaming_id.clone(), first_player_stream);
        let start_second_player_stream = start_stream(streaming_id, second_player_stream);

        start_first_player_stream
            .then(pause_first_player_stream)
            .then(start_second_player_stream)
    }

    pub fn game_state(&self) -> Option<Game> {
        let game = &self.game;
        if game.is_some() {
            log!("Game board:");
            log!("{}", game.as_ref().unwrap().format_board());
        }
        game.clone()
    }

    pub fn cube_state(&self, x: i8, y: i8) -> Option<GameCube> {
        let game = &self.game;
        require!(game.is_some(), "Game is not started!");

        let pos: Option<GameCubePosition> = GameCubePosition::from(x, y);
        match pos {
            Some(pos) => {
                let cube = game.as_ref().unwrap().get_cube_at(pos);
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

    pub fn pass_move(&mut self) -> Option<Promise> {
        require!(self.game.is_some(), "Game is not started!");
        let mut game = self.game.as_ref().unwrap().clone();

        let current = env::signer_account_id();
        assert!(
            (current == *self.first.as_ref().unwrap().account() && game.active_player == 1)
                || (current == *self.second.as_ref().unwrap().account() && game.active_player == 2),
            "Wrong player's turn! "
        );

        let phase = game.phase;
        let active_player_ind = game.active_player;
        let other_player_ind = 3 - active_player_ind;

        match phase {
            GamePhase::End => {
                log!("Game is finished, no moves allowed");
                None
            }
            _ => {
                game.phase = GamePhase::Roll;
                game.active_player = other_player_ind;
                self.game = Some(game);
                log!(
                    "Player {} passed the turn. It is player {} Roll phase.",
                    active_player_ind,
                    other_player_ind
                );
                match other_player_ind {
                    // 1 => Some(self.check_winner(self.first_player(), self.second_player())),
                    // 2 => Some(self.check_winner(self.second_player(), self.first_player())),
                    1 => Some(self.check_winner(self.second_player(), self.first_player())),
                    2 => Some(self.check_winner(self.first_player(), self.second_player())),
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn make_move(&mut self, from_x: i8, from_y: i8, to_x: i8, to_y: i8) -> Option<Promise> {
        require!(self.game.is_some(), "Game is not started!");
        let mut game = self.game.as_ref().unwrap().clone();

        let current = env::signer_account_id();
        let active_before = game.active_player;
        assert!(
            (current == *self.first.as_ref().unwrap().account() && game.active_player == 1)
                || (current == *self.second.as_ref().unwrap().account() && game.active_player == 2),
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

        let move_report = game.try_make_move(from, to);

        log!("{}", move_report);
        log!("{}", game.format_board());

        let active_after = game.active_player;
        //NOTE: as a result of the move a game can end!
        if game.phase == GamePhase::End {
            self.is_finished = true;
        };

        self.game = Some(game.clone());

        if (active_after != active_before) || self.is_finished {
            match active_after {
                // 1 => Some(self.check_winner(self.first_player(), self.second_player())),
                // 2 => Some(self.check_winner(self.second_player(), self.first_player())),
                1 => Some(self.check_winner(self.second_player(), self.first_player())),
                2 => Some(self.check_winner(self.first_player(), self.second_player())),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }

    fn check_winner(&self, active: &Player, passive: &Player) -> Promise {
        require!(self.game.is_some(), "Game is not started!");
        let game = self.game.as_ref().unwrap().clone();
        let streaming_id = self.streaming_id.as_ref().unwrap();
        let current_id = env::current_account_id();

        match game.phase {
            GamePhase::End => {
                log!("player {} WON!", active.account());
                let promise = stop_stream(streaming_id.clone(), active.stream().unwrap().clone());
                let promise = promise.then(stop_stream(
                    streaming_id.clone(),
                    passive.stream().unwrap().clone(),
                ));
                let promise = promise.then(get_stream(
                    streaming_id.clone(),
                    active.stream().unwrap().clone(),
                ));
                let promise = promise.then(
                    Self::ext(current_id)
                        .query_transferred_tokens_callback(active.account().clone()),
                );
                promise
            }

            _ => pause_stream(streaming_id.clone(), passive.stream().unwrap().clone()).then(
                start_stream(streaming_id.clone(), active.stream().unwrap().clone()),
            ),
        }
    }

    #[private]
    pub fn query_transferred_tokens_callback(
        &mut self,
        #[callback_result] call_result: Result<HashMap<String, Value>, PromiseError>,
        player_id: AccountId,
    ) -> Promise {
        let res = call_result.unwrap();
        log!("res: {:?}", res);
        let withdrawn: u128 = res
            .get("tokens_total_withdrawn")
            .and_then(|v| v.as_str())
            .expect("unexpected response from roke.to contract")
            .parse()
            .expect("couldn't parse tokens amount in roke.to response");
        let win_money = if self.first_player().account() == &player_id {
            self.first_player().deposit().0
        } else if self.second_player().account() == &player_id {
            self.second_player().deposit().0
        } else {
            unreachable!();
        } - withdrawn;
        let win_money = win_money / 100 * 90;
        log!("reward {} tokens (90%) to {}", win_money, player_id);
        token::ext(self.token_id.as_ref().unwrap().clone())
            .with_attached_deposit(1)
            .ft_transfer_call(
                player_id,
                U128::from(win_money),
                String::new(),
                String::new(),
            )
    }

    #[private]
    pub fn query_stream_id_callback(
        &mut self,
        #[callback_result] call_result: Result<HashMap<String, Value>, PromiseError>,
        player_id: AccountId,
    ) -> U128 {
        let res = call_result.unwrap();
        let id = res.get("last_created_stream").unwrap().as_str().unwrap();
        log!("[{}] stream id: {}", player_id, id);

        let first = self.first.as_mut().unwrap();
        let second = self.second.as_mut().unwrap();
        if first.account() == &player_id {
            first.stream = Some(id.to_string());
        } else if second.account() == &player_id {
            second.stream = Some(id.to_string());
        } else {
            panic!("unknown player ID");
        }
        U128(0)
    }
}

fn streaming(streaming_id: AccountId) -> StreamingRoketoExt {
    streaming_roketo::ext(streaming_id)
        .with_attached_deposit(1)
        .with_static_gas(Gas(60 * TGAS))
}

fn start_stream(streaming_id: AccountId, stream_id: String) -> Promise {
    streaming(streaming_id).start_stream(stream_id)
}

fn pause_stream(streaming_id: AccountId, stream_id: String) -> Promise {
    streaming(streaming_id).pause_stream(stream_id)
}

fn stop_stream(streaming_id: AccountId, stream_id: String) -> Promise {
    streaming(streaming_id).stop_stream(stream_id)
}

fn get_stream(streaming_id: AccountId, stream_id: String) -> Promise {
    streaming(streaming_id).get_stream(stream_id)
}

impl Default for Contract {
    fn default() -> Self {
        Self::new(None)
    }
}
