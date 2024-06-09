use crate::CreepTarget;
use std::collections::{hash_map::Entry, HashMap};
use log::{debug, info, warn};
use screeps::{
    constants::{ErrorCode, ResourceType}, enums::StructureObject, find, objects::Creep, HasId, HasPosition, SharedCreepProperties,
};

mod structures;
mod path;

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

fn diagnostics() {
    // owned structures
    let owned_structures = structures::get_owned();

    owned_structures.iter().for_each(|structure| {
        debug!("structure: {:?}", structure);
    });
    structures::get_actionable().iter().for_each(|structure| {
        debug!("actionable: {:?}", structure);
    });

    // is repairable
    let repairable = owned_structures.iter().filter(|structure| structures::is_repairable(structure));
    repairable.for_each(|structure| {
        debug!("repairable: {:?}", structure);
    });

    // has free capacity
    let free_capacity = owned_structures.iter().filter(|structure| structures::has_free_capacity(structure));
    free_capacity.for_each(|structure| {
        debug!("free capacity: {:?}", structure);
    });
}

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

    let creep_energy_capacity = creep.store().get_used_capacity(Some(ResourceType::Energy));

    // TODO :: abstract case logic to a function

    match target {
        Entry::Occupied(entry) => {
            let creep_target = entry.get();
            match creep_target {
                CreepTarget::StoreSpawn(spawn_id)
                    if creep_energy_capacity > 0 =>
                {
                    if let Some(spawn) = spawn_id.resolve() {
                        if creep.pos().is_near_to(spawn.pos()) {
                            creep.transfer(&spawn, ResourceType::Energy, Some(creep_energy_capacity)).unwrap_or_else(|e| {
                                // attempt to transfer a reduced amount of energy
                                let _ = creep.transfer(&spawn, ResourceType::Energy, Some(spawn.store().get_free_capacity(Some(ResourceType::Energy)).try_into().or_else(|e| {
                                    warn!("MEGA couldn't convert capacity: {:?}", e);
                                    Err(0)
                                }).expect("couldn't get free capacity")));

                                entry.remove();
                            });
                        } else {
                            let _ = creep.move_to(&spawn);
                        }
                    } else {
                        entry.remove();
                    }
                }
                CreepTarget::StoreTower(tower_id)
                    if creep_energy_capacity > 0 =>
                {
                    if let Some(tower) = tower_id.resolve() {
                        if creep.pos().is_near_to(tower.pos()) {
                            creep.transfer(&tower, ResourceType::Energy, Some(creep_energy_capacity)).unwrap_or_else(|e| {
                                // attempt to transfer a reduced amount of energy
                                let _ = creep.transfer(&tower, ResourceType::Energy, Some(tower.store().get_free_capacity(Some(ResourceType::Energy)).try_into().or_else(|e| {
                                    warn!("MEGA couldn't convert capacity: {:?}", e);
                                    Err(0)
                                }).expect("couldn't get free capacity")));

                                entry.remove();
                            });
                        } else {
                            let _ = creep.move_to(&tower);
                        }
                    } else {
                        entry.remove();
                    }
                }
                CreepTarget::StoreExtension(extension_id)
                    if creep_energy_capacity > 0 =>
                {
                    if let Some(extension) = extension_id.resolve() {
                        if creep.pos().is_near_to(extension.pos()) {
                            creep.transfer(&extension, ResourceType::Energy, Some(creep_energy_capacity)).unwrap_or_else(|e| {
                                // attempt to transfer a reduced amount of energy
                                let _ = creep.transfer(&extension, ResourceType::Energy, Some(extension.store().get_free_capacity(Some(ResourceType::Energy)).try_into().or_else(|e| {
                                    warn!("MEGA couldn't convert capacity: {:?}", e);
                                    Err(0)
                                }).expect("couldn't get free capacity")));

                                entry.remove();
                            });
                        } else {
                            let _ = creep.move_to(&extension);
                        }
                    } else {
                        entry.remove();
                    }
                }
                CreepTarget::Upgrade(controller_id)
                    if creep_energy_capacity > 0 =>
                {
                    if let Some(controller) = controller_id.resolve() {
                        creep
                            .upgrade_controller(&controller)
                            .unwrap_or_else(|e| match e {
                                ErrorCode::NotInRange => {
                                    let _ = creep.move_to(&controller);
                                }
                                _ => {
                                    warn!("couldn't upgrade: {:?}", e);
                                    entry.remove();
                                }
                            });
                    } else {
                        entry.remove();
                    }
                }
                CreepTarget::Harvest(source_id)
                    if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 =>
                {
                    if let Some(source) = source_id.resolve() {
                        if creep.pos().is_near_to(source.pos()) {
                            creep.harvest(&source).unwrap_or_else(|e| {
                                warn!("couldn't harvest: {:?}", e);
                                entry.remove();
                            });
                        } else {
                            let _ = creep.move_to(&source);
                        }
                    } else {
                        entry.remove();
                    }
                }
                _ => {
                    entry.remove();
                }
            };
        }
        Entry::Vacant(entry) => {
            // debug!("VACANT :: assigning target");
            // no target, let's find one depending on if we have energy
            let room = creep.room().expect("couldn't resolve creep room");
            if creep_energy_capacity > 0 {
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

                let (source_id, _) = source_distances.first().expect("no source found");

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
    }
}