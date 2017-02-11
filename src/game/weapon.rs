use game::game_object::Point;
use game::game_character::*;

/* 
 * Traits 
 */
pub trait Weapon {
    fn get_damage(&self) -> i64 {
        self.damage
    }

    fn atack(&self, angle: Point) -> Self; 
}

pub trait HasBullets {
    fn get_bullets_left(&self) -> i64;
    fn get_max_bullets() -> i64;
}

/* 
 * Weapon structs 
 */
pub struct FireArm {
    bullets: Vec<Bullet>,
    damage: i64,
}

pub struct Chainsaw {
    damage: i64,
}

/*
 * Impls
 */
impl HasBullets for FireArm {
    pub fn get_bullets_left(&self) -> i64 {
        self.bullets.len()
    }

    // TODO: will keep it static for now
    pub fn get_max_bullets() -> i64 {
        // TODO: fix this
        100
    }

    pub fn atack(&self, angle: Point) ->FireArm {
        let bullet = self.bullets.pop();
        bullet.accelerate(angle);
        // TODO: finish him
    }
}

impl FireArm {
    pub fn new() -> FireArm {
        let mut bullets = Vec::new();
        for i in FireArm::get_max_bullets() {
            bullets.push(Bullet::new(Point::new(0f64, 0f64)));
        }

        FireArm { damage: 1, bullets: bullets }
    }
}

impl Weapon for Chainsaw {
    pub fn get_damage(&self) -> i64 {
        self.damage
    }

    pub fn attack(&self, angle: Point) -> Chainsaw {
        // FIXME: 
        self
    }
}
