use crate::ray::Ray;

pub trait Intersect {
    type Output;

    fn intersect(self, r: &Ray) -> Self::Output;
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<T: Intersect> {
    pub object: T,
    pub t: f32,
}

impl<T: Intersect> Intersection<T> {
    pub fn new(object: T, t: f32) -> Intersection<T> {
        Intersection { object, t }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    //use crate::sphere::Sphere;

    #[test]
    fn test_create_intersection() {
        let s = Sphere::new();
        let i = Intersection::new(&s, 3.5);
        assert_eq!(i.object as *const Sphere, &s as *const Sphere);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn test_create_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let xs: Vec<Intersection<&Sphere>> = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }
}
