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


thread_local!(static root: RefCell<world::World> = RefCell::new(world::World::new()));

fn main(){
    let x = init();

    //let mut world: Vec<Box<enitys::Enity>> = Vec::new();
    setup();

    run(&x);
}

fn init()-> render::Render{
let display = glutin::WindowBuilder::new()
            .with_dimensions(1024, 768)
            .with_title(format!("Forced Fracture"))
            .build_glium().unwrap();
        root.with(|w| w.borrow_mut().set_context(display));

    let x =root.with(
        |w| render::Render::new(w.borrow().contex()));
    x
}


fn setup(){
    println!("test");
    let a = box things::mobs::Rock::
                  new("".to_string(), Vec2d::new(-20.1,-1.0));

println!("a");
    let b = box things::mobs::Rock::
                  new("".to_string(), Vec2d::new(78.0,9.0));
println!("b");
    let c = box things::mobs::Rock::
                  new("".to_string(), Vec2d::new(-45.0,45.0));
println!("c");
    let d = box things::mobs::Rock::
                  new("".to_string(), Vec2d::new(23.0,-450.0));
println!("d");
    let e = box things::mobs::DevDan::
                  new("Dan".to_string(),Vec2d::new(0.0,0.0));
println!("e");
    let f = box things::mobs::John::
                  new("117".to_string(),Vec2d::new(-50.0,-70.0));
println!("f");
    let g = box things::mobs::John::
                  new("104".to_string(),Vec2d::new(20.0,50.0));
    
println!("g");
    root.with(|w| {
        let mut world = w.borrow_mut();

        world.add(a);
        world.add(b);
        world.add(c);
        world.add(d);
        world.add(e);
        world.add(f);
        world.add(g);
    });
    println!("post");
}

fn run(rend_engine: &render::Render){
//loop{ //play loop
       //polling and handling the events received by the window
            //let world = w.borrow();
            //let events = world.contex().poll_events();
            //for event in events{
            //    match event {
            //        glutin::Event::Closed => return,
            //        _ => ()
            //    }
            //}
println!("preloop");
        let mut camera = location::Vec2d::new(0.0,0.0);
        root.with(|w|{
        loop{
            println!("loop");
            let world = w.borrow();
            let mut draws = Vec::new();
            //draw things
            //let world = w;
            //let ww = world.borrow();
            for (key,val) in world.all_the_things(){
                //let (key,val) = x;
                let temp = val.draw_handle();
                if let Some(thing) = temp {
                    draws.push(thing);
                    }
                }
        rend_engine.draw_frame(draws.iter(), &camera);
    }//end main loop
    })

}
