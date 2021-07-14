use std::{fs::File, io::Write, ops::Neg};

use num::ToPrimitive;
use raytracer::{
    canvas::Canvas,
    color::Color,
    intersections::{Intersectable, Intersection},
    rays::Ray,
    spheres::Sphere,
    tuple::{IsTuple, Point},
};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels.to_f64().unwrap();
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let shape = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y.to_f64().unwrap();
        for x in 0..canvas_pixels {
            let world_x = half.neg() + pixel_size * x.to_f64().unwrap();
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r).unwrap();
            let hit = Intersection::hit(xs);
            if hit.is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut file = File::create("castshadow.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_bytes()).unwrap();
}
