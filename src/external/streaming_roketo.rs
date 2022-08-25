use near_sdk::{ext_contract, AccountId};

#[ext_contract(streaming_roketo)]
trait StreamingRoketo {
    fn get_account(account_id: AccountId) -> String;

    fn pause_stream(stream_id: String);

    fn start_stream(stream_id: String);

    fn stop_stream(stream_id: String);

    fn get_stream(stream_id: String);
}
