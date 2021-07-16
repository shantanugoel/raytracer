use crate::spheres::Sphere;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Sphere(Sphere),
}
