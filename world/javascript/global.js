// Spawn
Game.spawns['Spawn1'].spawnCreep( [WORK, CARRY, MOVE], 'Harvester1' );
Game.spawns['Spawn1'].spawnCreep( [WORK, CARRY, MOVE], 'Harvester2' );
Game.spawns['Spawn1'].spawnCreep( [WORK, CARRY, MOVE], 'Upgrader1' );
Game.spawns['Spawn1'].spawnCreep( [WORK, CARRY, MOVE], 'Upgrader2' , { memory: { role: 'upgrader' } });
Game.spawns['Spawn1'].spawnCreep( [WORK, CARRY, MOVE], 'Builder1', { memory: { role: 'builder' } } );
Game.spawns['Spawn1'].spawnCreep( [WORK, CARRY, MOVE], 'Builder2', { memory: { role: 'builder' } } );
// Tower
Game.spawns['Spawn1'].room.createConstructionSite( 23, 22, STRUCTURE_TOWER );

// Memory references
Game.creeps['Harvester1'].memory.role = 'harvester'; 
Game.creeps['Upgrader1'].memory.role = 'upgrader';
// Reference already accounted for in spawn command above
Game.creeps['Builder1'].memory.role = 'builder';