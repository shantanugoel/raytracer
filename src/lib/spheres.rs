use std::ops::Neg;

use crate::tuple::Point;
use crate::{intersections::Intersection, rays::Ray};

pub struct Sphere {
    origin: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Point, radius: f64) -> Self {
        Sphere { origin, radius }
    }
}

impl Intersection for Sphere {
    fn intersect(&self, r: Ray) -> Vec<f64> {
        let sphere_to_ray = r.origin - self.origin;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * r.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        let mut intersections: Vec<f64> = Vec::with_capacity(2);
        println!("{} {} {} {}", a, b, c, discriminant);
        if discriminant.ge(&0.0) {
            intersections.push((b.neg() - discriminant.sqrt()) / (2.0 * a));
            intersections.push((b.neg() + discriminant.sqrt()) / (2.0 * a));
        }
        intersections
    }
}
