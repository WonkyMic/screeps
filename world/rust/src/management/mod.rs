use crate::CreepTarget;
use std::collections::{hash_map::Entry, HashMap};
use log::debug;
use screeps::{
    objects::Creep, SharedCreepProperties,
};

mod structures;
mod path;
mod assign;
mod perform;

/*

--TODO--
- Evenly distribute creeps to different sources
- Evenly distribute creeps to different structures
  - Prioritize structures that are closer to the source
  - Prioritize structures that are closer to the controller
  - Ensure that the controller is always being upgraded
- Implement a way to assign creeps to different roles
- Implement a way to assign creeps to different rooms

*/

// fn diagnostics() {
//     // owned structures
//     let owned_structures = structures::get_owned();

//     owned_structures.iter().for_each(|structure| {
//         debug!("structure: {:?}", structure);
//     });
//     structures::get_actionable().iter().for_each(|structure| {
//         debug!("actionable: {:?}", structure);
//     });

//     // is repairable
//     let repairable = owned_structures.iter().filter(|structure| structures::is_repairable(structure));
//     repairable.for_each(|structure| {
//         debug!("repairable: {:?}", structure);
//     });

//     // has free capacity
//     let free_capacity = owned_structures.iter().filter(|structure| structures::has_free_capacity(structure));
//     free_capacity.for_each(|structure| {
//         debug!("free capacity: {:?}", structure);
//     });
// }

pub fn run(creep: &Creep, creep_targets: &mut HashMap<String, CreepTarget>) {
    if creep.spawning() {
        return;
    }
    let name = creep.name();
    // debug!("running creep {}", name);
    // diagnostics();

    let target = creep_targets.entry(name);
    
    // log the target type
    // debug!("target: {:?}", target);

    // TODO :: abstract case logic to a function

    match target {
        Entry::Occupied(entry) => {
            perform::run(creep, entry);
        }
        Entry::Vacant(entry) => {
            assign::run(creep, entry);
        }
    }
}