use std::ops::Neg;

use crate::{
    intersections::{Intersectable, Intersection},
    matrix::{Matrix, MatrixError},
    rays::Ray,
    tuple::{Point, Vector},
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

    pub fn set_transform(&mut self, m: Matrix<f64>) {
        self.transform = m;
    }

    pub fn normal_at(&self, p: Point) -> Result<Vector, MatrixError> {
        let object_point = (&self.transform.inverse()? * p)?;
        let object_normal = object_point - self.origin;
        let mut world_normal = (&self.transform.inverse()?.transpose() * object_normal)?;
        world_normal.0.w = 0.0;
        Ok(world_normal.normalize())
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, r: Ray) -> Result<Vec<Intersection<Sphere>>, MatrixError> {
        let r2 = r.transform(&self.transform.inverse()?)?;
        let sphere_to_ray = r2.origin - self.origin;
        let a = r2.direction.dot(r2.direction);
        let b = 2.0 * r2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powf(2.0) - 4.0 * a * c;

        let mut intersections: Vec<Intersection<Sphere>> = Vec::new();
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
        Ok(intersections)
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Axis;
    use crate::tuple::IsTuple;

    use super::*;

    #[test]
    fn test_sphere_default_transformation() {
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        assert_eq!(Matrix::identity(4, 1.0), s.transform);
    }

    #[test]
    fn test_sphere_normal() {
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        assert_eq!(
            Vector::new(1.0, 0.0, 0.0),
            s.normal_at(Point::new(1.0, 0.0, 0.0)).unwrap()
        );

        assert_eq!(
            Vector::new(0.0, 1.0, 0.0),
            s.normal_at(Point::new(0.0, 1.0, 0.0)).unwrap()
        );

        assert_eq!(
            Vector::new(0.0, 0.0, 1.0),
            s.normal_at(Point::new(0.0, 0.0, 1.0)).unwrap()
        );

        assert_eq!(
            Vector::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            ),
            s.normal_at(Point::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            ))
            .unwrap()
        );

        let n = s
            .normal_at(Point::new(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ))
            .unwrap();
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn test_sphere_normal_with_transformations() {
        let mut s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n1 = s.normal_at(Point::new(0.0, 1.70711, -0.70711)).unwrap();
        let e1 = Vector::new(0.0, 0.70711, -0.70711);
        assert_eq!(e1, n1.limit_precision(5));

        s.set_transform(
            Matrix::rotation(Axis::Z, std::f64::consts::PI / 5.0)
                .scale(1.0, 0.5, 1.0)
                .unwrap(),
        );
        let n2 = s
            .normal_at(Point::new(
                0.0,
                2.0_f64.sqrt() / 2.0,
                2.0_f64.sqrt().neg() / 2.0,
            ))
            .unwrap();
        let e2 = Vector::new(0.0, 0.97014, -0.24254);
        assert_eq!(e2, n2.limit_precision(5));
    }
}
