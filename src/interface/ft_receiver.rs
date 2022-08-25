use std::collections::HashMap;

use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    env, json_types::U128, log, near_bindgen, serde_json, AccountId, Gas, PromiseOrValue,
};

use crate::{
    external::{streaming_roketo::streaming_roketo, token::token, TGAS},
    game::Game,
    interface::RoketoStreamingCreateRequest,
    player::Player,
    Contract, ContractExt,
};

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let token_id = env::predecessor_account_id();
        log!(
            "received {:?}[{}] tokens from {} with: {}",
            amount,
            token_id,
            sender_id,
            msg
        );

        assert!(self.game.is_none(), "Game already started");

        if self.first.is_none() {
            let msg_values: HashMap<String, String> = serde_json::from_str(&msg).unwrap();
            let tokens_per_sec = msg_values
                .get(&"tokens_per_sec".to_string())
                .expect("entry required in msg, eg: {{ \"tokens_per_sec\": \"string\" }}")
                .clone();

            self.register_first_player(sender_id, token_id, amount, tokens_per_sec)
        } else if self.second.is_none() {
            assert!(
                self.token_id
                    .as_ref()
                    .expect("somehow token ID is NOT set yet")
                    == &token_id,
                "wrong token id"
            );

            assert!(
                self.first_player().deposit() == amount,
                "deposit should be: {amount:?}"
            );
            log!(
                "second player registered: {} with deposit: {:?}, game started!",
                sender_id,
                amount
            );
            self.second = Some(Player::new(sender_id, amount, 2));
            let first = self.first_player();

            log!("create stream for first player");
            let streaming_id = self.streaming_id();

            let memo = format!("Roketo transfer: {}", first.account());
            let current_account = env::current_account_id();

            let mut request = RoketoStreamingCreateRequest {
                balance: self.deposit.to_string(),
                owner_id: current_account.clone(),
                receiver_id: first.account().clone(),
                token_name: token_id.clone(),
                tokens_per_sec: self.tokens_per_sec.clone(),
                is_locked: false,
                is_auto_start_enabled: false,
                description: "{\"player\": \"first\"}".to_string(),
            };
            let request_json =
                serde_json::to_string(&request).expect("failed to serialize request");
            let msg = format!("{{\"Create\": {{ \"request\": {request_json} }}}}");

            let first_player_deposit_to_stream = token::ext(token_id.clone())
                .with_static_gas(Gas(60 * TGAS))
                .with_attached_deposit(1)
                .ft_transfer_call(streaming_id.clone(), amount, memo, msg);

            let first_player_get_stream_account =
                streaming_roketo::ext(streaming_id.clone()).get_account(current_account.clone());

            let first_account_id = first.account().clone();
            let first_player_query_stream_id =
                Self::ext(current_account.clone()).query_stream_id_callback(first_account_id);

            let second = self.second_player();
            let memo = format!("Roketo transfer: {}", second.account());
            request.receiver_id = second.account().clone();
            request.description = "{\"player\": \"second\"}".to_string();
            let request_json =
                serde_json::to_string(&request).expect("failed to serialize request");
            let msg = format!("{{\"Create\": {{ \"request\": {request_json} }}}}");

            let second_player_deposit_to_stream = token::ext(token_id)
                .with_static_gas(Gas(60 * TGAS))
                .with_attached_deposit(1)
                .ft_transfer_call(streaming_id.clone(), amount, memo, msg);

            let second_player_get_stream_account =
                streaming_roketo::ext(streaming_id.clone()).get_account(current_account.clone());

            let second_account_id = second.account().clone();
            let second_player_query_stream_id =
                Self::ext(current_account).query_stream_id_callback(second_account_id);

            self.game = Some(Game::game_setup(self.num_cubes));

            let promise = first_player_deposit_to_stream
                .then(first_player_get_stream_account)
                .then(first_player_query_stream_id)
                .then(second_player_deposit_to_stream)
                .then(second_player_get_stream_account)
                .then(second_player_query_stream_id);
            PromiseOrValue::Promise(promise)
        } else {
            panic!("all players are in, registration closed");
        }
    }
}
