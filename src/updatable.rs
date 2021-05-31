pub use crate::prelude::*;

pub trait Updatable 
{
    fn draw(&self, ctx : &mut Context);
    fn update(&mut self, ctx : &mut Context, delta : f32);
}