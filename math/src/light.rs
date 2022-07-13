use crate::color::Color;
use crate::point::Point;

#[derive(PartialEq, Debug)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::point;

    #[test]
    fn test_create_point_light() {
        let position = point(0., 0., 0.);
        let intensity = Color::new(1., 1., 1.);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
