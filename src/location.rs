//use std::option::Option;
use std::ops::{Add, Sub, Mul, Div, Neg};
//use std::num::{NumCast};
use glium;

#[derive(Copy,Clone,Default)]
pub struct Vec2d{
    pub x: f32,
    pub y: f32,
}

impl Vec2d{
    pub fn new(inx: f32, iny: f32) -> Vec2d{
        Vec2d{
            x: inx,
            y: iny
        }
    }
}


//impl<T: Eq> Eq for Vec2d { }

impl Add for Vec2d{
    type Output = Vec2d;
    fn add(self, rhs: Vec2d) -> Vec2d { Vec2d{x: self.x + rhs.x, y: self.y + rhs.y}}
}

impl Sub<Vec2d> for Vec2d{
    type Output = Vec2d;
    fn sub(self, rhs: Vec2d) -> Vec2d { Vec2d{x:self.x - rhs.x, y:self.y - rhs.y} }
}

impl Neg<> for Vec2d{
    type Output = Vec2d;
    fn neg(self) -> Vec2d { Vec2d{x: -self.x, y: -self.y} }
}

impl Mul<f32> for Vec2d{
    type Output = Vec2d;
    fn mul(self, rhs: f32) -> Vec2d { Vec2d{x: self.x * rhs, y: self.y * rhs} }
}

impl Div<f32> for Vec2d{
    type Output = Vec2d;
    fn div(self, rhs: f32) -> Vec2d { Vec2d{x: self.x / rhs, y: self.y / rhs} }
}

impl Vec2d {
    pub fn to_array(self) -> [f32; 2] { [self.x, self.y] }
}

//#[derive(Clone)]
pub struct ImgData{
    pub matrix: [[f32; 4]; 4],
    pub texture: glium::texture::CompressedTexture2d
}

pub trait ImgVal {
    fn pull_matrix(&self)  -> &[[f32; 4]; 4];
    fn pull_texture(&self) -> &glium::texture::CompressedTexture2d;
}

impl ImgVal for ImgData{
    fn pull_matrix(&self) -> &[[f32; 4]; 4]{
        &self.matrix
    }
    fn pull_texture(&self) -> &glium::texture::CompressedTexture2d{
        &self.texture
    }
}

