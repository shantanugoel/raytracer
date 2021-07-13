#[cfg(test)]

mod tests {
    use raytracer::{
        intersections::Intersection,
        rays::Ray,
        spheres::Sphere,
        tuple::{IsTuple, Point, Vector},
    };

    #[test]
    fn test_sphere_ray_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0]);
        assert_eq!(6.0, xs[1]);
    }
}
