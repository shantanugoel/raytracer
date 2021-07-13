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
        let r1 = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);
        let xs1 = s.intersect(r1);
        assert_eq!(2, xs1.len());
        assert_eq!(4.0, xs1[0]);
        assert_eq!(6.0, xs1[1]);

        let r2 = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs2 = s.intersect(r2);
        assert_eq!(2, xs2.len());
        assert_eq!(5.0, xs2[0]);
        assert_eq!(5.0, xs2[1]);

        let r3 = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs3 = s.intersect(r3);
        assert_eq!(0, xs3.len());

        let r4 = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs4 = s.intersect(r4);
        assert_eq!(2, xs4.len());
        assert_eq!(-1.0, xs4[0]);
        assert_eq!(1.0, xs4[1]);

        let r5 = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let xs5 = s.intersect(r5);
        assert_eq!(2, xs5.len());
        assert_eq!(-6.0, xs5[0]);
        assert_eq!(-4.0, xs5[1]);
    }
}
