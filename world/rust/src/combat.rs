use log::debug;
use screeps::{
    enums::StructureObject,
    find, game, prelude::*,
};

pub fn run() {
    debug!("running combat");

    let room = game::rooms().values().next().expect("room not found");

    for structure in room.find(find::STRUCTURES, None).iter() {
        if let StructureObject::StructureTower(tower) = structure {
            let target = tower.pos().find_closest_by_range(find::HOSTILE_CREEPS);
                // .or_else(|| tower.pos().find_closest_by_range(find::HOSTILE_STRUCTURES));

            if let Some(target) = target {
                let _ = tower.attack(&target);
            }
        }
    }
}