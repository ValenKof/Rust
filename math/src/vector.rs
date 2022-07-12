#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Vector {
        Vector { x, y, z }
    }

    pub fn sq_len(self) -> f32 {
        dot(self, self)
    }

    pub fn len(self) -> f32 {
        self.sq_len().sqrt()
    }

    pub fn normalized(self) -> Vector {
        self / self.len()
    }

    pub fn reflected(self, normal: Vector) -> Vector {
        self - normal * (2. * dot(self, normal))
    }
}

// TODO: return Vector.
pub fn vector<T: crate::F32Const>(x: T, y: T, z: T) -> crate::tuple::Tuple {
    crate::tuple::Tuple::vector(x.to_f32(), y.to_f32(), z.to_f32())
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, v: Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, v: Vector) -> Vector {
        Vector::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, f: f32) -> Vector {
        Vector::new(self.x * f, self.y * f, self.z * f)
    }
}

impl std::ops::Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, v: Vector) -> Vector {
        v * self
    }
}

impl std::ops::Div<f32> for Vector {
    type Output = Vector;

    fn div(self, f: f32) -> Vector {
        self * (1.0 / f)
    }
}

pub fn dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn cross(a: Vector, b: Vector) -> Vector {
    Vector::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

impl crate::approx::Approx for Vector {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        self.x.is_near(&other.x, eps)
            && self.y.is_near(&other.y, eps)
            && self.z.is_near(&other.z, eps)
    }
}

#[cfg(test)]
mod tests {

    use super::{cross, dot, Vector};
    use crate::test_utils::*;
    use std::f32::consts::FRAC_1_SQRT_2;

    // TODO: Delete.
    pub fn vector<T: crate::F32Const>(x: T, y: T, z: T) -> Vector {
        crate::tuple::Tuple::vector(x.to_f32(), y.to_f32(), z.to_f32())
            .try_into()
            .unwrap()
    }

    #[test]
    fn test_create_vector() {
        let v = vector(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(v, Vector::new(4.3, -4.2, 3.1));
    }

    #[test]
    fn test_adding_two_vectors() {
        let v1 = vector(3, -2, 5);
        let v2 = vector(-2, 3, 1);
        assert_eq!(v1 + v2, vector(1, 1, 6));
    }

    #[test]
    fn test_subtracting_two_vectors() {
        let v1 = vector(3, 2, 1);
        let v2 = vector(5, 6, 7);
        assert_eq!(v1 - v2, vector(-2, -4, -6));
    }

    #[test]
    fn test_subtracting_vector_from_zero_vector() {
        let zero = vector(0, 0, 0);
        let v = vector(1, 2, 3);
        assert_eq!(zero - v, vector(-1, -2, -3));
    }

    #[test]
    fn test_negating_vector() {
        let v = vector(1, -2, 3);
        assert_eq!(-v, vector(-1, 2, -3));
    }

    #[test]
    fn test_multiplying_vector_by_scalar() {
        let v = vector(1, -2, 3);
        assert_eq!(v * 3.5, vector(3.5, -7., 10.5));
        assert_eq!(3.5 * v, vector(3.5, -7., 10.5));
    }

    #[test]
    fn test_multiplying_vector_by_fraction() {
        let v = vector(1, -2, 3);
        assert_eq!(v * 0.5, vector(0.5, -1.0, 1.5));
    }

    #[test]
    fn test_dividing_vector_by_scalar() {
        let v = vector(1, -2, 3);
        assert_eq!(v / 2., vector(0.5, -1.0, 1.5));
    }

    #[test]
    fn test_length_of_vector() {
        assert_eq!(vector(1, 0, 0).len(), 1.0);
        assert_eq!(vector(0, 1, 0).len(), 1.0);
        assert_eq!(vector(0, 0, 1).len(), 1.0);
        assert_eq!(vector(1, 2, 3).len(), 14_f32.sqrt());
        assert_eq!(vector(-1, -2, -3).len(), 14_f32.sqrt());
    }

    #[test]
    fn test_normalize_vector() {
        assert_eq!(vector(4, 0, 0).normalized(), vector(1, 0, 0));
        assert_near!(
            vector(1, 2, 3).normalized(),
            vector(0.26726, 0.53452, 0.80178)
        );
        assert_near!(vector(1, 2, 3).normalized().len(), 1.0);
    }

    #[test]
    fn test_dot_product_of_two_vectors() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(dot(a, b), 20.0);
    }

    #[test]
    fn test_cross_product_of_two_vector() {
        let a = vector(1, 2, 3);
        let b = vector(2, 3, 4);
        assert_eq!(cross(a, b), vector(-1, 2, -1));
        assert_eq!(cross(b, a), vector(1, -2, 1));
    }

    #[test]
    fn test_reflect_vector_at_45_angle() {
        let v = vector(1, -1, 0);
        let n = vector(0, 1, 0);
        assert_eq!(v.reflected(n), vector(1, 1, 0));
    }

    #[test]
    fn test_reflect_vector_off_slanted_surface() {
        let v = vector(0, -1, 0);
        let n = vector(FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.0);
        assert_near!(v.reflected(n), vector(1, 0, 0));
    }
}
