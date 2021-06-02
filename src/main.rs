mod player;
mod updatable;
mod tilemap;
mod entity_factory;
mod board;

mod prelude
{
    pub use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
    pub use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
    pub use tetra::input::{self, Key};
    pub use tetra::{Context, ContextBuilder, State};
    pub use tetra::math::{self, Vec2, Rect};
    pub use tetra::time::{self, get_delta_time};
    pub use ldtk::{self, Project, Level, LayerInstance};
    pub use serde_json::from_slice;
    pub use crate::player::*;
    pub use crate::updatable::*;
    pub use crate::tilemap::*;
    pub use crate::entity_factory::*;
    pub use crate::board::*;
}

use prelude::*;

struct GameState
{
    scaler : ScreenScaler,
    tilemap : Tilemap,
    entities : Vec<Box<dyn Updatable>>
}

impl GameState
{
    fn new(ctx : &mut Context) -> tetra::Result<GameState>
    {
        // TODO there will be various structs that contain game state, mainly one for assets. It will be created here
        let mut current_tilemap = Tilemap::new(Texture::new(ctx, "./assets/tilemap.png")?);
        current_tilemap.create_level_tilemap("level_0").ok();
        let entity_texture = Texture::new(ctx, "./assets/entities.png")?;

        Ok(GameState
        {
            scaler : ScreenScaler::with_window_size(ctx, 320, 240, ScalingMode::ShowAllPixelPerfect)?,
            entities : entity_factory::create_entities_to_spawn(&current_tilemap.entities, entity_texture),
            tilemap : current_tilemap,
        })
    }
}

impl State for GameState
{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result 
    {
        // Set the canvas to the scaler
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);

        // Draw entities
        self.tilemap.draw(ctx);
        self.entities.iter()
            .for_each(|e| e.draw(ctx));


        // Reset the canvas and draw the scaler
        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);

        self.scaler.draw(ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result  
    {
        let delta = time::get_delta_time(ctx).as_secs_f32();
        
        self.entities.iter_mut()
            .for_each(|e| e.update(ctx, delta));

        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: tetra::Event) -> tetra::Result  
    {
        Ok(())
    }
}

fn main() -> tetra::Result
{
    ContextBuilder::new("Tetra Dungeon", 1280, 960)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
        
}