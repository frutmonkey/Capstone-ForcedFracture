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

use glium;
use std::io::Cursor;
use std::string::String;
use location::*;
use enitys::*;

use image;
pub struct DevDan{
       full_img: ImgData,
       pos: Vec2d,
       name: String
}
//extern display:glutin::WindowBuilder;

impl<'a> DevDan{
    
    pub fn new(in_name: String, start_pos: Vec2d, disp: &glium::backend::glutin_backend::GlutinFacade)-> DevDan{
        let raw_dan = image::load(Cursor::new(&include_bytes!("../../content/textures/actors/Full/DevDan.png")[..]),image::PNG).unwrap();
        let dan_tex = glium::texture::CompressedTexture2d::new(disp, raw_dan);
        let composite = ImgData{ matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0, 0.0, 0.0, 1.0f32] ],
                                texture: dan_tex };

        DevDan {
            full_img: composite,
            pos: start_pos,
            name : in_name
        }
    }
}

impl<'a> Enity<'a> for DevDan{
    
    fn name(&self) -> &str{
        self.name.as_slice()
    }

    fn parent_id(&self)-> usize{0}
    
    fn draw_handle(& 'a self) -> Option<& 'a Drawable>{
        let x:& 'a Drawable = self;
        Some(x)
    }
}

impl<'a> Drawable for DevDan{
    
    fn panel(&self) -> &ImgVal{
       let x: &ImgVal = &self.full_img; 
        x
    }
    fn location(&self) -> Vec2d{
        self.pos
    }
    fn size(&self) -> f32 { 0.25 }
}
