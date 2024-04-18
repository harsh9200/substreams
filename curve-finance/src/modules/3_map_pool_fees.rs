use substreams::log;
use substreams::scalar::BigInt;
use substreams::store::{StoreGet, StoreGetString};
use substreams_ethereum::pb::eth::rpc::RpcResponse;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_ethereum::rpc::RpcBatch;
use substreams_helper::hex::Hexable;

use crate::abi::pool_contract::functions::{AdminFee, Fee};
use crate::pb::contract::v1::{PoolFee, PoolFees};

#[substreams::handlers::map]
pub fn map_pool_fees(
    blk: eth::Block,
    pools_store: StoreGetString,
) -> Result<PoolFees, substreams::errors::Error> {
    let mut fees: Vec<PoolFee> = vec![];

    for log in blk.logs() {
        if pools_store
            .get_last(log.address().to_owned().to_hex())
            .is_none()
        {
            continue;
        }

        let pool_fees = get_pool_fees(log.address().to_vec());

        fees.push(PoolFee {
            address: log.address().to_owned().to_hex(),

            fee: RpcBatch::decode::<_, Fee>(&pool_fees[0]).map(|x| x.to_u64()),
            admin_fee: RpcBatch::decode::<_, Fee>(&pool_fees[1]).map(|x| x.to_u64()),
            // admin_balances: admin_balances,
            
            log_index: log.index() as i64,
            block_number: blk.number as i64,
            block_timestmap: Some(blk.timestamp().to_owned()),
            
            transaction_index: log.receipt.transaction.index as i64,
            transaction_hash: log.receipt.transaction.hash.to_hex(),
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

    // let fee = RpcBatch::decode::<_, Fee>(&responses[0]);
    // let admin_fee = RpcBatch::decode::<_, AdminFee>(&responses[1]);

    responses
}
