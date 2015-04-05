use std::option::{Option};
use std::boxed::Box;
//use std::slice::Iter;
use location::*;
use std::iter::Iterator;
use std;

use std::marker::MarkerTrait;

pub trait Enity<'a>{
    fn parent_id(&self) ->usize;
    fn name(&self) -> &str;
    //fn childeren() -> HashMap<usize,&Enity;
    
    fn draw_handle(& 'a self) -> Option<& 'a Drawable>{
        None::<&Drawable>
    }

    fn update_handle(& mut self) -> Option<& 'a Updates>{
        None::<&Updates>
    }
    fn combat_handle(& mut self) -> Option<& 'a Combat>{
        None::<&Combat>
    }
}

pub trait Drawable{
    fn panel(&self)-> &ImgVal;
    fn location(&self) -> Vec2d;
    fn size(&self) -> f32;
}

pub trait Updates{
    fn update(&self,events: std::slice::Iter<&Event>, sec: f32);
}

pub trait Combat{
    fn hp(&self) -> usize;
    fn take_hit(&self, usize);
    fn hit_bounds(&self) -> Option<&Bound>;
    fn atk_bounds(&self) -> Option<(&Atk)>;
}

pub trait Event:MarkerTrait{
    
}
