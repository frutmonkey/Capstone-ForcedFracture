use std::collections::VecMap;
use glium;
use enitys::*;
use std::collections::vec_map::Iter;

pub struct World<'a>{
   next_id: usize,
   map: VecMap<Box<Enity<'a>>>,
   gl_contex: Option<glium::backend::glutin_backend::GlutinFacade>
}

impl<'a> World<'a> {
    pub fn new() -> World<'a> {
        World {
            next_id: 0,
            map: VecMap::new(),
            gl_contex: None
        }
    }

    pub fn set_context(&mut self, disp: glium::backend::glutin_backend::GlutinFacade){
        self.gl_contex = Some(disp);
    }
    
    pub fn contex(&self) -> &glium::backend::glutin_backend::GlutinFacade{
        let x = self.gl_contex.as_ref().unwrap();
        x
    }
    
    pub fn thing(& 'a self, id : usize) -> & 'a Enity {
        &*self.map[id]
    }



    pub fn add(& mut self, e: Box<Enity<'a>>) {
        let id = e.ID();
        self.map[id] = e;
    }

    pub fn remove(&mut self, id : usize){
        self.map.remove(&id);
    }

    pub fn new_id(&mut self) -> usize {
        self.next_id += 1;
        self.next_id
    }

    pub fn all_the_things(&'a self) -> Iter<Box<Enity>>{
        self.map.iter()
    }

}
