use game::game_object::Point;
use game::game_character::*;

pub struct Enemy {
    position: Point,
    health: i64,
}

impl Enemy {
    pub fn new(position: Point, health: i64) -> Enemy {
        return Enemy { 
            position: Point::new(0f64, 0f64),
            health: 100i64,
        }
    }

    pub fn with_position(pos: Point) -> Enemy {
        Enemy::new(pos, 100i64)
    }

    pub fn with_health(health: i64) -> Enemy {
        Enemy::new(Point::new(0f64, 0f64), health)
    }

    pub fn default() -> Enemy {
        Enemy::new(Point::new(0f64, 0f64), 100i64)
    }
}

impl HealthAware for Enemy {
    fn set_health(&self, health: i64) -> Enemy {
        Enemy {
            position: self.position,
            health: health,
        }
    }

    fn get_health(&self) -> i64 {
        self.health
    }
}

impl PositionAware for Enemy {
    fn set_position(&self, position: Point) -> Enemy {
        Enemy {
            position: position,
            health: self.health,
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

impl Mortal for Enemy {
    fn is_alive(&self) -> bool {
        0 < self.health
    }
}

impl Accelerateable for Enemy {
    fn accelerate(&self, velocity: Point) -> Enemy {
        Enemy {
            position: self.position.add(velocity),
            health: self.health,
        }
    }
}
