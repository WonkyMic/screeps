use std::collections::HashSet;
use js_sys::{JsString, Object, Reflect};
use log::{debug, warn};
use screeps::{
    constants::Part,
    game,
    prelude::*,
};
use wasm_bindgen::JsCast;

fn purge() {
    debug!("running purge");
    for creep in game::creeps().values() {
        if creep.ticks_to_live().unwrap_or(0) < 25 {
            let _ =creep.suicide();
        }
    }
    
}

pub fn run() {
    purge();

    let mut additional = 0;
    let total_creeps = game::creeps().entries().count();
    for spawn in game::spawns().values() {
        debug!("Available Energy {:?}", spawn.room().unwrap().energy_available());

        let body = [Part::Move, Part::Move, Part::Carry, Part::Work];
        if total_creeps < 8 && spawn.room().unwrap().energy_available() >= body.iter().map(|p| p.cost()).sum(){
            // create a unique name, spawn.
            let name_base = game::time();
            let name = format!("{}-{}", name_base, additional);
            match spawn.spawn_creep(&body, &name) {
                Ok(()) => additional += 1,
                Err(e) => warn!("couldn't spawn: {:?}", e),
            }
        }
    }

    // memory cleanup; memory gets created for all creeps upon spawning, and any time move_to
    // is used; this should be removed if you're using RawMemory/serde for persistence
    if game::time() % 1000 == 0 {
        let mut alive_creeps = HashSet::new();
        
        // add all living creep names to a hashset
        for creep_name in game::creeps().keys() {
            alive_creeps.insert(creep_name);
        }

        // grab `Memory.creeps` (if it exists)
        if let Ok(memory_creeps) = Reflect::get(&screeps::memory::ROOT, &JsString::from("creeps")) {
            // convert from JsValue to Object
            let memory_creeps: Object = memory_creeps.unchecked_into();
            // iterate memory creeps
            for creep_name_js in Object::keys(&memory_creeps).iter() {
                // convert to String (after converting to JsString)
                let creep_name = String::from(creep_name_js.dyn_ref::<JsString>().unwrap());

                // check the HashSet for the creep name, deleting if not alive
                if !alive_creeps.contains(&creep_name) {
                    // info!("deleting memory for dead creep {}", creep_name);
                    let _ = Reflect::delete_property(&memory_creeps, &creep_name_js);
                }
            }
        }
    }
}