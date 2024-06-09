use crate::CreepTarget;
use std::collections::{hash_map::Entry, HashMap};
use screeps::{
    objects::Creep, SharedCreepProperties,
};

mod structures;
mod path;
mod assign;
mod perform;

pub fn run(creep: &Creep, creep_targets: &mut HashMap<String, CreepTarget>) {
    if creep.spawning() {
        return;
    }
    
    let name = creep.name();
    let target = creep_targets.entry(name);
    
    match target {
        Entry::Occupied(entry) => {
            perform::run(creep, entry);
        }
        Entry::Vacant(entry) => {
            assign::run(creep, entry);
        }
    }
}