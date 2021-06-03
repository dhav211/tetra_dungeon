use crate::{entity_factory, prelude::*};

pub enum State
{
    Loading,
    Playing,
    Clearing
}

pub struct Scene <'a>
{
    pub tilemap : Tilemap,
    pub entities : Vec<Box<dyn Updatable>>,
    pub board : Board<'a>,
}


    // entities : entity_factory::create_entities_to_spawn(&current_tilemap.entities, entity_texture),
    // tilemap : current_tilemap,
    // board : board

impl<'a> Scene<'a>
{
    pub fn load_scene(ctx : &mut Context, scene_name : &str) -> Result<Scene<'a>, tetra::TetraError>
    {
        let mut current_tilemap = Tilemap::new(Texture::new(ctx, "./assets/tilemap.png")?);
        current_tilemap.create_level_tilemap("level_0").ok();
        let mut board = Board::new(&current_tilemap.tiles);
        let entity_texture = Texture::new(ctx, "./assets/entities.png")?;

        Ok(Scene
        {
            entities : entity_factory::create_entities_to_spawn(&current_tilemap.entities, entity_texture),
            tilemap : current_tilemap,
            board : board
        })
    }
}