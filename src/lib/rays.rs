use crate::{
    matrix::{Matrix, MatrixError},
    tuple::{Point, Vector},
};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix<f64>) -> Result<Self, MatrixError> {
        Ok(Ray {
            origin: (m.clone() * self.origin)?,
            direction: (m.clone() * self.direction)?,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::tuple::IsTuple;

    use super::*;

    #[test]
    fn test_position() {
        let ray = Ray::new(Point::new(2., 3., 4.), Vector::new(1., 0., 0.));
        assert_eq!(Point::new(2., 3., 4.), ray.position(0.));
        assert_eq!(Point::new(3., 3., 4.), ray.position(1.));
        assert_eq!(Point::new(1., 3., 4.), ray.position(-1.));
        assert_eq!(Point::new(4.5, 3., 4.), ray.position(2.5));
    }

    #[test]
    fn test_translate_ray() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let ray2 = ray.transform(m).unwrap();
        assert_eq!(Point::new(4.0, 6.0, 8.0), ray2.origin);
        assert_eq!(Vector::new(0.0, 1.0, 0.0), ray2.direction);
    }

    #[test]
    fn test_scale_ray() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let ray2 = ray.transform(m).unwrap();
        assert_eq!(Point::new(2.0, 6.0, 12.0), ray2.origin);
        assert_eq!(Vector::new(0.0, 3.0, 0.0), ray2.direction);
    }
}
