use crate::{
    color::{Color, CommonColor},
    lights::PointLight,
    tuple::{Point, Vector},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
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
        point: Point,
        eyev: Vector,
        normalv: Vector,
    ) -> Color {
        // Combine the surface color with the light's color/intensity
        let effective_color = self.color * light.intensity;

        // Find the direction to the light source
        let lightv = (light.position - point).normalize();

        // Compute the ambient contribution
        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(normalv);
        let mut diffuse = CommonColor::Black.value();
        let mut specular = CommonColor::Black.value();
        if light_dot_normal.ge(&0.0) {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye.gt(&0.0) {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
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
                .limit_precision(4)
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
                .limit_precision(4)
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
