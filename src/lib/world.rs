use crate::{lights::PointLight, objects::Object};

#[derive(Debug, Clone, PartialEq)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
}

impl World {
    pub fn new(objects: Vec<Object>, lights: Vec<PointLight>) -> Self {
        World { objects, lights }
    }
}

#[cfg(test)]
mod tests {
    // use crate::{
    //     color::Color,
    //     spheres::Sphere,
    //     tuple::{IsTuple, Point},
    // };

    use super::*;

    #[test]
    fn test_default() {
        let w = World::default();
        assert_eq!(0, w.objects.len());
        assert_eq!(0, w.lights.len());
    }

    // #[test]
    // fn test_create() {
    //     let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
    //     let s1 = Sphere::default();
    //     let s2 = Sphere::default();
    //     let mut v_s = Vec::new();
    //     v_s.push(s1);
    //     v_s.push(s2);
    //     let mut v_l = Vec::new();
    //     v_l.push(light);
    //     let w = World::new(v_s, v_l);
    // }
}
