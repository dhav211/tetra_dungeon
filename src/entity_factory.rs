use crate::prelude::*;

pub fn create_entities_to_spawn(entities_to_spawn : &Vec<EntityToSpawn>, texture : Texture) -> Vec<Box<dyn Updatable>>
{
    let mut entities : Vec<Box<dyn Updatable>> = vec![];

    entities_to_spawn.iter()
        .for_each(|e| match e.entity_type
        {
            EntityType::Player =>
            {
                entities.push(Box::new(Player::new(e.grid_position, texture.clone(), Rectangle::new(0.0, 0.0, 12.0, 12.0))));
            }

            EntityType::Enemy =>
            {
                // TODO: Create random monster to spawn
                println!("No way to spawn an enemy yet, work on it!");
            }
            EntityType::Chest => {}
        });

    entities
}