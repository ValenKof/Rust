use crate::ray::Ray;
use crate::world::WorldObject;

pub trait Intersect {
    type Output;

    fn intersect(self, r: &Ray) -> Self::Output;
}

#[derive(Debug, PartialEq)]
pub struct Intersection<'a, T: WorldObject> {
    pub object: &'a T,
    pub t: f32,
}

impl<'a, T: WorldObject> Copy for Intersection<'a, T> {}

impl<'a, T: WorldObject> Clone for Intersection<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, T: WorldObject> Intersection<'a, T> {
    pub fn new(object: &'a T, t: f32) -> Intersection<'a, T> {
        Intersection { object, t }
    }
}

pub fn hit<'a, T: WorldObject + Clone>(xs: &[Intersection<'a, T>]) -> Option<Intersection<'a, T>> {
    xs.iter()
        .map(|x| *x)
        .filter(|x| x.t > 0.0)
        .reduce(|x, y| if x.t < y.t { x } else { y })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

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
        let xs: Vec<Intersection<Sphere>> = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn test_hit_all_positive() {
        let s = Sphere::new();
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let xs: Vec<Intersection<Sphere>> = vec![i2, i1];
        assert_eq!(hit(&xs), Some(i1));
    }

    #[test]
    fn test_hit_mixed() {
        let s = Sphere::new();
        let i1 = Intersection::new(&s, -1.0);
        let i2 = Intersection::new(&s, 1.0);
        let xs: Vec<Intersection<Sphere>> = vec![i2, i1];
        assert_eq!(hit(&xs), Some(i2));
    }

    #[test]
    fn test_hit_all_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(&s, -2.0);
        let i2 = Intersection::new(&s, -1.0);
        let xs: Vec<Intersection<Sphere>> = vec![i2, i1];
        assert_eq!(hit(&xs), None);
    }

    #[test]
    fn test_hit_returns_lowest_non_negative() {
        let s = Sphere::new();
        let i1 = Intersection::new(&s, 5.0);
        let i2 = Intersection::new(&s, 7.0);
        let i3 = Intersection::new(&s, -3.0);
        let i4 = Intersection::new(&s, 2.0);
        let xs: Vec<Intersection<Sphere>> = vec![i1, i2, i3, i4];
        assert_eq!(hit(&xs), Some(i4));
    }
}
