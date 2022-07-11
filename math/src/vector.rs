use crate::tuple::Tuple;

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
}

impl std::convert::From<Vector> for Tuple {
    fn from(v: Vector) -> Tuple {
        Tuple::vector(v.x, v.y, v.z)
    }
}

impl std::convert::TryFrom<Tuple> for Vector {
    type Error = &'static str;

    fn try_from(t: Tuple) -> Result<Vector, Self::Error> {
        if t.is_vector() {
            Ok(Vector::new(t.x, t.y, t.z))
        } else {
            Err("Tuple has w != 0")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn test_vector_struct() {
        let v = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(v.x, 4.3);
        assert_eq!(v.y, -4.2);
        assert_eq!(v.z, 3.1);
        assert_eq!(Tuple::from(v), Tuple::new(4.3, -4.2, 3.1, 0.0));
    }

    #[test]
    fn test_vector_from_tuple() {
        let t0 = tuple(4, 3, 2, 0);
        let t1 = tuple(4, 3, 2, 1);
        assert_eq!(Vector::try_from(t0), Ok(Vector::new(4.0, 3.0, 2.0)));
        assert_eq!(Vector::try_from(t1), Err("Tuple has w != 0"));
    }
}
