extern crate glutin;
#[macro_use]
extern crate glium;

extern crate image;
use glium::{DisplayBuild, Surface};
use std::io::Cursor;

fn main(){
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Forced Fracture"))
        .build_glium().unwrap();

    let img = image::load(Cursor::new(&include_bytes!("../content/textures/actors/Full/DevDan.png")[..]),image::PNG).unwrap();

    let glTex = glium::texture::CompressedTexture2d::new(&display, img);
    
    let vertex_buffer = {
        #[derive(Copy)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2]
        }

        implement_vertex!(Vertex, position, tex_coords);
        glium::VertexBuffer::new(&display,
                                 vec![
                                     Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                                     Vertex { position: [-1.0, 1.0], tex_coords: [0.0, 1.0] },
                                     Vertex { position: [ 1.0, 1.0], tex_coords: [1.0, 1.0] },
                                     Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
                                 ] )
    };
     let index_buffer = glium::IndexBuffer::new(&display,glium::index::TriangleStrip(vec![1 as u16, 2, 0, 3]));

    let program = glium::Program::from_source(&display, r"
        #version 110
        uniform mat4 matrix;
        attribute vec2 position;
        attribute vec2 tex_coords;
        varying vec2 v_tex_coords;
        void main() {
        gl_Position = matrix * vec4(position, 0.0, 1.0);
        v_tex_coords = tex_coords;
        }
        ", r"
        #version 110
        uniform sampler2D texture;
        varying vec2 v_tex_coords;
        void main() {
        gl_FragColor = texture2D(texture, v_tex_coords);
        }
        ", None).unwrap();

    loop{
        let uniforms = uniform! {
            matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
            ],
            texture: &glTex 
        };
        
        let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &std::default::Default::default()).unwrap();
            target.finish();
            // polling and handling the events received by the window
            for event in display.poll_events() {
                match event {
                    glutin::Event::Closed => return,
                    _ => ()
                }
            }


    }
}
