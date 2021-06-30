use crate::tuple::{Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}

pub fn tick_run() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(2.0, 1.0, 0.0),
    };
    let e = Environment {
        gravity: Vector::new(0.0, -1.0, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut i = 0;
    loop {
        i += 1;
        p = tick(&e, &p);
        println!("Iteration {}: Position: {:?}", i, p.position);
        if p.position.0.y <= 0.0 {
            break;
        };
    }
}
