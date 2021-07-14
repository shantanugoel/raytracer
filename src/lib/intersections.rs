use crate::rays::Ray;

pub trait Intersectable {
    fn intersect(&self, r: Ray) -> Vec<Intersection<Self>>
    where
        Self: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a, T> {
    pub time: f64,
    pub object: &'a T,
}

impl<'a, T> Intersection<'a, T>
where
    T: Intersectable,
{
    pub fn new(time: f64, object: &'a T) -> Self {
        Intersection { time, object }
    }

    pub fn intersections(is: Vec<Intersection<T>>) -> Vec<f64> {
        let mut xs = Vec::<f64>::with_capacity(is.len());
        for i in is {
            xs.push(i.time);
        }
        xs
    }

    pub fn hit(is: Vec<Intersection<T>>) -> Option<Intersection<T>> {
        let mut result: Option<Intersection<T>> = None;
        let mut lowest_time = std::f64::MAX;
        for i in is {
            if i.time >= 0.0 && i.time <= lowest_time {
                lowest_time = i.time;
                result = Some(i);
            }
        }
        result
    }
}
