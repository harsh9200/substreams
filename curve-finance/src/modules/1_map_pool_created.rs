use std::str::FromStr;

use ethabi::ethereum_types::Address;
use substreams_ethereum::pb::eth::v2::{self as eth};
use substreams_helper::event_handler::EventHandler;
use substreams_helper::hex::Hexable;

use crate::abi::registry_contract::events::PoolAdded;
use crate::common::constants;
use crate::pb::contract::v1::{Pool, Pools};

#[substreams::handlers::map]
pub fn map_pool_created(blk: eth::Block) -> Result<Pools, substreams::errors::Error> {
    let mut pools: Vec<Pool> = vec![];

    get_pools(&blk, &mut pools);
    Ok(Pools { pools })
}

fn get_pools(blk: &eth::Block, pools: &mut Vec<Pool>) {
    let mut on_pair_created = |event: PoolAdded, tx: &eth::TransactionTrace, log: &eth::Log| {
        pools.push(Pool {
            address: event.pool.to_hex(),
            log_index: log.index as i64,
            block_number: blk.number as i64,
            block_timestmap: Some(blk.timestamp().to_owned()),
            transaction_index: tx.index as i64,
            transaction_hash: tx.hash.to_hex(),
        })
    };

    let mut eh = EventHandler::new(blk);
    eh.filter_by_address(vec![Address::from_str(constants::REGISTRY_ADDRESS).unwrap()]);

    eh.on::<PoolAdded, _>(&mut on_pair_created);
    eh.handle_events();
}
