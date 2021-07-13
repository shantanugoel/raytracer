use crate::rays::Ray;

pub trait Intersection {
    fn intersect(&self, r: Ray) -> Vec<f64>;
}
