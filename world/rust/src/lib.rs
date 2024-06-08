use std::{
    cell::RefCell,
    collections::HashMap,
};
use log::*;
use screeps::{
    game,
    local::ObjectId,
    objects::{Source, StructureController, StructureExtension},
};
use wasm_bindgen::prelude::*;

mod lifecycle;
mod logging;
mod management;
mod combat;

// this is one way to persist data between ticks within Rust's memory, as opposed to
// keeping state in memory on game objects - but will be lost on global resets!
thread_local! {
    static CREEP_TARGETS: RefCell<HashMap<String, CreepTarget>> = RefCell::new(HashMap::new());
}

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

// this enum will represent a creep's lock on a specific target object, storing a js reference
// to the object id so that we can grab a fresh reference to the object each successive tick,
// since screeps game objects become 'stale' and shouldn't be used beyond the tick they were fetched
#[derive(Clone, Debug)]
enum CreepTarget {
    Upgrade(ObjectId<StructureController>),
    Harvest(ObjectId<Source>),
    Store(ObjectId<StructureExtension>),
}

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    INIT_LOGGING.call_once(|| {
        // show all output of Info level, adjust as needed
        logging::setup_logging(logging::Debug);
    });

    debug!("loop starting! CPU: {}", game::cpu::get_used());

    // mutably borrow the creep_targets refcell, which is holding our creep target locks
    // in the wasm heap
    CREEP_TARGETS.with(|creep_targets_refcell| {
        let mut creep_targets = creep_targets_refcell.borrow_mut();
        // debug!("running creeps");
        for creep in game::creeps().values() {
            management::run(&creep, &mut creep_targets);
        }
    });

    // lifecycle
    lifecycle::run();

    combat::run();

    info!("done! cpu: {}", game::cpu::get_used())
}