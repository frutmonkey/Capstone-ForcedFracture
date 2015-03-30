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

    //let raw_rock = image::load(Cursor::new(&include_bytes!("../content/textures/blocks/surface rock.png")[..]),image::PNG).unwrap();
    //let raw_dan = image::load(Cursor::new(&include_bytes!("../content/textures/actors/Full/DevDan.png")[..]),image::PNG).unwrap();
    //println!("loaded text");
    //let rock_tex = glium::texture::CompressedTexture2d::new(&display, raw_rock);
    //let dan_tex = glium::texture::CompressedTexture2d::new(&display, raw_dan);
    //

    //let uni_rock = uniform! {
    //    matrix: [
    //        [1.0, 0.0, 0.0, 0.0],
    //        [0.0, 1.0, 0.0, 0.0],
    //        [0.0, 0.0, 1.0, 0.0],
    //        [0.0, 0.0, 0.0, 1.0f32]
    //    ], texture: &rock_tex};
    //let uni_dan = uniform! {
    //    matrix: [
    //        [1.0, 0.0, 0.0, 0.0],
    //        [0.0, 1.0, 0.0, 0.0],
    //        [0.0, 0.0, 1.0, 0.0],
    //        [0.0, 0.0, 0.0, 1.0f32]
    //    ], texture: &dan_tex};

    let mut world: Vec<Box<enitys::Enity>> = Vec::new();
    let mut camera = location::Vec2d::new(0.0,0.0);

    loop{ //play loop
        let mut target = display.draw();
        
        let mut draws = Vec::new();
        for x in world.iter(){
            let temp = x.draw_handle();
            if let Some(thing) = temp {
                draws.push(thing);
            }
        }
        
        rendEngine.draw_frame(&display, draws.iter(), &camera);
        
        //polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }
    }//end main loop
}
