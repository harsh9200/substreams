use std::ops::Mul;
use std::str::FromStr;

use substreams::scalar::{BigDecimal, BigInt};
use substreams::store::{StoreGet, StoreGetBigDecimal, StoreGetProto};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChange, EntityChanges};

use crate::common::constants;
use crate::pb::erc20::v1::Erc20Token;
use crate::pb::uniswap::v2::event::Type::{DepositType, SwapType, WithdrawType};
use crate::pb::uniswap::v2::{DepositEvent, Event, Events, Pool, SwapEvent, WithdrawEvent};
use crate::store_key::StoreKey;
use crate::utils;

#[substreams::handlers::map]
pub fn map_events_entity(
    pool_events_map: Events,
    pool_store: StoreGetProto<Pool>,
) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    for event in pool_events_map.events {
        let ordinal = event.log_ordinal as u64;

        let pool =
            pool_store.must_get_last(StoreKey::Pool.get_unique_pool_key(&event.clone().pool));

        match event.clone().r#type.unwrap() {
            SwapType(swap) => entity_changes.push(create_swap_transaction(
                ordinal,
                &event,
                &swap,
            )),
            _ => {}
        }
    }

    Ok(EntityChanges { entity_changes })
}

fn get_event_id(event: &Event) -> String {
    [event.hash.clone(), event.log_index.to_string()].join("-")
}

fn create_swap_transaction(
    ordinal: u64,
    event: &Event,
    swap: &SwapEvent,
) -> EntityChange {
    let id = get_event_id(event);

    let mut swap_entity_change: EntityChange =
        EntityChange::new("Swap", id.as_str(), ordinal, Operation::Create);

    let token_in = swap.token_in.clone().unwrap();
    let token_out = swap.token_out.clone().unwrap();

    let amount_in = BigInt::from_str(swap.amount_in.as_str()).unwrap();
    let amount_out = BigInt::from_str(swap.amount_out.as_str()).unwrap();

    swap_entity_change
        .change("id", id)
        .change("hash", event.hash.clone())
        .change("logIndex", event.log_index as i32)
        .change("protocol", constants::UNISWAP_V2_FACTORY.to_string())
        .change("to", event.to.clone())
        .change("from", event.from.clone())
        .change("blockNumber", BigInt::from(event.block_number))
        .change("timestamp", BigInt::from(event.timestamp))
        .change("tokenIn", token_in.address)
        .change("amountIn", amount_in)
        .change("amountInUSD", BigDecimal::zero())
        .change("tokenOut", token_out.address)
        .change("amountOut", amount_out)
        .change("amountOutUSD", BigDecimal::zero())
        .change("pool", event.pool.clone());

    swap_entity_change
}
