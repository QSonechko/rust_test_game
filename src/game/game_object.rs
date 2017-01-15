
pub trait GameObject {
    fn set_position(&self, _: Point);

    fn get_position(&self) -> Point;
    fn get_render_parts(&self) -> Vec<Part>;
    fn get_collision_parts(&self) -> Vec<Part> {
        self.get_render_parts()
    }
    fn terminate_flag(&self, boundary: Point) -> bool {
        self.get_position().get_x() < boundary.get_x() || self.get_position().get_x() > boundary.get_y()
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        return Point { x: x, y: y };
    }

    pub fn floor(&self) -> Point {
        Point { 
            x: self.x.floor(),
            y: self.y.floor()
        }
    }

    pub fn mult_sclr(&self, scalar: f64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar
        }
    }

    pub fn add(&self, that: Point) -> Point {
        Point {
            x: self.x + that.get_x(),
            y: self.y + that.get_y(),
        }
    }

    pub fn sub(&self, that: Point) -> Point {
        Point {
            x: self.x - that.get_x(),
            y: self.y - that.get_y()
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }
}

pub struct Part {
    pub radial: Point,
    pub angle: Point
}
