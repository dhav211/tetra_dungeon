mod player;
mod updatable;

mod prelude
{
    pub use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
    pub use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
    pub use tetra::input::{self, Key};
    pub use tetra::{Context, ContextBuilder, State};
    pub use tetra::math::{self, Vec2, Rect};
    pub use tetra::time::{self, get_delta_time};
    pub use crate::player::*;
    pub use crate::updatable::*;
}

use prelude::*;

struct GameState
{
    scaler : ScreenScaler,
    player : Player,
}

impl GameState
{
    
}

impl State for GameState
{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result 
    {
        // Set the canvas to the scaler
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::BLACK);

        // Draw entities
        self.player.draw(ctx);

        // Reset the canvas and draw the scaler
        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);

        self.scaler.draw(ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result  
    {
        let delta = time::get_delta_time(ctx).as_secs_f32();

        self.player.update(ctx, delta);

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
        .run(|ctx| Ok(GameState 
            {
                scaler : ScreenScaler::with_window_size(ctx, 640, 480, ScalingMode::Stretch)?,
                player : Player::new(
                    Vec2::new(5, 5), 
                    Texture::new(ctx, "./assets/entities.png")?,
                    Rectangle::new(0.0, 0.0, 12.0, 12.0)
                ),
            }))
}