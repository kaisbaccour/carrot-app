mod common;

use crate::common::{create_position, setup_test_tube, USDC, USDT};
use carrot_app::msg::{AppExecuteMsgFns, AppQueryMsgFns, AssetsBalanceResponse, PositionResponse};
use cosmwasm_std::{coin, coins, Decimal, Uint128};
use cw_orch::{
    anyhow,
    osmosis_test_tube::osmosis_test_tube::{
        osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgWithdrawPosition,
        ConcentratedLiquidity, Module,
    },
    prelude::*,
};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::PositionByIdRequest;

#[test]
fn deposit_lands() -> anyhow::Result<()> {
    let (_, carrot_app) = setup_test_tube(false)?;

    let deposit_amount = 5_000;
    let max_fee = Uint128::new(deposit_amount).mul_floor(Decimal::percent(3));
    // Create position
    create_position(
        &carrot_app,
        coins(deposit_amount, USDT.to_owned()),
        coin(1_000_000, USDT.to_owned()),
        coin(1_000_000, USDC.to_owned()),
    )?;
    // Check almost everything landed
    let balance: AssetsBalanceResponse = carrot_app.balance()?;
    let sum = balance
        .balances
        .iter()
        .fold(Uint128::zero(), |acc, e| acc + e.amount);
    assert!(sum.u128() > deposit_amount - max_fee.u128());

    // Do the deposit
    carrot_app.deposit(vec![coin(deposit_amount, USDT.to_owned())])?;
    // Check almost everything landed
    let balance: AssetsBalanceResponse = carrot_app.balance()?;
    let sum = balance
        .balances
        .iter()
        .fold(Uint128::zero(), |acc, e| acc + e.amount);
    assert!(sum.u128() > (deposit_amount - max_fee.u128()) * 2);

    // Do the second deposit
    carrot_app.deposit(vec![coin(deposit_amount, USDT.to_owned())])?;
    // Check almost everything landed
    let balance: AssetsBalanceResponse = carrot_app.balance()?;
    let sum = balance
        .balances
        .iter()
        .fold(Uint128::zero(), |acc, e| acc + e.amount);
    dbg!(sum);
    assert!(sum.u128() > (deposit_amount - max_fee.u128()) * 3);
    Ok(())
}

#[test]
fn withdraw_position() -> anyhow::Result<()> {
    let (_, carrot_app) = setup_test_tube(false)?;

    let chain = carrot_app.get_chain().clone();

    // Create position
    create_position(
        &carrot_app,
        coins(10_000, USDT.to_owned()),
        coin(1_000_000, USDT.to_owned()),
        coin(1_000_000, USDC.to_owned()),
    )?;

    let balance: AssetsBalanceResponse = carrot_app.balance()?;
    let balance_usdc_before_withdraw = chain
        .bank_querier()
        .balance(chain.sender(), Some(USDT.to_owned()))?
        .pop()
        .unwrap();
    let balance_usdt_before_withdraw = chain
        .bank_querier()
        .balance(chain.sender(), Some(USDC.to_owned()))?
        .pop()
        .unwrap();

    // Withdraw half of liquidity
    let liquidity_amount: Uint128 = balance.liquidity.parse().unwrap();
    let half_of_liquidity = liquidity_amount / Uint128::new(2);
    carrot_app.withdraw(half_of_liquidity)?;

    let balance_usdc_after_half_withdraw = chain
        .bank_querier()
        .balance(chain.sender(), Some(USDT.to_owned()))?
        .pop()
        .unwrap();
    let balance_usdt_after_half_withdraw = chain
        .bank_querier()
        .balance(chain.sender(), Some(USDC.to_owned()))?
        .pop()
        .unwrap();

    assert!(balance_usdc_after_half_withdraw.amount > balance_usdc_before_withdraw.amount);
    assert!(balance_usdt_after_half_withdraw.amount > balance_usdt_before_withdraw.amount);

    // Withdraw rest of liquidity
    carrot_app.withdraw_all()?;
    let balance_usdc_after_full_withdraw = chain
        .bank_querier()
        .balance(chain.sender(), Some(USDT.to_owned()))?
        .pop()
        .unwrap();
    let balance_usdt_after_full_withdraw = chain
        .bank_querier()
        .balance(chain.sender(), Some(USDC.to_owned()))?
        .pop()
        .unwrap();

    assert!(balance_usdc_after_full_withdraw.amount > balance_usdc_after_half_withdraw.amount);
    assert!(balance_usdt_after_full_withdraw.amount > balance_usdt_after_half_withdraw.amount);
    Ok(())
}

#[test]
fn deposit_both_assets() -> anyhow::Result<()> {
    let (_, carrot_app) = setup_test_tube(false)?;

    // Create position
    create_position(
        &carrot_app,
        coins(10_000, USDT.to_owned()),
        coin(1_000_000, USDT.to_owned()),
        coin(1_000_000, USDC.to_owned()),
    )?;

    carrot_app.deposit(vec![coin(258, USDT.to_owned()), coin(234, USDC.to_owned())])?;

    Ok(())
}
#[test]
fn create_position_on_instantiation() -> anyhow::Result<()> {
    let (_, carrot_app) = setup_test_tube(true)?;

    let position: PositionResponse = carrot_app.position()?;
    assert!(position.position.is_some());
    Ok(())
}

#[test]
fn withdraw_after_user_withdraw_liquidity_manually() -> anyhow::Result<()> {
    let (_, carrot_app) = setup_test_tube(true)?;
    let chain = carrot_app.get_chain().clone();

    let position: PositionResponse = carrot_app.position()?;

    let test_tube = chain.app.borrow();
    let cl = ConcentratedLiquidity::new(&*test_tube);
    let position_breakdown = cl
        .query_position_by_id(&PositionByIdRequest {
            position_id: position.position.unwrap().position_id,
        })?
        .position
        .unwrap();
    let position = position_breakdown.position.unwrap();

    cl.withdraw_position(
        MsgWithdrawPosition {
            position_id: position.position_id,
            sender: chain.sender().to_string(),
            liquidity_amount: position.liquidity,
        },
        &chain.sender,
    )?;

    // Ensure it errors
    carrot_app.withdraw_all().unwrap_err();
    Ok(())
}
