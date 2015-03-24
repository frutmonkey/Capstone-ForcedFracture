extern crate glutin;
extern crate glium;

fn main() {
	use glium::DisplayBuild;
	
	let display = glutin::WindowBuilder::new()
		.with_dimensions(1024, 768)
		.with_title(format!("Hello world"))
		.build_glium().unwrap();

	loop{
		//keep up window
	}
}
