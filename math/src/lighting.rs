use crate::color::Color;
use crate::light::PointLight;
use crate::material::Material;
use crate::tuple::{dot, Tuple};

pub fn phong(m: &Material, l: &PointLight, pos: Tuple, v: Tuple, n: Tuple) -> Color {
    let i = m.color * l.intensity;

    let lm = (l.position - pos).normalize();
    let rm = (-lm).reflect(n);

    let ambient_lighting = m.ambient * i;

    let diffuse_lighting = {
        let x = dot(lm, n);
        if x > 0.0 {
            m.diffuse * x * i
        } else {
            Color::black()
        }
    };

    let specular_lighting = {
        let x = dot(rm, v);
        if x > 0.0 {
            m.specular * x.powf(m.shininess) * l.intensity
        } else {
            Color::black()
        }
    };

    ambient_lighting + diffuse_lighting + specular_lighting
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::test_utils::*;
    use std::f32::consts::FRAC_1_SQRT_2;

    #[test]
    fn test_lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let pos = point(0, 0, 0);
        let eye_vec = vector(0, 0, -1);
        let normal_vec = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, -10), Color::new(1., 1., 1.));
        let result = phong(&m, &light, pos, eye_vec, normal_vec);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn test_lighting_with_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let pos = point(0, 0, 0);
        let eye_vec = Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let normal_vec = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, -10), Color::new(1., 1., 1.));
        let result = phong(&m, &light, pos, eye_vec, normal_vec);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn test_lighting_with_eye_opposite_surface_light_offset_45() {
        let m = Material::new();
        let pos = point(0, 0, 0);
        let eye_vec = vector(0, 0, -1);
        let normal_vec = vector(0, 0, -1);
        let light = PointLight::new(point(0, 10, -10), Color::new(1., 1., 1.));
        let result = phong(&m, &light, pos, eye_vec, normal_vec);
        assert_near!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn test_lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::new();
        let pos = point(0, 0, 0);
        let eye_vec = Tuple::vector(0.0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let normal_vec = vector(0, 0, -1);
        let light = PointLight::new(point(0, 10, -10), Color::new(1., 1., 1.));
        let result = phong(&m, &light, pos, eye_vec, normal_vec);
        assert_near!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn test_lighting_with_light_behind_surface() {
        let m = Material::new();
        let pos = point(0, 0, 0);
        let eye_vec = vector(0, 0, -1);
        let normal_vec = vector(0, 0, -1);
        let light = PointLight::new(point(0, 0, 10), Color::new(1., 1., 1.));
        let result = phong(&m, &light, pos, eye_vec, normal_vec);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
