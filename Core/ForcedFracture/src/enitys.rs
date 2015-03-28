use std::option::{Option};
use std::boxed::Box;
use glium::uniforms::Uniforms;
use location::*;

use std::marker::MarkerTrait;

pub trait Enity{
    fn parent_ID(&self) ->usize;
    fn name(&self) -> str;
    //fn childeren() -> HashMap<usize,&Enity;
    
    fn draw_handle(&self) -> Option<Box<Drawable>>{
        None::<Box<Drawable>>
    }

    fn update_handle(& mut self) -> Option<Box<Updates>>{
        None::<Box<Updates>>
    }
    fn combat_handle(& mut self) -> Option<Box<Combat>>{
        None::<Box<Combat>>
    }
}

pub trait Drawable{
    fn panel(&self)-> ImgData;
    fn location(&self) -> Vec2d;
    fn size(&self) -> f32;
}

pub trait Updates : MarkerTrait{

}

pub trait Combat : MarkerTrait{

}
