use crate::EntityEnum;

pub mod levelstore;

extern crate alloc;
use alloc::vec::Vec;
use serde_json_core::from_slice;

pub const LEVELSIZE: usize = 30;

#[inline(never)]
pub fn load_level(level: usize, entity_array: &mut [EntityEnum]) -> usize{
    let message_bytes = levelstore::LEVELS[level].trim().as_bytes();
    //let (parsed_entities, _): ([EntityEnum; levelstore::LEVELSIZE], _) = from_slice(message_bytes).unwrap();
    let (parsed_entities, _): (Vec<EntityEnum>, usize) = from_slice(message_bytes).unwrap();

    for i in 0..parsed_entities.len() {
        entity_array[i + 2] = parsed_entities[i];
        entity_array[i + 2].set_id(i as i16);
        entity_array[i + 2].reload_rotation_matrices();
        entity_array[i + 2].recalculate_points();
        entity_array[i + 2].refresh_model_matrix();
    }
	return parsed_entities.len();
}
