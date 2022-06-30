use crate::intersect::{Intersect, Intersection};
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::transforms::Transform;
use crate::tuple::{dot, Tuple};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix<4, 4>,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix<4, 4>) {
        self.transform = transform;
    }
}

impl<'a> Intersect for &'a Sphere {
    type Output = Vec<Intersection<&'a Sphere>>;

    fn intersect(self, r: &Ray) -> Self::Output {
        let r = r.apply(&self.transform.inverse().unwrap());
        let origin = Tuple::from(r.origin);
        let direction = Tuple::from(r.direction);

        let a = direction.sq_len();
        let b = dot(direction, origin);
        let c = origin.sq_len() - 1.0;

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
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::transforms::{scaling, translation};
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

    #[test]
    fn test_default_transform() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::<4, 4>::identity());
    }

    #[test]
    fn test_change_transform() {
        let mut s = Sphere::new();
        let t = translation(2., 3., 4.);
        s.set_transform(t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn test_intersect_scaled_sphere() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let mut s = Sphere::new();
        s.set_transform(scaling(2., 2., 2.));
        assert_eq!(
            s.intersect(&r),
            vec![Intersection::new(&s, 3.0), Intersection::new(&s, 7.0)]
        );
    }

    #[test]
    fn test_intersect_translated_sphere() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let mut s = Sphere::new();
        s.set_transform(translation(5., 0., 0.));
        assert_eq!(s.intersect(&r), vec![]);
    }
}
