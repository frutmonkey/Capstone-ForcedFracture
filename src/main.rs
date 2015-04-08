#![feature(core)] 
#![feature(box_syntax)]
#![allow(unstable)]
#![feature(convert)
#![feature(collections)]
extern crate glutin;
#[macro_use]
extern crate glium;

extern crate image;
use glium::{DisplayBuild, Surface};
use std::io::Cursor;
use std::cell::RefCell;
use location::Vec2d;
use enitys::{Enity, Updates};
//use core::slice::Iter;
use std::collections::VecMap;

//add my mods here
mod location;
mod world;
mod enitys;
mod render;
mod things;


thread_local!(static root: RefCell<world::World> = RefCell::new(world::World::new()));

fn main(){
    use things::mobs::*;
    let rend_engine= init();

    let mut world : VecMap<Box<Enity>>= VecMap::new();

    {
        let a = box things::mobs::Rock::
            new("".to_string(), Vec2d::new(-20.1,-1.0));
        let b = box things::mobs::Rock::
            new("".to_string(), Vec2d::new(78.0,9.0));
        let c = box things::mobs::Rock::
            new("".to_string(), Vec2d::new(-45.0,45.0));
        let d = box things::mobs::Rock::
            new("".to_string(), Vec2d::new(23.0,-450.0));
        let e = box things::mobs::DevDan::
            new("Dan".to_string(),Vec2d::new(0.0,0.0));
        let f = box things::mobs::John::
            new("117".to_string(),Vec2d::new(-50.0,-70.0));
        let g = box things::mobs::John::
            new("104".to_string(),Vec2d::new(20.0,50.0));
        world.insert(a.ID(),a);
        world.insert(b.ID(),b);
        world.insert(c.ID(),c);
        world.insert(e.ID(),e);
        world.insert(f.ID(),f);
        world.insert(d.ID(),d);
        world.insert(g.ID(),g);
    }
    
    let mut camera = location::Vec2d::new(0.0,0.0);
    loop{
        win_events();
        let mut draws = Vec::new();
        //draw things
        ////let world = w;
        /////let ww = world.borrow();
        {
            let mut ups = Vec::new();
            for (key,val) in world.iter_mut(){
                if let Some(thin) = val.update_handle(){
                    ups.push(thin);
                }
            }
            for up in ups.iter(){
                up.update(0.01f32);
            }
        }

        for (key,val) in world.iter(){
            if let Some(thing) = val.draw_handle(){
                draws.push(thing);
            }
        }

        rend_engine.draw_frame(draws.iter(), &camera);
    }//end main loop
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

fn setup(world: &mut VecMap<Box<enitys::Enity>>){
    let a = box things::mobs::Rock::
        new("".to_string(), Vec2d::new(-20.1,-1.0));
    let b = box things::mobs::Rock::
        new("".to_string(), Vec2d::new(78.0,9.0));
    let c = box things::mobs::Rock::
        new("".to_string(), Vec2d::new(-45.0,45.0));
    let d = box things::mobs::Rock::
        new("".to_string(), Vec2d::new(23.0,-450.0));
    let e = box things::mobs::DevDan::
        new("Dan".to_string(),Vec2d::new(0.0,0.0));
    let f = box things::mobs::John::
        new("117".to_string(),Vec2d::new(-50.0,-70.0));
    let g = box things::mobs::John::
        new("104".to_string(),Vec2d::new(20.0,50.0));
    world.insert(a.ID(),a);
    world.insert(b.ID(),b);
    world.insert(c.ID(),c);
    world.insert(e.ID(),e);
    world.insert(f.ID(),f);
    world.insert(d.ID(),d);
    world.insert(g.ID(),g);
}

fn run(rend_engine: &render::Render){
    //loop{ //play loop
    //polling and handling the events received by the window
    //let world = w.borrow();

}

fn win_events(){
    use glutin::VirtualKeyCode;
    root.with(|w|{
        let world = w.borrow();
        let events = world.contex().poll_events();
        for event in events{
            match event {
                glutin::Event::Closed => panic!(),
                glutin::Event::KeyboardInput(ele_state,scan_code, vkey_code) 
                    => match vkey_code {
                        Some(x) => match x {
                            VirtualKeyCode::A => {
                                println!("things");
                                //root.with(|w| {
                                //    let ref temp =*w.borrow_mut();
                                //    let mut player = temp.thing(115); 
                                //});
                                // world_mut(|w| {
                                //     let t = w.thing(115).ID();
                                //           println!("{}",t);
                                // })

                            },
                            _ => {}
                        },
                        None => ()
                    },
                    _ => ()
            }
        }
    });
}


pub fn with<A,F>(mut f:F) -> A 
where F: FnMut(&world::World) -> A{
    root.with(|w| f(& *w.borrow()))
}

pub fn world_mut<A,F>(mut f:F) -> A
where F: FnMut(&mut world::World) -> A{
    root.with(|w| f(&mut *w.borrow_mut()))
}

