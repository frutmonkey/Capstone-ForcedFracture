

pub struct Render{
    display: glium::display,
    img_shader: glium::Program,
    img_index_org: glium::IndexBuffer
}

impl Render {
    fn new(disp: glutin::WindowBuilder) -> Render{
        img_shader = glium::Program::from_source(&display, r"
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

        
    }
}
