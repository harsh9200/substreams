use substreams::store::StoreSetProto;
use substreams::store::{StoreNew, StoreSet};

use crate::pb::ens::v1 as ENS;

#[substreams::handlers::store]
pub fn store_ens_record(map_domains: ENS::Domains, output: StoreSetProto<ENS::Domain>) {
    for domain in map_domains.items {
        output.set(0, format!("Domain:{}", domain.ens_name), &domain);
    }
}
