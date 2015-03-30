use glium;
use glium::texture::Texture;
//use glium::uniforms::value::IntoUniformValue;
use glium::Surface;
use glutin;
use enitys::*;
use location::*;
use std;

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

    pub fn draw_frame<'a, I: Iterator<Item = & 'a Drawable>>
        (&self, disp: &glium::backend::glutin_backend::GlutinFacade, things: I, camera: &Vec2d)
    {
        #![feature(core)] 
        use std::num::ToPrimitive; 

        let mut target = disp.draw();
        target.clear_color(25.0, 0.0, 0.0, 0.0);

        for x in things{
            let img_hight = x.panel().texture.get_height().unwrap().to_f32().unwrap();
            let h = 1.0f32 / x.size();
            let w = x.panel().texture.get_width().to_f32().unwrap()
                / x.size() / img_hight;
            
            let offset = camera - (x.location() / 100.0);
            
            let ver_buffer = glium::VertexBuffer::
                new(disp, vec![
                        BasicVertex::new(Vec2d::new(0.0, h) - offset, Vec2d::new(0.0,1.0)),
                        BasicVertex::new(Vec2d::new(w,h) - offset, Vec2d::new(1.0,1.0)),
                        BasicVertex::new(Vec2d::new(w,0.0) - offset, Vec2d::new(1.0, 0.0)),
                        BasicVertex::new(Vec2d::new(0.0,0.0) - offset, Vec2d::new(0.0,0.0)) 
                    ]);
            let mat = x.panel().matrix;
            let tex = x.panel().texture;
            let uni = uniform!{matrix: mat, texture: tex};
            
            target.draw(&ver_buffer,&self.img_index_org, &self.img_shader, &uni, &std::default::Default::default()).unwrap();
        } 

        target.finish();
    }
}

#[derive(Copy)]
struct BasicVertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}

impl BasicVertex{
    pub fn new(pos1: Vec2d, pos2: Vec2d) ->BasicVertex{
        BasicVertex{
            position: pos1.to_array(),
            tex_coords: pos2.to_array()
        }
    }
}

implement_vertex!(BasicVertex, position, tex_coords);
