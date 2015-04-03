#![feature(core)] 
#![feature(box_syntax)]
#![allow(unstable)]
extern crate glutin;
#[macro_use]
extern crate glium;

extern crate image;
use glium::{DisplayBuild, Surface};
use std::io::Cursor;
use location::Vec2d;
//use core::slice::Iter;

//add my mods here
mod location;
mod world;
mod enitys;
mod render;
mod things;

fn main(){

    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Forced Fracture"))
        .build_glium().unwrap();
    
    let rend_engine = render::Render::new(&display);

    let mut world: Vec<Box<enitys::Enity>> = Vec::new();
    world.push(box things::mobs::Rock::
               new("".to_string(), Vec2d::new(-20.1,-1.0),&display));
     world.push(box things::mobs::Rock::
               new("".to_string(), Vec2d::new(78.0,9.0),&display));
    world.push(box things::mobs::Rock::
               new("".to_string(), Vec2d::new(-45.0,45.0),&display));
    world.push(box things::mobs::Rock::
               new("".to_string(), Vec2d::new(23.0,-450.0),&display));
    world.push(box things::mobs::DevDan::
               new("Dan".to_string(),Vec2d::new(0.0,0.0),&display));
    world.push(box things::mobs::John::
               new("117".to_string(),Vec2d::new(-50.0,-70.0),&display));
    world.push(box things::mobs::John::
               new("104".to_string(),Vec2d::new(20.0,50.0),&display));
    let mut camera = location::Vec2d::new(0.0,0.0);

    loop{ //play loop
        //polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }

        //draw things
        let mut draws = Vec::new();
        for x in world.iter(){
            let temp = x.draw_handle();
            if let Some(thing) = temp {
                draws.push(thing);
            }
        }

        let mut target = display.draw();
        rend_engine.draw_frame(&display, draws.iter(), &camera);

    }//end main loop
}
