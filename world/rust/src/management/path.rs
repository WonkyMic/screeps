use screeps::{
    find,
    HasPosition,
    Source,
};

pub fn check_if_source_is_occuppied(source: &Source) -> bool {
    // if two creeps are next to the source, then it is occuppied
    let creeps = source.room().unwrap().find(find::MY_CREEPS, None);
    let mut count = 0;
    for creep in creeps.iter() {
        if creep.pos().is_near_to(source.pos()) {
            count += 1;
        }
    }
    count >= 2
}

pub fn check_if_extension_is_occupied(extension: &screeps::StructureExtension) -> bool {
    // if two creeps are next to the extension, then it is occupied
    let creeps = extension.room().unwrap().find(find::MY_CREEPS, None);
    let mut count = 0;
    for creep in creeps.iter() {
        if creep.pos().is_near_to(extension.pos()) {
            count += 1;
        }
    }
    count >= 2
}