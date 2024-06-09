use crate::CreepTarget;
use std::collections::hash_map::VacantEntry;
use log::{debug, warn};
use screeps::{
    constants::ResourceType, enums::StructureObject, find, objects::Creep, HasId, HasPosition,
};

use crate::management::path;
use crate::management::structures;

pub fn run(creep: &Creep, entry: VacantEntry<String, CreepTarget>) {
    // debug!("VACANT :: assigning target");
    // no target, let's find one depending on if we have energy
    let room = creep.room().expect("couldn't resolve creep room");
    if creep.store().get_used_capacity(Some(ResourceType::Energy)) > 0 {
        // debug!("VACANT :: has energy");

        /*
            when looping through structures, the order is not guaranteed, so search for priority targets first
            Priority 1: Spawn
            Priority 2: Extensions
            Priority 3: Controller
        */
        
        let mut target = None;
        
        // find Spawns to fill
        if let Some(spawn) = structures::get_spawn(creep) {
            // find the closest spawn that is not occupied by self and has free capacity
            if spawn.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
                target = Some(CreepTarget::StoreSpawn(spawn.id()));
                debug!("VACANT :: assigned to store");
            }
        }

        // find towers to fill
        if target.is_none() {
            for structure in room.find(find::STRUCTURES, None).iter() {
                // debug!("VACANT :: found structure {}", structure.structure_type());

                // find Towers to fill
                if let StructureObject::StructureTower(tower) = structure {
                    // find the closest tower that is not occupied by self and has free capacity
                    if tower.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
                        target = Some(CreepTarget::StoreTower(tower.id()));
                        debug!("VACANT :: assigned to store");
                        break;
                    }
                }
            }
        }

        if target.is_none() {
            for structure in room.find(find::STRUCTURES, None).iter() {
                // debug!("VACANT :: found structure {}", structure.structure_type());

                // find Extensions to fill
                if let StructureObject::StructureExtension(extension) = structure {
                    // find the closest extension that is not occupied by self and has free capacity
                    if !path::check_if_extension_is_occupied(&extension) && extension.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
                        target = Some(CreepTarget::StoreExtension(extension.id()));
                        debug!("VACANT :: assigned to store");
                        break;
                    }
                }
            }
        }

        // If no extension was found or all are full, find a controller to upgrade
        if target.is_none() {
            for structure in room.find(find::STRUCTURES, None).iter() {
                // find a controller to upgrade
                if let StructureObject::StructureController(controller) = structure {
                    debug!("VACANT :: assigned to upgrade");
                    target = Some(CreepTarget::Upgrade(controller.id()));
                    break;
                }
            }
        }

        if let Some(target) = target {
            entry.insert(target);
        } else {
            warn!("no target found for creep");
        }

    } else if !room.find(find::SOURCES_ACTIVE, None).is_empty() {
        // debug!("VACANT :: no energy, has sources");
        // calculate the distance between the spawn and the sources
        let spawn = structures::get_spawn(creep).expect("no spawn found");
        let mut source_distances = room
            .find(find::SOURCES_ACTIVE, None)
            .iter()
            .map(|source| (source.id(), spawn.pos().get_range_to(source.pos())))
            .collect::<Vec<_>>();
        source_distances.sort_by_key(|(_, distance)| *distance);

        // debug!("VACANT :: source is occupied");
        // find the closest source that is not occupied
        for (source_id, _) in source_distances.iter() {
            if !path::check_if_source_is_occuppied(&source_id.resolve().expect("source not found")){
                entry.insert(CreepTarget::Harvest(*source_id));
                debug!("VACANT :: assigned to harvest");
                break;
            }
        }
    } else {
        warn!("no target found for creep");
    }
}