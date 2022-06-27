use crate::intersect::{Intersect, Intersection};
use crate::ray::Ray;
use crate::tuple::{dot, Point, Tuple};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }
}

impl<'a> Intersect for &'a Sphere {
    type Output = Vec<Intersection<&'a Sphere>>;

    fn intersect(self, r: &Ray) -> Self::Output {
        let origin = Tuple::from(r.origin) - Tuple::from(self.center);
        let direction = Tuple::from(r.direction);

        let a = direction.sq_len();
        let b = dot(direction, origin);
        let c = origin.sq_len() - self.radius * self.radius;

        let d = (b * b - a * c).sqrt();
        if d.is_nan() {
            vec![]
        } else {
            vec![
                Intersection::new(self, (-b - d) / a),
                Intersection::new(self, (-b + d) / a),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intersect::Intersect;
    use crate::ray::Ray;
    use crate::tuple::{Point, Vector};

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        assert_eq!(
            s.intersect(&r),
            vec![Intersection::new(&s, 4.0), Intersection::new(&s, 6.0)]
        );
    }
    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        assert_eq!(
            s.intersect(&r),
            vec![Intersection::new(&s, -6.0), Intersection::new(&s, -4.0)]
        );
    }

    #[test]
    fn test_intersect_at_point() {
        let r = Ray::new(Point::new(1., 5., 0.), Vector::new(0., 1., 0.));
        let s = Sphere::new();
        assert_eq!(
            s.intersect(&r),
            vec![Intersection::new(&s, -5.0), Intersection::new(&s, -5.0)]
        );
    }

    #[test]
    fn test_close_miss() {
        let r = Ray::new(Point::new(1.00001, 5., 0.), Vector::new(0., 1., 0.));
        let s = Sphere::new();
        assert_eq!(s.intersect(&r), vec![]);
    }

    #[test]
    fn test_clear_miss() {
        let r = Ray::new(Point::new(2., 2., 2.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        assert_eq!(s.intersect(&r), vec![]);
    }
}
