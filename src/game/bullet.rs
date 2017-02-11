use game::game_character::*;

pub struct Bullet {
   position: Point,
}

impl PositionAware for Bullet {
    fn set_position(&self, position: Point) -> Bullet {
        Bullet { position: position }
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

impl Accelerateable for Bullet {
    fn accelerate(&self, velocity: Point) -> Bullet {
        Bullet { position: position.add(velocity) }
    }
}

impl Bullet {
    fn new(position: Point) -> Bullet {
        Bullet { position: position }
    }
}
