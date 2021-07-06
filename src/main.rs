mod canvas;
mod color;
mod matrix;
mod projectiles;
mod tuple;
mod utils;

use std::{fs::File, io::Write};

use num::ToPrimitive;

use canvas::Canvas;
use color::Color;
use projectiles::{tick, Environment, Projectile};
use tuple::{Point, Vector};

fn projectile_project() {
    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1.0, 0.0, 0.0);
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25_f64,
    };
    let e = Environment {
        gravity: Vector::new(0.0, -1.0, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    loop {
        let x = p
            .position
            .0
            .x
            .to_usize()
            .expect("Could not convert position x from f64 to usize");
        let y = canvas.height()
            - p.position
                .0
                .y
                .to_usize()
                .expect("Could not convert position x from f64 to usize");
        canvas.write_pixel(x, y, color);
        p = tick(&e, &p);
        if p.position.0.y <= 0.0 {
            break;
        };
    }

    let mut file = File::create("projectile.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_bytes()).unwrap();
}

fn main() {
    projectile_project();
}
