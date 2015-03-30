#![feature(core)] 
extern crate glutin;
#[macro_use]
extern crate glium;

extern crate image;
use glium::{DisplayBuild, Surface};
use std::io::Cursor;

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
    
    let rendEngine = render::Render::new(&display);

    let mut world: Vec<Box<enitys::Enity>> = Vec::new();
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
        rendEngine.draw_frame(&display, draws.iter(), &camera);

    }//end main loop
}
