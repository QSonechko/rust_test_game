use game::game_object::Point;
use game::game_character::*;

pub struct Player {
    position: Point,
    health: i64,
}

enum PlayerCounstructorArgs {
    Default,
    WithPosition(Point),
    WithHealth(i64),
    WithPositionAndHealth(Point, i64)
}

impl Player {
    pub fn new(args: PlayerCounstructorArgs) -> Player {
        match args {
            Default => 
                return Player { 
                    position: Point::new(0f64, 0f64),
                    health: 100 
                },
            WithPosition(p) => 
                return Player { 
                    position: p,
                    health: 100 
                },
            WithHealth(h) => 
                return Player { 
                    position: Point::new(0f64, 0f64),
                    health: h 
                },
            WithPositionAndHealth(p, h) => 
                return Player { 
                    position: p,
                    health: h 
                }
        }
    }
}

impl HealthAware for Player {
    fn set_health(&self, health: i64) -> Player {
        Player {
            position: self.position,
            health: health,
        }
    }

    fn get_health(&self) -> i64 {
        self.health
    }
}

impl PositionAware for Player {
    fn set_position(&self, position: Point) -> Player {
        Player {
            position: position,
            health: self.health,
        }
    }

    fn get_position(&self) -> Point {
        self.position
    }
}

impl Mortal for Player {
    fn is_alive(&self) -> bool {
        0 < self.health
    }
}

impl Accelerateable for Player {
    fn accelerate(&self, velocity: Point) -> Player {
        Player {
            position: self.position.add(velocity),
            health: self.health,
        }
    }
}
