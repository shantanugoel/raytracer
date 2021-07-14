use std::ops::Neg;

use crate::{
    intersections::{Intersectable, Intersection},
    matrix::Matrix,
    rays::Ray,
    tuple::Point,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    origin: Point,
    radius: f64,
    transform: Matrix<f64>,
}

impl Sphere {
    pub fn new(origin: Point, radius: f64) -> Self {
        let m = Matrix::identity(4, 1.0);
        Sphere {
            origin,
            radius,
            transform: m,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, r: Ray) -> Vec<Intersection<Sphere>> {
        let sphere_to_ray = r.origin - self.origin;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * r.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        let mut intersections: Vec<Intersection<Sphere>> = Vec::new();
        println!("{} {} {} {}", a, b, c, discriminant);
        if discriminant.ge(&0.0) {
            intersections.push(Intersection::new(
                (b.neg() - discriminant.sqrt()) / (2.0 * a),
                self,
            ));
            intersections.push(Intersection::new(
                (b.neg() + discriminant.sqrt()) / (2.0 * a),
                self,
            ));
        }
        intersections
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::IsTuple;

    use super::*;

    #[test]
    fn test_sphere_default_transformation() {
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        assert_eq!(Matrix::identity(4, 1.0), s.transform);
    }
}
