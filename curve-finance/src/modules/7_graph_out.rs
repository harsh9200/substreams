use substreams_entity_change::pb::entity::{EntityChange, EntityChanges};

#[substreams::handlers::map]
pub fn graph_out(fees_entity_map: EntityChanges) -> Result<EntityChanges, ()> {
    let mut entity_changes: Vec<EntityChange> = vec![];

    entity_changes.extend(fees_entity_map.entity_changes);

    Ok(EntityChanges { entity_changes })
}
