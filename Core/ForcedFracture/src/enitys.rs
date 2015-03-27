

pub trait Enity{
    //id: uint,
    //name: str
}

pub trait Drawable{
    fn panel()->glium::uniform;
    fn location() -> location::Vec2d;
    fn size() -> f32;
}
