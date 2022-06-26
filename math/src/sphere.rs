use crate::ray::{Intersect, Ray};
use crate::tuple::{dot, Point, Tuple};

struct Sphere {
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

impl Intersect for Sphere {
    fn intersect(&self, r: &Ray) -> Vec<f32> {
        let origin = Tuple::from(r.origin) - Tuple::from(self.center);
        let direction = Tuple::from(r.direction);

        let a = direction.sq_len();
        let b = dot(direction, origin);
        let c = origin.sq_len() - self.radius * self.radius;

        let d = (b * b - a * c).sqrt();
        if d.is_nan() {
            Vec::new()
        } else {
            vec![(-b - d) / a, (-b + d) / a]
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ray::{Intersect, Ray};
    use crate::tuple::{Point, Vector};

    #[test]
    fn test_ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        assert_eq!(s.intersect(&r), vec![4.0, 6.0]);
    }

    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray::new(Point::new(0., 0., 5.), Vector::new(0., 0., 1.));
        let s = Sphere::new();
        assert_eq!(s.intersect(&r), vec![-6.0, -4.0]);
    }

    #[test]
    fn test_intersect_at_point() {
        let r = Ray::new(Point::new(1., 5., 0.), Vector::new(0., 1., 0.));
        let s = Sphere::new();
        assert_eq!(s.intersect(&r), vec![-5.0, -5.0]);
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
