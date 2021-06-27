use std::ops::{Add, Div, Mul, Neg, Sub};

#[allow(unused)]

/// General Tuple to hold a point or a vector
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

pub trait IsTuple {
    fn tuple(self: &Self) -> Tuple;
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn magnitude(self: &Self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(self: &Self) -> Tuple {
        *self / self.magnitude()
    }

    fn dot(self: &Self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(self: &Self, other: Tuple) -> Tuple {
        Tuple::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
            0.0,
        )
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point(Tuple);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector(Tuple);

impl IsPoint for Point {}
impl IsVector for Vector {}

impl IsTuple for Point {
    fn tuple(self: &Self) -> Tuple {
        self.0
    }
}

impl IsTuple for Vector {
    fn tuple(self: &Self) -> Tuple {
        self.0
    }
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Point {
        Point(Tuple {
            x,
            y,
            z,
            w: POINT_VALUE,
        })
    }
}

impl Vector {
    fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector(Tuple {
            x,
            y,
            z,
            w: VECTOR_VALUE,
        })
    }

    fn magnitude(self: &Self) -> f64 {
        self.0.magnitude()
    }

    fn normalize(self: &Self) -> Vector {
        Vector::from(self.0.normalize())
    }

    fn dot(self: &Self, other: Vector) -> f64 {
        self.0.dot(other.0)
    }

    fn cross(self: &Self, other: Vector) -> Vector {
        Vector::from(self.0.cross(other.0))
    }
}

impl From<Tuple> for Vector {
    fn from(t: Tuple) -> Self {
        assert_eq!(
            VECTOR_VALUE, t.w,
            "Attempted implicit conversion from point to vector"
        );
        Vector::new(t.x, t.y, t.z)
    }
}

impl From<Tuple> for Point {
    fn from(t: Tuple) -> Self {
        assert_eq!(
            POINT_VALUE, t.w,
            "Attempted implicit conversion from vector to point"
        );
        Point::new(t.x, t.y, t.z)
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
