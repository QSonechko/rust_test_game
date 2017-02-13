extern crate test_project;
#[macro_use]
extern crate glium;
extern crate image;

//use test_project::circle::Circle;
use test_project::shape::vertex::Vertex;
use test_project::game::camera::Camera;
use test_project::game::game_object::Point;
use test_project::game::game_character::{PositionAware, Accelerateable};

use glium::Surface;
use glium::DisplayBuild;
use std::io::Cursor;

fn main() {
    let vertex1 = Vertex { position: [-0.001, -0.001], tex_coords: [0.0, 0.0] };
    //let vertex2 = Vertex { position: [0.0, 0.5], tex_coords: [0.0, 1.0] };
    //let vertex3 = Vertex { position: [0.5, -0.25], tex_coords: [1.0, 0.0] };
    //let shape = vec![vertex1, vertex2, vertex3];
    let mut shape = vec![vertex1];
    let triangle_amount: i32 = 10000;
    let twice_pi: f32 = 2.0 * std::f32::consts::PI;
    let x = -0.001;
    let y = -0.001;
    let tex_coord_x = 1.0;
    let tex_coord_y = 1.0;
    let radius = 0.5;

    for i in 0..triangle_amount {
        let tmp = (i as f32) * twice_pi / (triangle_amount as f32); 
        let vrtx = Vertex {
            position: [x + (radius * tmp.cos()), y + (radius * tmp.sin())],
            tex_coords: [tex_coord_x + (radius * tmp.cos()), tex_coord_y + (radius * tmp.sin())] 
        }; 
        shape.push(vrtx);
    }

    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title(format!("Helo world"))
        .build_glium()
        .unwrap();

    let camera = Camera::default(&display);
    println!("CAMERA POSITION: {}", camera.get_position());
    let pos = camera.set_position(Point::new(1f64, 1f64))
             .accelerate(Point::new(1f64, 0f64))
             .get_position();
    println!("NEW CAMERA POS: {}", pos);

    let image = image::load(Cursor::new(&include_bytes!("../images/doom.png")[..]),
                                        image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        image.into_raw(), 
        image_dimensions
    );

    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indicies = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

    let program = glium::Program::from_source(
        &display,
        include_str!("shaders/circle.vs"),
        include_str!("shaders/circle.frag"),
        None
    ).unwrap();

    let mut t: f32 = -1.0;
    let step: f32 = 0.1;


    loop {
        if t > 10.0 {
            t /= 1.0;
        }
        
        t += step;

        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };

        let mut target: glium::Frame = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);

        target.draw(
            &vertex_buffer,
            &indicies,
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }

        std::thread::sleep(std::time::Duration::new(1 / 50, 0));
    }
}
