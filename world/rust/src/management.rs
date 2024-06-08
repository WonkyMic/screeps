use crate::CreepTarget;
use std::collections::{hash_map::Entry, HashMap};
use log::{debug, warn};
use screeps::{
    constants::{ErrorCode, ResourceType}, enums::StructureObject, find, objects::Creep, HasId, HasPosition, SharedCreepProperties, StructureProperties
};

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

pub fn run(creep: &Creep, creep_targets: &mut HashMap<String, CreepTarget>) {
    if creep.spawning() {
        return;
    }
    let name = creep.name();
    debug!("running creep {}", name);

    let target = creep_targets.entry(name);
    
    // log the target type
    debug!("target: {:?}", target);

    let creep_energy_capacity = creep.store().get_used_capacity(Some(ResourceType::Energy));

    match target {
        Entry::Occupied(entry) => {
            let creep_target = entry.get();
            match creep_target {
                CreepTarget::Store(extension_id)
                    if creep_energy_capacity > 0 =>
                {
                    if let Some(extension) = extension_id.resolve() {
                        if creep.pos().is_near_to(extension.pos()) {
                            creep.transfer(&extension, ResourceType::Energy, Some(creep_energy_capacity)).unwrap_or_else(|e| {
                                warn!("couldn't transfer: {:?}", e);
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
            debug!("VACANT :: assigning target");
            // no target, let's find one depending on if we have energy
            let room = creep.room().expect("couldn't resolve creep room");
            if creep_energy_capacity > 0 {
                debug!("VACANT :: has energy");

                /*
                 when looping through structures, the order is not guaranteed, so search for priority targets first
                 Priority 1: Extensions
                 Priority 2: Controller
                */
                
                let mut target = None;
                
                for structure in room.find(find::STRUCTURES, None).iter() {
                    // debug!("VACANT :: found structure {}", structure.structure_type());

                    // find Extensions to fill
                    if let StructureObject::StructureExtension(extension) = structure {
                        if extension.store().get_free_capacity(Some(ResourceType::Energy)) > 0 {
                            target = Some(CreepTarget::Store(extension.id()));
                            debug!("VACANT :: assigned to store");
                            break;
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

            } else if let Some(source) = room.find(find::SOURCES_ACTIVE, None).first() {
                debug!("VACANT :: has no energy");
                entry.insert(CreepTarget::Harvest(source.id()));
            } else {
                warn!("no target found for creep");
            }
        }
    }
}