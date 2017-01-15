
pub struct Circle {
	radius: f32
}

impl Circle
{
    pub fn new(r: f32) -> Circle {
        return Circle { radius: r }
    }

    pub fn area(&self) -> f32 {
        3.14 * self.radius * self.radius
    }
}

