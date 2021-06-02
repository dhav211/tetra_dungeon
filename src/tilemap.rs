use ldtk::EntityInstance;

use crate::prelude::*;

pub struct Tilemap 
{
    pub width : i32,
    pub height : i32,
    pub tiles : Vec<Tile>,
    texture : Texture,
    pub entities : Vec<EntityToSpawn>
}

impl Tilemap
{
    pub fn new(texture : Texture) -> Self
    {
        Self
        {
            width : 0,
            height : 0,
            tiles : vec![],
            texture,
            entities : vec![]
        }
    }

    pub fn create_level_tilemap(&mut self, _level_name : &str) -> Result<(), Box<dyn std::error::Error>>
    {
        let map : Project = serde_json::from_slice(include_bytes!("../assets/tetra_rogue_map.ldtk"))?;

        let layer_instances = map.levels[0].layer_instances.as_ref().unwrap();
        self.width = map.levels[0].px_wid / 12;
        self.height = map.levels[0].px_hei / 12;
        self.tiles = self.create_empty_tiles(layer_instances[0].__c_wid, layer_instances[0].__c_hei);

        for layer_instance in layer_instances
        {
            match layer_instance.__type.as_str()
            {
                "Entities" => 
                {
                    let entity_instances = &layer_instance.entity_instances;

                    entity_instances.iter()
                        .for_each(|e| match e.__identifier.as_str()
                        {
                            "Player" =>
                            {
                                self.entities.push(self.create_entity_to_spawn(EntityType::Player, e));
                            }
                            "Enemy" =>
                            {
                                self.entities.push(self.create_entity_to_spawn(EntityType::Enemy, e));
                            }
                            _ => 
                            {
                                println!("ERROR in Entity Instances. Instance didn't have an identifer");
                            }
                        })
                },
                "Tiles" =>
                {
                    // TODO extrac this into a function
                    for grid_tile in layer_instance.grid_tiles.iter()
                    {
                        let position = Vec2::new(grid_tile.px[0] as f32, grid_tile.px[1] as f32);
                        let grid_pos = Vec2::new((position.x / 12.0) as i32, (position.y / 12.0) as i32);
                        let tile_index = self.get_tile_index(grid_pos.x, grid_pos.y);
                        let tile_coords = self.get_tile_coords_from_id(grid_tile.t);

                        self.tiles[tile_index].region_rect = Rectangle::new(tile_coords.x, tile_coords.y, 12.0, 12.0);
                        self.tiles[tile_index].grid_position = grid_pos;
                        self.tiles[tile_index].position = Vec2::new((grid_pos.x * 12) as f32, (grid_pos.y * 12) as f32);
                        self.tiles[tile_index].is_null = false;
                    }
                },
                "IntGrid" =>
                {
                    println!("The layer {} has collisions", layer_instance.__identifier);
                }
                _ =>
                {
                    println!("ERROR in Tilemap loading. Layer Instance doesn't have a type.");
                }
            }
        }

        self.remove_null_tiles();

        Ok(())
    }

    fn create_empty_tiles(&self, width : i32, height : i32) -> Vec<Tile>
    {
        let mut empty_tiles : Vec<Tile> = vec![];
        let empty_tile = Tile 
        {
            region_rect : Rectangle::new(0.0, 0.0, 0.0, 0.0),
            grid_position : Vec2::new(0, 0),
            position : Vec2::new(0.0, 0.0),
            is_collidable : false,
            is_null : true,
        };
        empty_tiles.resize((width * height) as usize, empty_tile);
        empty_tiles
    }

    fn remove_null_tiles(&mut self)
    {
        self.tiles.retain(|x| !x.is_null);
    }

    fn get_tile_index(&self, x : i32, y : i32) -> usize
    {
        ((y * self.width) + x) as usize
    }

    fn get_tile_coords_from_id(&self, tile_id : i32) -> Vec2<f32>
    {
        let grid_base_size = 25;  // TODO This is just the number of tiles wide the tileset is. LDTK importer should have this, probably __cWid
        let spacing = 1; // TODO these should be grabbed from the ldtk file
        let padding = 1;
        let grid_size = 12;
        let grid_tile_x = tile_id - grid_base_size * (tile_id / grid_base_size);
        let pixel_tile_x = padding + grid_tile_x * (grid_size + spacing);
        let grid_tile_y = tile_id / grid_base_size;
        let pixel_tile_y = padding + grid_tile_y * (grid_size + spacing);

        Vec2::new(pixel_tile_x as f32, pixel_tile_y as f32)
    }

    fn create_entity_to_spawn(&self, entity_type : EntityType, entity_instance : &EntityInstance) -> EntityToSpawn
    {
        EntityToSpawn
        {
            entity_type : entity_type,
            position : Vec2::new(entity_instance.px[0] as f32, entity_instance.px[1] as f32),
            grid_position : Vec2::new(entity_instance.px[0] / 12, entity_instance.px[1] / 12),
        }
    }
}

impl Updatable for Tilemap
{
    fn draw(&self, ctx : &mut Context)
    {
        for tile in self.tiles.iter()
        {
            self.texture.draw_region(ctx, tile.region_rect, tile.position);
        }
    }

    fn update(&mut self, ctx : &mut Context, delta : f32) {}
}

#[derive(Copy, Clone)]
pub struct Tile
{
    region_rect : Rectangle<f32>,
    grid_position : Vec2<i32>,
    position : Vec2<f32>,
    is_collidable : bool,
    is_null : bool,
}

#[derive(Copy, Clone)]
pub enum EntityType
{
    Player,
    Enemy,
    Chest,
}
#[derive(Copy, Clone)]
pub struct EntityToSpawn
{
    pub entity_type : EntityType,
    pub position : Vec2<f32>,
    pub grid_position : Vec2<i32>
}