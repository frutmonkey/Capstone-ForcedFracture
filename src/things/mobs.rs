use glium;
use std::io::Cursor;
use std::string::String;
use location::*;
use enitys::*;
use world::*;
use std::cell::RefCell;
//use collections::borrow::BorrowMut;
use image;
pub struct DevDan{
       full_img: ImgData,
       pos: Vec2d,
       name: String,
       id: usize,
       velo: Vec2d
}
//extern display:glutin::WindowBuilder;

impl DevDan{
    
    pub fn new(in_name: String, start_pos: Vec2d)-> DevDan{
        let raw = image::load(Cursor::new(&include_bytes!("../../content/textures/actors/Full/DevDan.png")[..]),image::PNG).unwrap();
        
        let tex = ::root.with(|w| glium::texture::CompressedTexture2d
            ::new(w.borrow().contex(), raw));
        let composite = ImgData{ matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0, 0.0, 0.0, 1.0f32] ],
                                texture: tex };
        //let id = ::root.with(|w| { w.borrow_mut().new_id()});
        let id = 115;

        DevDan {
            full_img: composite,
            pos: start_pos,
            name : in_name,
            id: id,
            velo: Vec2d::new(500.0,0.0)
        }
    }
}

impl Enity for DevDan{
    
    fn name(&self) -> &str{
        self.name.as_slice()
    }

    fn ID(&self) -> usize{
        self.id
    }

    fn parent_id(&self)-> usize{0}
    
    fn draw_handle(&self) -> Option<&Drawable>{
        let x:&Drawable = self;
        Some(x)
    }

    fn update_handle(&mut self) -> Option<&mut Updates>{
        let x: &mut Updates = self;
        Some(x)
    }
}

impl Drawable for DevDan{
    
    fn panel(&self) -> &ImgVal{
       let x: &ImgVal = &self.full_img; 
        x
    }
    fn location(&self) -> Vec2d{
        self.pos
    }
    fn size(&self) -> f32 { 0.25f32 }
}

impl Updates for DevDan{
    fn update(&mut self,time: f32){
        self.pos = self.pos + (self.velo * time);
        self.velo = Vec2d::new(0.0,0.0);
    }
    
    fn change_velo(&mut self, vel: Vec2d){
        self.velo = self.velo + vel;
    }
}

    

pub struct John{
     full_img: ImgData,
     pos: Vec2d,
     name: String,
     id: usize
}

impl John{
    pub fn new(in_name: String, start_pos: Vec2d)-> John{
        let raw = image::load(Cursor::new(&include_bytes!("../../content/textures/actors/Full/idle.png")[..]),image::PNG).unwrap();
        let tex = ::root.with(|w| glium::texture::CompressedTexture2d
            ::new(w.borrow().contex(), raw));
        let composite = ImgData{ matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0, 0.0, 0.0, 1.0f32] ],
                                texture: tex };

        John{
            full_img: composite,
            pos: start_pos,
            name : in_name,
            id: ::root.with(|w| w.borrow_mut().new_id())
        }
    }
}

impl Enity for John{
    fn name(&self)-> &str{
        self.name.as_slice()
    }

    fn parent_id(&self) -> usize{ 0 }

    fn ID(&self) -> usize{ self.id }

    fn draw_handle(&self) -> Option<&Drawable>{
        let x:&Drawable = self;
        Some(x)
    }
}

impl Drawable for John{
    fn panel(&self) -> &ImgVal{
        let x: &ImgVal = &self.full_img;
        x
    }
    fn location(&self) -> Vec2d{
        self.pos
    }
    fn size(&self) -> f32 { 0.25f32 }
}



pub struct Rock{
     full_img: ImgData,
     pos: Vec2d,
     name: String,
     id: usize
}

impl Rock{
     pub fn new(in_name: String, start_pos: Vec2d)-> Rock{
        let raw = image::load(Cursor::new(&include_bytes!("../../content/textures/blocks/surface rock.png")[..]),image::PNG).unwrap();
        let tex = ::root.with(|w| glium::texture::CompressedTexture2d
            ::new(w.borrow().contex(), raw));

        let composite = ImgData{ matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [0.0, 0.0, 0.0, 1.0f32] ],
                                texture: tex };

        Rock{
            full_img: composite,
            pos: start_pos,
            name : in_name,
            id: ::root.with(|w| w.borrow_mut().new_id())
        }
    }
}

impl Enity for Rock{
    fn name(&self)-> &str{
        self.name.as_slice()
    }

    fn parent_id(&self) -> usize{ 0 }
    
    fn ID(&self)-> usize { self.id }
    fn draw_handle(&self) -> Option<&Drawable>{
        let x:&Drawable = self;
        Some(x)
    }
}

impl Drawable for Rock{
    fn panel(&self) -> &ImgVal{
        let x: &ImgVal = &self.full_img;
        x
    }
    fn location(&self) -> Vec2d{
        self.pos
    }
    fn size(&self) -> f32 { 0.20f32 }
}    
