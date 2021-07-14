use crate::rays::Ray;

pub trait Intersectable {
    fn intersect(&self, r: Ray) -> Vec<Intersection<Self>>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<T> {
    pub time: f64,
    pub object: T,
}

impl<T> Intersection<T>
where
    T: Intersectable,
{
    pub fn new(time: f64, object: T) -> Self {
        Intersection { time, object }
    }

    pub fn intersections(i: Vec<Intersection<T>>) -> Vec<f64> {
        let mut xs = Vec::<f64>::with_capacity(i.len());
        for intersection in i {
            xs.push(intersection.time);
        }
        xs
    }
}
