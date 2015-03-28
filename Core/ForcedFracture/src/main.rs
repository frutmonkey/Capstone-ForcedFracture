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
    println!("hello dark and scarry world");
    #[derive(Copy)]
    struct Vertex {
        position: [f32; 2],
        tex_coords: [f32; 2]
    }

    implement_vertex!(Vertex, position, tex_coords);


    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Forced Fracture"))
        .build_glium().unwrap();

    let raw_rock = image::load(Cursor::new(&include_bytes!("../content/textures/blocks/surface rock.png")[..]),image::PNG).unwrap();
    let raw_dan = image::load(Cursor::new(&include_bytes!("../content/textures/actors/Full/DevDan.png")[..]),image::PNG).unwrap();
    println!("loaded text");
    let rock_tex = glium::texture::CompressedTexture2d::new(&display, raw_rock);
    let dan_tex = glium::texture::CompressedTexture2d::new(&display, raw_dan);
    let vb1 = glium::VertexBuffer::new(&display,
                                 vec![
                                 Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] },
                                 Vertex { position: [-0.5, 0.5], tex_coords: [0.0, 1.0] },
                                 Vertex { position: [ 0.5, 0.5], tex_coords: [1.0, 1.0] },
                                 Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] }
                                 ] );
    

    let index_buffer = glium::IndexBuffer::new(&display,glium::index::TriangleStrip(vec![1 as u16, 2, 0, 3]));

  
    let uni_rock = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ], texture: &rock_tex};
    let uni_dan = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ], texture: &dan_tex};

    let mut t = 0.0;
    
    loop{ //play loop
        let vb2 = glium::VertexBuffer::new(&display,
                                           vec![
                                Vertex { position: [-0.5, 1.0 - t], tex_coords: [0.0, 1.0] },
                                Vertex { position: [ 0.5, 1.0 - t], tex_coords: [1.0, 1.0] },
                                Vertex { position: [ 0.5, -0.5 - t], tex_coords: [1.0, 0.0] },
                                Vertex { position: [-0.5, -0.5 - t], tex_coords: [0.0, 0.0] }
                                           ] );
        let mut target = display.draw();
       // target.clear_color(25.0, 0.0, 0.0, 0.0);
       // target.draw(&vb1, &index_buffer, &program, &uni_rock, &std::default::Default::default()).unwrap();
       // target.draw(&vb2,&index_buffer,&program, &uni_dan, &std::default::Default::default()).unwrap();
       // target.finish();
        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }
        
        t += 0.01;
    }
}
