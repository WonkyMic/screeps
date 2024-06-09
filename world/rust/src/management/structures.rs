use screeps::{
    enums::StructureObject,
    find,
    game,
    HasHits,
    objects::Creep,
    ResourceType,
    StructureSpawn,
};

// pub fn get_owned() -> Vec<StructureObject> {
//     let mut structures = Vec::new();
//     for room in game::rooms().values() {
//         for structure in room.find(find::MY_STRUCTURES, None) {
//             structures.push(structure);
//         }
//     }
//     structures
// }

pub fn get_spawn(creep: &Creep) -> Option<StructureSpawn> {
    let room = creep.room()?;
    let spawns = room.find(find::MY_SPAWNS, None);
    spawns.iter().next().cloned()
}

// pub fn has_free_capacity(structure: &StructureObject) -> bool {
//     match structure {
//         StructureObject::StructureTower(tower) => {
//             tower.store().get_free_capacity(Some(ResourceType::Energy)) > 0
//         }
//         StructureObject::StructureSpawn(spawn) => {
//             spawn.store().get_free_capacity(Some(ResourceType::Energy)) > 0
//         }
//         StructureObject::StructureExtension(extension) => {
//             extension.store().get_free_capacity(Some(ResourceType::Energy)) > 0
//         }
//         _ => false,
//     }
// }

// pub fn is_repairable(structure: &StructureObject) -> bool {
//     match structure {
//         StructureObject::StructureRoad(road) => road.hits() < road.hits_max(),
//         StructureObject::StructureContainer(container) => container.hits() < container.hits_max(),
//         StructureObject::StructureWall(wall) => wall.hits() < wall.hits_max(),
//         StructureObject::StructureRampart(rampart) => rampart.hits() < rampart.hits_max(),
//         _ => false,
//     }
// }

// pub fn get_actionable() -> Vec<StructureObject> {
//     let mut actionable = get_owned();
//     actionable.retain(|structure| has_free_capacity(structure));
//     actionable
// }