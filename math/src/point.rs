use crate::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }
}

pub fn point<T: crate::F32Const>(x: T, y: T, z: T) -> Point {
    Point::new(x.to_f32(), y.to_f32(), z.to_f32())
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, v: Vector) -> Point {
        Point::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, v: Vector) -> Point {
        Point::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, p: Point) -> Vector {
        Vector::new(self.x - p.x, self.y - p.y, self.z - p.z)
    }
}

impl crate::approx::Approx for Point {
    fn is_near(&self, other: &Self, eps: f32) -> bool {
        self.x.is_near(&other.x, eps)
            && self.y.is_near(&other.y, eps)
            && self.z.is_near(&other.z, eps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::test_utils::*;
    use crate::vector::vector;

    #[test]
    fn test_create_point() {
        let p = point(4.3, -4.2, 3.1);
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
        assert_eq!(p, Point::new(4.3, -4.2, 3.1));
    }

    #[test]
    fn test_subtracting_two_points() {
        let p1 = point(3, 2, 1);
        let p2 = point(5, 6, 7);
        assert_eq!(p1 - p2, vector(-2, -4, -6));
    }

    #[test]
    fn test_subtracting_vector_from_point() {
        let p = point(3, 2, 1);
        let v = vector(5, 6, 7);
        assert_eq!(p - v, point(-2, -4, -6));
    }
}
