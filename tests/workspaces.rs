use near_sdk::json_types::U128;
use near_sdk::ONE_YOCTO;
use near_units::parse_near;
use workspaces::prelude::*;
use workspaces::{Account, AccountId, Contract, DevNetwork, Network, Worker};
use near_deblockle::GameWithData;

async fn init(worker: &Worker<impl DevNetwork>) -> anyhow::Result<(Contract, Account, Account)> {
    let d_contract = worker
        .dev_deploy(&include_bytes!("../out/main.wasm").to_vec())
        .await?;

    let alice = d_contract
        .as_account()
        .create_subaccount(worker, "alice")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    let bob = d_contract
        .as_account()
        .create_subaccount(worker, "bob")
        .initial_balance(parse_near!("10 N"))
        .transact()
        .await?
        .into_result()?;

    let res = d_contract
        .call(worker, "new")
        .gas(100_000_000_000_000)
        .transact()
        .await?;
    assert!(res.is_success());

    Ok((d_contract, alice, bob))
}

#[tokio::test]
async fn test_games_are_created() -> anyhow::Result<()> {
    let initial_balance = U128::from(parse_near!("10000 N"));
    let worker = workspaces::sandbox().await?;
    let (contract, alice, bob) = init(&worker).await?;

    let res = contract
        .call(&worker, "create_game")
        .args_json((alice.id(), bob.id(), Option::<usize>::None))?
        .transact()
        .await?;
    assert!(res.is_success());

    let res = contract
        .call(&worker, "create_game")
        .args_json((alice.id(), bob.id(), Option::<usize>::None))?
        .transact()
        .await?;
    assert!(res.is_success());
    assert_eq!(res.json::<i64>()?, 1);

    let res = contract
        .call(&worker, "game_state")
        .args_json((1,))?
        .transact()
        .await?
        .json::<GameWithData>()?;
    assert_eq!(res.first_player.as_str(), alice.id().as_str());

    Ok(())
}
