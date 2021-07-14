use crate::spheres::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}
