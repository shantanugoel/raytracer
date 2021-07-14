use crate::{objects::Object, rays::Ray};

pub trait Intersectable {
    fn intersect(&self, r: Ray) -> Vec<Intersection>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub time: f64,
    pub object: Object,
}

impl Intersection {
    pub fn new(time: f64, object: Object) -> Self {
        Intersection { time, object }
    }

    pub fn intersections(i: Vec<Intersection>) -> Vec<f64> {
        let mut xs = Vec::<f64>::with_capacity(i.len());
        for intersection in i {
            xs.push(intersection.time);
        }
        xs
    }
}
