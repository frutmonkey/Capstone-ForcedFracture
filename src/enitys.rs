use std::option::{Option};
use std::boxed::Box;
use location::*;

use std::marker::MarkerTrait;

pub trait Enity<'a>{
    fn parent_id(&self) ->usize;
    fn name(&self) -> &str;
    //fn childeren() -> HashMap<usize,&Enity;
    
    fn draw_handle(& 'a self) -> Option<& 'a Drawable>{
        None::<&Drawable>
    }

    fn update_handle(& mut self) -> Option<Box<Updates>>{
        None::<Box<Updates>>
    }
    fn combat_handle(& mut self) -> Option<Box<Combat>>{
        None::<Box<Combat>>
    }
}

pub trait Drawable{
    fn panel(&self)-> &ImgVal;
    fn location(&self) -> Vec2d;
    fn size(&self) -> f32;
}

pub trait Updates : MarkerTrait{

}

pub trait Combat : MarkerTrait{

}
