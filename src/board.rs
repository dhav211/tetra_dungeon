use crate::prelude::*;

pub struct Board
{
    width : i32,
    height : i32,
    pub spaces : Vec<Space> 
}

pub struct Space
{
    is_null : bool,
    is_wall : bool,
}