use screeps::{
    find,
    objects::Creep,
    StructureSpawn,
};

pub fn get_spawn(creep: &Creep) -> Option<StructureSpawn> {
    let room = creep.room()?;
    let spawns = room.find(find::MY_SPAWNS, None);
    spawns.iter().next().cloned()
}