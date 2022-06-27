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

#[cfg(test)]
mod tests {
    use super::*;
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
}
