#[cfg(test)]

mod tests {
    use raytracer::{
        intersections::{Intersectable, Intersection},
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
        assert_eq!(4.0, xs1[0].time);
        assert_eq!(6.0, xs1[1].time);

        let r2 = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs2 = s.intersect(r2);
        assert_eq!(2, xs2.len());
        assert_eq!(5.0, xs2[0].time);
        assert_eq!(5.0, xs2[1].time);

        let r3 = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs3 = s.intersect(r3);
        assert_eq!(0, xs3.len());

        let r4 = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs4 = s.intersect(r4);
        assert_eq!(2, xs4.len());
        assert_eq!(-1.0, xs4[0].time);
        assert_eq!(1.0, xs4[1].time);

        let r5 = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let xs5 = s.intersect(r5);
        assert_eq!(2, xs5.len());
        assert_eq!(-6.0, xs5[0].time);
        assert_eq!(-4.0, xs5[1].time);

        let r6 = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs6 = s.intersect(r6);
        assert_eq!(2, xs6.len());
        assert_eq!(s, *xs6[0].object);
        assert_eq!(s, *xs6[1].object);
    }

    #[test]
    fn test_hit() {
        let s = Sphere::new(Point::new(0.0, 0.0, 0.0), 1.0);

        // All intersections with positive t
        let mut is1: Vec<Intersection<Sphere>> = Vec::new();
        let i11 = Intersection::new(1.0, &s);
        let i12 = Intersection::new(2.0, &s);
        is1.push(i11.clone());
        is1.push(i12);
        assert_eq!(i11, Intersection::hit(is1).unwrap());

        // Some intersections with negative t
        let mut is2: Vec<Intersection<Sphere>> = Vec::new();
        let i21 = Intersection::new(-1.0, &s);
        let i22 = Intersection::new(2.0, &s);
        is2.push(i21);
        is2.push(i22.clone());
        assert_eq!(i22, Intersection::hit(is2).unwrap());

        // All intersection with negative t
        let mut is3: Vec<Intersection<Sphere>> = Vec::new();
        let i31 = Intersection::new(-1.0, &s);
        let i32 = Intersection::new(-2.0, &s);
        is3.push(i31);
        is3.push(i32);
        assert_eq!(None, Intersection::hit(is3));

        // Test that it always returns lowest non negative intersection
        let mut is4: Vec<Intersection<Sphere>> = Vec::new();
        let i41 = Intersection::new(5.0, &s);
        let i42 = Intersection::new(7.0, &s);
        let i43 = Intersection::new(-3.0, &s);
        let i44 = Intersection::new(2.0, &s);
        is4.push(i41);
        is4.push(i42);
        is4.push(i43);
        is4.push(i44.clone());
        assert_eq!(i44, Intersection::hit(is4).unwrap());
    }
}
