use glium;
use glutin;
use enitys::*;

use std::boxed::Box;
use std::iter::Iterator;

pub struct Render{
    //display: glium::Display,
    img_shader: glium::Program,
    img_index_org: glium::IndexBuffer
}

impl Render {
    pub fn new<F>(disp: &F) -> Render where F: glium::backend::Facade {
        let prog = glium::Program::from_source(disp, r"
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
            }", None).unwrap();

        let index_buffer = glium::IndexBuffer::new(disp,glium::index::TriangleStrip(vec![1 as u16, 2, 0, 3]));
        
        Render{
            //display: disp,
            img_shader: prog,
            img_index_org: index_buffer
        }
    }

    pub fn draw_frame<I: Iterator<Item = Box<Drawable>>>(disp: &glium::backend::glutin_backend::GlutinFacade, things: I)
        {
        
        let mut target = disp.draw();

        for x in things{
            
            
    

        }   
    }
}
