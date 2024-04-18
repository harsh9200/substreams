use substreams::store::StoreNew;
use substreams::store::StoreSetIfNotExists;
use substreams::store::StoreSetIfNotExistsString;

use crate::pb::contract::v1::Pools;

#[substreams::handlers::store]
pub fn store_pools(pools_created: Pools, store: StoreSetIfNotExistsString) {
    for pool in pools_created.pools {
        store.set_if_not_exists(0, &pool.address, &pool.address);
    }
}
