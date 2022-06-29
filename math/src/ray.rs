use crate::matrix::Matrix;
use crate::transforms::Transform;
use crate::tuple::{Point, Tuple, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f32) -> Point {
        Point::try_from(Tuple::from(self.origin) + Tuple::from(self.direction) * t).unwrap()
    }
}

impl Transform for Ray {
    fn apply(&self, t: &Matrix<4, 4>) -> Self {
        Ray::new(
            Point::try_from(Tuple::from(self.origin).apply(t)).unwrap(),
            Vector::try_from(Tuple::from(self.direction).apply(t)).unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transforms::Transform;
    use crate::tuple::{Point, Vector};

    #[test]
    fn test_create_ray() {
        let origin = Point::new(1., 2., 3.);
        let direction = Vector::new(4., 5., 6.);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn test_query_ray_position() {
        let r = Ray::new(Point::new(2., 3., 4.), Vector::new(1., 0., 0.));
        assert_eq!(r.position(0.0), Point::new(2., 3., 4.));
        assert_eq!(r.position(1.0), Point::new(3., 3., 4.));
        assert_eq!(r.position(-1.0), Point::new(1., 3., 4.));
        assert_eq!(r.position(2.5), Point::new(4.5, 3., 4.));
    }

    #[test]
    fn test_translate_ray() {
        let r = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let r2 = r.translate(3., 4., 5.);
        assert_eq!(r2.origin, Point::new(4., 6., 8.));
        assert_eq!(r2.direction, Vector::new(0., 1., 0.));
    }

    #[test]
    fn test_scale_ray() {
        let r = Ray::new(Point::new(1., 2., 3.), Vector::new(0., 1., 0.));
        let r2 = r.scale(2., 3., 4.);
        assert_eq!(r2.origin, Point::new(2., 6., 12.));
        assert_eq!(r2.direction, Vector::new(0., 3., 0.));
    }
}
