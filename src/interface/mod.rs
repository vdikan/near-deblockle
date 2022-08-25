use near_sdk::serde::Serialize;
use near_sdk::AccountId;

mod ft_receiver;

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
struct RoketoStreamingCreateRequest {
    balance: String,
    owner_id: AccountId,
    receiver_id: AccountId,
    token_name: AccountId,
    tokens_per_sec: String,
    is_locked: bool,
    is_auto_start_enabled: bool,
    description: String,
}
