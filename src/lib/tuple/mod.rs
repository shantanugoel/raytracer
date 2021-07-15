use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::utils::is_eq_float;

/// General Tuple to hold a point or a vector
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub trait IsTuple {
    fn tuple(&self) -> Tuple;

    fn new(x: f64, y: f64, z: f64) -> Self;
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Tuple {
        *self / self.magnitude()
    }

    fn dot(&self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: Tuple) -> Tuple {
        Tuple::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            0.0,
        )
    }

    pub fn limit_precision(&self, num_places: i32) -> Self {
        let mut t = Tuple::new(self.x, self.y, self.z, self.w);
        let factor = 10.0_f64.powi(num_places);
        t.x = (self.x * factor).round() / factor;
        t.y = (self.y * factor).round() / factor;
        t.z = (self.z * factor).round() / factor;
        t.w = (self.w * factor).round() / factor;
        t
    }
}

impl Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, rhs: Tuple) -> Self::Output {
        // assert_ne!(
        //     true,
        //     self.is_point() && rhs.is_point(),
        //     "Cannot add two points"
        // );
        Tuple::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z,
            self.w + rhs.w,
        )
    }
}

impl Sub<Tuple> for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w)
    }
}

impl Mul<Tuple> for f64 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, rhs: f64) -> Self::Output {
        Tuple::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w)
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Tuple::new(-self.x, -self.y, -self.z, -self.w)
    }
}

// Point and Vector
const POINT_VALUE: f64 = 1.0;
const VECTOR_VALUE: f64 = 0.0;

trait IsPoint {}
trait IsVector {}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub Tuple);

#[derive(Debug, Clone, Copy)]
pub struct Vector(pub Tuple);

impl IsPoint for Point {}
impl IsVector for Vector {}

impl IsTuple for Point {
    fn tuple(&self) -> Tuple {
        self.0
    }

    fn new(x: f64, y: f64, z: f64) -> Point {
        Point(Tuple {
            x,
            y,
            z,
            w: POINT_VALUE,
        })
    }
}

impl IsTuple for Vector {
    fn tuple(&self) -> Tuple {
        self.0
    }

    fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector(Tuple {
            x,
            y,
            z,
            w: VECTOR_VALUE,
        })
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        is_eq_float(&self.0.x, &other.0.x)
            && is_eq_float(&self.0.y, &other.0.y)
            && is_eq_float(&self.0.z, &other.0.z)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        is_eq_float(&self.0.x, &other.0.x)
            && is_eq_float(&self.0.y, &other.0.y)
            && is_eq_float(&self.0.z, &other.0.z)
    }
}

impl Point {
    pub fn limit_precision(&self, num_places: i32) -> Self {
        Point::from(self.0.limit_precision(num_places))
    }
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        self.0.magnitude()
    }

    pub fn normalize(&self) -> Vector {
        Vector::from(self.0.normalize())
    }

    pub fn dot(&self, other: Vector) -> f64 {
        self.0.dot(other.0)
    }

    pub fn cross(&self, other: Vector) -> Vector {
        Vector::from(self.0.cross(other.0))
    }

    pub fn limit_precision(&self, num_places: i32) -> Self {
        Vector::from(self.0.limit_precision(num_places))
    }
}

impl From<Tuple> for Vector {
    fn from(t: Tuple) -> Self {
        assert!(
            is_eq_float(&VECTOR_VALUE, &t.w),
            "Attempted implicit conversion from point to vector"
        );
        Vector::new(t.x, t.y, t.z)
    }
}

impl From<[f64; 3]> for Vector {
    fn from(s: [f64; 3]) -> Self {
        Vector::new(s[0], s[1], s[2])
    }
}

impl From<Tuple> for Point {
    fn from(t: Tuple) -> Self {
        assert!(
            is_eq_float(&POINT_VALUE, &t.w),
            "Attempted implicit conversion from vector to point"
        );
        Point::new(t.x, t.y, t.z)
    }
}

impl From<[f64; 3]> for Point {
    fn from(s: [f64; 3]) -> Self {
        Point::new(s[0], s[1], s[2])
    }
}

// Only implement Vector as rhs because 2 points cannot be added
impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point::from(self.tuple() + rhs.tuple())
    }
}

impl<T> Add<T> for Vector
where
    T: IsTuple + From<Tuple>,
{
    type Output = T;
    fn add(self, rhs: T) -> Self::Output {
        T::from(self.tuple() + rhs.tuple())
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::from(self.tuple() - rhs.tuple())
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::from(self.tuple() - rhs.tuple())
    }
}

// Cannot subtract point from vector
impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::from(self.tuple() - rhs.tuple())
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector::from(self.0 * rhs)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::from(self * rhs.0)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Vector::from(self.0 / rhs)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector::from(-self.0)
    }
}

#[cfg(test)]
mod tests;
