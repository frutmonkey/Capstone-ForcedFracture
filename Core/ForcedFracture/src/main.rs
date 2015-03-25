extern crate glutin;
extern crate glium;

extern crate image;
use std::old_io::BufReader;
use glium::{DisplayBuild, Surface};

fn main(){
    use glium::DisplayBuild;

    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Forced Fracture"))
        .build_glium().unwrap();

    let image = image::load(BufReader::new(include_bytes!("../content/textures/actors/Full/DevDan.png")),image::PNG).unwrap();

    let glTex = glium::texture::CompressedTexture2d::new(&display, image);
    
    let vertex_buffer = {
        #[derive(Copy)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
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

    
}
