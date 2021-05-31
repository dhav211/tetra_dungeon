use crate::prelude::*;

pub struct Player
{
    pub position : Vec2<f32>,
    pub grid_position : Vec2<i32>,
    texture : Texture,
    region_rect : Rectangle,
}

impl Player
{
    pub fn new(grid_position : Vec2<i32>, texture : Texture, region_rect : Rectangle) -> Self
    {
        Self
        {
            position : Vec2::new((grid_position.x * 12) as f32, (grid_position.y * 12) as f32),
            grid_position,
            texture,
            region_rect,
        }
    }
}

impl Updatable for Player
{
    fn update(&mut self, ctx : &mut Context, delta : f32)
    {
        if input::is_key_pressed(ctx, Key::D)
        {
            self.position.x += 12.0;
        }
    }

    fn draw(&self, ctx : &mut Context)
    {
        self.texture.draw_region(ctx, self.region_rect, self.position)
    }
}