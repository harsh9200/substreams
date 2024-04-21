use substreams::log;
use substreams::scalar::BigInt;
use substreams::store::{DeltaBigInt, Deltas, StoreGet, StoreGetBigInt};
use substreams_ethereum::pb::eth::rpc::RpcResponse;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::rpc::RpcBatch;
use substreams_helper::hex::Hexable;

use crate::abi::pool_contract::functions::{AdminFee, Fee};
use crate::pb::contract::v1::{PoolFee, PoolFees};
use crate::store_key::StoreKey;

#[substreams::handlers::map]
pub fn map_pool_fees(
    blk: eth::Block,
    admin_balances: StoreGetBigInt,
    admin_balance_deltas: Deltas<DeltaBigInt>,
) -> Result<PoolFees, substreams::errors::Error> {
    let mut fees: Vec<PoolFee> = vec![];

    for calls in blk.calls() {
        let address = calls.call.address.clone();

        if admin_balances
            .get_last(StoreKey::Pool.get_unique_key(&address.to_hex()))
            .is_none()
        {
            continue;
        }

        let admin_balances = get_admin_balances(
            &admin_balance_deltas,
            &address.to_hex(),
            calls.call.end_ordinal,
        );

        if admin_balances.iter().any(|x| x < &BigInt::zero())
            || admin_balances.iter().all(|x| x == &BigInt::zero())
        {
            log::debug!("Admin Balances unchanged.");
            continue;
        }

        let pool_fees = get_pool_fees(address.clone());

        fees.push(PoolFee {
            address: address.clone().to_hex(),

            fee: RpcBatch::decode::<_, Fee>(&pool_fees[0]).map(|x| x.to_u64()),
            admin_fee: RpcBatch::decode::<_, Fee>(&pool_fees[1]).map(|x| x.to_u64()),

            admin_balances: admin_balances.iter().map(|x| x.to_string()).collect(),

            log_index: 0 as i64,
            block_number: blk.number as i64,
            block_timestmap: Some(blk.timestamp().to_owned()),

            transaction_index: calls.transaction.index as i64,
            transaction_hash: calls.transaction.hash.to_hex(),
        })
    }

    Ok(PoolFees { fees })
}

fn get_pool_fees(contract: Vec<u8>) -> Vec<RpcResponse> {
    let batch = RpcBatch::new();

    let responses = batch
        .add(Fee {}, contract.clone())
        .add(AdminFee {}, contract.clone())
        .execute()
        .unwrap()
        .responses;

    responses
}

fn get_admin_balances(
    admin_balance_deltas: &Deltas<DeltaBigInt>,
    address: &String,
    ordinal: u64,
) -> Vec<BigInt> {
    let mut balances = vec![BigInt::zero(); 4];

    for delta in admin_balance_deltas.deltas.iter() {
        match delta {
            ref x if x.key == StoreKey::AdminBalanceToken0.get_unique_key(address) => {
                balances[0] = delta.new_value.clone() - delta.old_value.clone();
            }
            ref x if x.key == StoreKey::AdminBalanceToken1.get_unique_key(address) => {
                balances[1] = delta.new_value.clone() - delta.old_value.clone();
            }
            ref x if x.key == StoreKey::AdminBalanceToken2.get_unique_key(address) => {
                balances[2] = delta.new_value.clone() - delta.old_value.clone();
            }
            ref x if x.key == StoreKey::AdminBalanceToken3.get_unique_key(address) => {
                balances[3] = delta.new_value.clone() - delta.old_value.clone();
            }

            _ => {}
        }
    }

    balances
}
