use game::game_object::Point;
use game::game_character::*;

pub struct Enemy {
    position: Point,
    health: i64,
}

enum EnemyCounstructorArgs {
    Default,
    WithPosition(Point),
    WithHealth(i64),
    WithPositionAndHealth(Point, i64)
}


impl Enemy {
    pub fn new(args: EnemyCounstructorArgs) -> Enemy {
        match args {
            Default => 
                return Enemy { 
                    position: Point::new(0f64, 0f64),
                    health: 100 
                },
            WithPosition(p) => 
                return Enemy { 
                    position: p,
                    health: 100 
                },
            WithHealth(h) => 
                return Enemy { 
                    position: Point::new(0f64, 0f64),
                    health: h 
                },
            WithPositionAndHealth(p, h) => 
                return Enemy { 
                    position: p,
                    health: h 
                }
        }
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
