#![feature(core)] 
#![feature(box_syntax)]
#![allow(unstable)]
#![feature(collections)]
extern crate glutin;
#[macro_use]
extern crate glium;

extern crate image;
use glium::{DisplayBuild, Surface};
use std::io::Cursor;
use std::cell::RefCell;
use location::Vec2d;
//use core::slice::Iter;

//add my mods here
mod location;
mod world;
mod enitys;
mod render;
mod things;


thread_local!(static root: RefCell<world::World<'static>> = RefCell::new(world::World::new()));

fn main(){
    {
        let display = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_title(format!("Forced Fracture"))
            .build_glium().unwrap();
        root.with(|w| w.borrow_mut().set_context(display));
    }
    let rend_engine: render::Render;
        
        root.with(|w| rend_engine = render::Render::new(w.borrow().contex()));

    //let mut world: Vec<Box<enitys::Enity>> = Vec::new();
    
    root.with(|w| {
        let mut world = w.borrow_mut();

        world.add(box things::mobs::Rock::
                  new("".to_string(), Vec2d::new(-20.1,-1.0)));
        world.add(box things::mobs::Rock::
                  new("".to_string(), Vec2d::new(78.0,9.0)));
        world.add(box things::mobs::Rock::
                  new("".to_string(), Vec2d::new(-45.0,45.0)));
        world.add(box things::mobs::Rock::
                  new("".to_string(), Vec2d::new(23.0,-450.0)));
        world.add(box things::mobs::DevDan::
                  new("Dan".to_string(),Vec2d::new(0.0,0.0)));
        world.add(box things::mobs::John::
                  new("117".to_string(),Vec2d::new(-50.0,-70.0)));
        world.add(box things::mobs::John::
                  new("104".to_string(),Vec2d::new(20.0,50.0)));
    });
    let mut camera = location::Vec2d::new(0.0,0.0);

    loop{ //play loop
        //polling and handling the events received by the window
            //let world = w.borrow();
            //let events = world.contex().poll_events();
            //for event in events{
            //    match event {
            //        glutin::Event::Closed => return,
            //        _ => ()
            //    }
            //}

        root.with(|w|{
            //draw things
            let mut draws = Vec::new();
            //let world_list = w.borrow().all_the_things();
                //for x in w.borrow().all_the_things(){
                    //let (key,val) = x;
                    //let temp = val.draw_handle();
                    //if let Some(thing) = temp {
                    //    draws.push(thing);
                    //}
                //}

            rend_engine.draw_frame(draws.iter(), &camera);
        });
    }//end main loop
}
