use game::game_object::Point;

pub trait HealthAware {
    fn set_health(&self, health: i64) -> Self;
    fn get_health(&self) -> i64;
}    

pub trait PositionAware {
    fn set_position(&self, position: Point) -> Self;
    fn get_position(&self) -> Point;
}

pub trait Accelerateable {
    fn accelerate(&self, velocity: Point) -> Self;
}

pub trait Mortal {
    fn is_alive(&self) -> bool;
}

