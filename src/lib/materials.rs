use crate::{
    color::Color,
    lights::PointLight,
    tuple::{Point, Vector},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Material {
    pub fn lighting(
        &self,
        light: PointLight,
        position: Point,
        eyev: Vector,
        normalv: Vector,
    ) -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        lights::PointLight,
        tuple::{IsTuple, Point, Vector},
    };

    use super::*;

    #[test]
    // Eye between light and suface
    fn test_lighting_1() {
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let position = Point::new(0.0, 0.0, 0.0);
        let m = Material::default();
        assert_eq!(
            Color::new(1.9, 1.9, 1.9),
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    // Eye at a point 45 deg. off of the normal
    fn test_lighting_2() {
        let eyev = Vector::new(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let position = Point::new(0.0, 0.0, 0.0);
        let m = Material::default();
        assert_eq!(
            Color::new(1.0, 1.0, 1.0),
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    // Eye opposite surface and light at a point 45 deg. off of the normal
    fn test_lighting_3() {
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let position = Point::new(0.0, 0.0, 0.0);
        let m = Material::default();
        assert_eq!(
            Color::new(0.7364, 0.7364, 0.7364),
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    // Eye in the path of the reflection vector
    fn test_lighting_4() {
        let eyev = Vector::new(0.0, -2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let position = Point::new(0.0, 0.0, 0.0);
        let m = Material::default();
        assert_eq!(
            Color::new(1.6364, 1.6364, 1.6364),
            m.lighting(light, position, eyev, normalv)
        );
    }

    #[test]
    // Light behind the surface
    fn test_lighting_5() {
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let position = Point::new(0.0, 0.0, 0.0);
        let m = Material::default();
        assert_eq!(
            Color::new(0.1, 0.1, 0.1),
            m.lighting(light, position, eyev, normalv)
        );
    }
}
