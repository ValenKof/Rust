use crate::matrix::Matrix;
use crate::point::Point;
use crate::transforms::Transform;
use crate::vector::Vector;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}

impl Transform for Ray {
    fn apply(&self, t: &Matrix<4, 4>) -> Self {
        Ray::new(self.origin.apply(t), self.direction.apply(t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::point;
    use crate::transforms::Transform;
    use crate::vector::vector;

    #[test]
    fn test_create_ray() {
        let origin = point(1., 2., 3.);
        let direction = vector(4., 5., 6.);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn test_query_ray_position() {
        let r = Ray::new(point(2., 3., 4.), vector(1., 0., 0.));
        assert_eq!(r.position(0.0), point(2., 3., 4.));
        assert_eq!(r.position(1.0), point(3., 3., 4.));
        assert_eq!(r.position(-1.0), point(1., 3., 4.));
        assert_eq!(r.position(2.5), point(4.5, 3., 4.));
    }

    #[test]
    fn test_translate_ray() {
        let r = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
        let r2 = r.translate(3., 4., 5.);
        assert_eq!(r2.origin, point(4., 6., 8.));
        assert_eq!(r2.direction, vector(0., 1., 0.));
    }

    #[test]
    fn test_scale_ray() {
        let r = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
        let r2 = r.scale(2., 3., 4.);
        assert_eq!(r2.origin, point(2., 6., 12.));
        assert_eq!(r2.direction, vector(0., 3., 0.));
    }
}
