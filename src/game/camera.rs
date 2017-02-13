use game::game_object::Point;
use game::game_character::{PositionAware, Accelerateable};
use glium::backend::Facade;

pub struct Camera<'a> {
    display: &'a Facade,
    position: Point,
    forward: Point,
    up: Point,
    mouse_locked: bool,
    //pub y_axis
    //center_position: [f64, 2],
}

impl<'a> Camera<'a> {
    pub fn new<F>(facade: &F, position: Point,
                              forward: Point, up: Point,
                              mouse_locked: bool) -> Camera where F: Facade 
    {
        Camera {
            position: position,
            forward: forward,
            up: up,
            mouse_locked: mouse_locked,
            display: facade
        }
    }

    pub fn default<F>(facade: &F) -> Camera where F: Facade {
        Camera::new(facade, Point::new(0f64, 0f64), Point::new(0f64, 0f64), 
                    Point::new(0f64, 1f64), false)
    }
}

impl<'a> PositionAware for Camera<'a> {
    fn set_position(&self, position: Point) -> Self {
        Camera {
            position: position,
            forward: self.forward,
            up: self.up,
            mouse_locked: self.mouse_locked,
            display: self.display
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

impl<'a> Accelerateable for Camera<'a> {
    fn accelerate(&self, velocity: Point) -> Self {
        Camera {
            position: self.position.add(velocity),
            forward: self.forward,
            up: self.up,
            mouse_locked: self.mouse_locked,
            display: self.display
        }
    }
}
