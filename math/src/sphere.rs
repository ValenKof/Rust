use crate::material::Material;
use crate::matrix::Matrix;
use crate::point::{point, Point};
use crate::ray::Ray;
use crate::transforms::Transform;
use crate::vector::{dot, vector, Vector};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    transform: Matrix<4, 4>,
    material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix<4, 4>) {
        self.transform = transform;
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn intersect(&self, r: &Ray) -> Vec<f32> {
        let r = r.apply(&self.transform.inverse().unwrap());
        let origin = r.origin - point(0, 0, 0);
        let direction = r.direction;

        let a = direction.sq_len();
        let b = dot(direction, origin);
        let c = origin.sq_len() - 1.0;

        let d = (b * b - a * c).sqrt();
        if d.is_nan() {
            vec![]
        } else {
            vec![(-b - d) / a, (-b + d) / a]
        }
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let object_point = world_point.apply(&self.transform.inverse().unwrap());
        let object_normal = object_point - point(0., 0., 0.);
        let world_normal = crate::tuple::Tuple::from(object_normal)
            .apply(&self.transform.inverse().unwrap().transpose());
        vector(world_normal.x, world_normal.y, world_normal.z).normalized()
    }

    pub fn material(&self) -> Material {
        self.material
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix;
    use crate::point::point;
    use crate::point::Point;
    use crate::ray::Ray;
    use crate::test_utils::*;
    use crate::transforms::{rotation_z, scaling, translation};
    use crate::vector::{vector, Vector};
    use std::f32::consts::{PI, SQRT_2};

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
        assert_eq!(s.intersect(&r), vec![3.0, 7.0]);
    }

    #[test]
    fn test_intersect_translated_sphere() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let mut s = Sphere::new();
        s.set_transform(translation(5., 0., 0.));
        assert_eq!(s.intersect(&r), vec![]);
    }

    #[test]
    fn test_normal_xyz() {
        assert_eq!(Sphere::new().normal_at(point(1, 0, 0)), vector(1, 0, 0));
        assert_eq!(Sphere::new().normal_at(point(0, 1, 0)), vector(0, 1, 0));
        assert_eq!(Sphere::new().normal_at(point(0, 0, 1)), vector(0, 0, 1));
    }

    #[test]
    fn test_normal_nonaxial() {
        let a = 1.0 / 3.0_f32.sqrt();
        let n = Sphere::new().normal_at(point(a, a, a));
        assert_near!(n, vector(a, a, a));
    }

    #[test]
    fn test_normal_of_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(translation(0., 1., 0.));
        let n = s.normal_at(point(0.0, 1.70711, -0.70711));
        assert_near!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn test_normal_of_transformedsphere() {
        let mut s = Sphere::new();
        s.set_transform(&scaling(1.0, 0.5, 1.0) * &rotation_z(PI / 5.0));
        let n = s.normal_at(point(0.0, SQRT_2 / 2.0, -SQRT_2 / 2.0));
        assert_near!(n, vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn test_default_material() {
        assert_eq!(Sphere::new().material(), Material::new());
    }

    #[test]
    fn test_assigned_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(m);
        assert_eq!(s.material(), m);
    }
}
