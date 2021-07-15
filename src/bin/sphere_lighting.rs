use std::{fs::File, io::Write, ops::Neg};

use num::ToPrimitive;
use raytracer::{
    canvas::Canvas,
    color::{Color, CommonColor},
    intersections::{Intersectable, Intersection},
    lights::PointLight,
    // matrix::Matrix,
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
    // let color = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
    shape.material.color = Color::new(0.4, 0.8, 0.4);

    // let m = Matrix::scaling(1.0, 0.5, 1.0);
    // let m = (&Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * &Matrix::scaling(0.5, 1.0, 1.0))
    //     .unwrap();
    // shape.set_transform(m);

    let light = PointLight::new(Point::new(-10.0, -10.0, -10.0), CommonColor::White.value());

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y.to_f64().unwrap();
        for x in 0..canvas_pixels {
            let world_x = half.neg() + pixel_size * x.to_f64().unwrap();
            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(r).unwrap();
            let hit = Intersection::hit(xs);
            if hit.is_some() {
                let h = hit.unwrap();
                let point = r.position(h.time);
                let normal = h.object.normal_at(point).unwrap();
                let eye = -r.direction;
                let color = h.object.material.lighting(light, point, eye, normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut file = File::create("sphere_lighting.ppm").unwrap();
    file.write_all(canvas.to_ppm().as_bytes()).unwrap();
}
