use std::option::{Option};
use std::boxed::Box;
//use std::slice::Iter;
use location::*;
use std::iter::Iterator;
use std;

use std::marker::MarkerTrait;

pub trait Enity{
    fn parent_id(&self) ->usize;
    fn name(&self) -> &str;
    fn ID(&self) -> usize;
    //fn childeren() -> HashMap<usize,&Enity>;
    
    fn draw_handle(&self) -> Option<&Drawable>{
        None::<&Drawable>
    }
    fn update_handle(&mut self) -> Option<&Updates>{
        None::<&Updates>
    }
    fn combat_handle(&mut self) -> Option<&Combat>{
        None::<&Combat>
    }
}

pub trait Drawable{
    fn panel(&self)-> &ImgVal;
    fn location(&self) -> Vec2d;
    fn size(&self) -> f32;
}

pub trait Updates{
    fn update(&self,sec: f32);
    fn change_velo(&self,direc: Vec2d){}
}

pub trait Combat{
    fn hp(&self) -> usize;
    fn take_hit(&self, usize);
    fn hit_bounds(&self) -> Option<&Bound>;
    fn atk_bounds(&self) -> Option<(&Atk)>;
}

pub trait Event:MarkerTrait{
    
}
