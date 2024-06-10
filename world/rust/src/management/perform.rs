use crate::CreepTarget;
use std::collections::hash_map::OccupiedEntry;
use log::{info, warn};
use screeps::{
    constants::{ErrorCode, ResourceType}, objects::Creep, HasPosition, SharedCreepProperties,
};

use crate::management::path;

pub fn run (creep: &Creep, entry: OccupiedEntry<String, CreepTarget>) {
    
    let creep_target = entry.get();
    let creep_energy_capacity: u32 = creep.store().get_used_capacity(Some(ResourceType::Energy)).try_into().or_else(|e| {
        warn!("MEGA couldn't convert capacity: {:?}", e);
        Err(0)
    }).expect("couldn't get free capacity");

    match creep_target {
        CreepTarget::StoreSpawn(spawn_id)
            if creep_energy_capacity > 0 =>
        {
            let _ = creep.say("StoreSpawn", false);
            if let Some(spawn) = spawn_id.resolve() {
                if creep.pos().is_near_to(spawn.pos()) {
                    creep.transfer(&spawn, ResourceType::Energy, Some(creep_energy_capacity)).unwrap_or_else(|_| {
                        // attempt to transfer a reduced amount of energy
                        let _ = creep.transfer(&spawn, ResourceType::Energy, Some(spawn.store().get_free_capacity(Some(ResourceType::Energy)).try_into().or_else(|_| {
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
            let _ = creep.say("StoreTower", false);
            if let Some(tower) = tower_id.resolve() {
                if creep.pos().is_near_to(tower.pos()) {
                    creep.transfer(&tower, ResourceType::Energy, Some(creep_energy_capacity)).unwrap_or_else(|_| {
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
            let _ = creep.say("StoreExtension", false);
            if let Some(extension) = extension_id.resolve() {
                if creep.pos().is_near_to(extension.pos()) {
                    creep.transfer(&extension, ResourceType::Energy, Some(creep_energy_capacity)).unwrap_or_else(|_| {
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
            let _ = creep.say("UpgradeController", false);
            if let Some(controller) = controller_id.resolve() {
                creep
                    .upgrade_controller(&controller)
                    .unwrap_or_else(|e| match e {
                        ErrorCode::NotInRange => {
                            let _ = creep.move_to(&controller);
                        }
                        _ => {
                            info!("couldn't upgrade: {:?}", e);
                            entry.remove();
                        }
                    });
            } else {
                entry.remove();
            }
        }
        CreepTarget::Harvest(source_id)
            // Check if source is occupied. This will prevent deadlocks
            if creep.store().get_free_capacity(Some(ResourceType::Energy)) > 0 =>
        {
            let _ = creep.say("Harvest", false);
            if let Some(source) = source_id.resolve() {
                if creep.pos().is_near_to(source.pos()) {
                    creep.harvest(&source).unwrap_or_else(|e| {
                        warn!("couldn't harvest: {:?}", e);
                        entry.remove();
                    });
                } else if !path::check_if_source_is_occuppied(&source_id.resolve().expect("source not found")) {
                    let _ = creep.move_to(&source);
                } else {
                    entry.remove();
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