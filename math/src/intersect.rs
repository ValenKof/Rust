use crate::world::WorldObject;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<'a> {
    pub object: &'a WorldObject,
    pub t: f32,
}

impl<'a> Intersection<'a> {
    pub fn new(object: &'a WorldObject, t: f32) -> Self {
        Self { object, t }
    }
}

pub fn hit<'a>(xs: &[Intersection<'a>]) -> Option<Intersection<'a>> {
    xs.iter()
        .map(|x| *x)
        .filter(|x| x.t > 0.0)
        .reduce(|x, y| if x.t < y.t { x } else { y })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;
    use crate::world::WorldObject;

    #[test]
    fn test_create_intersection() {
        let s = WorldObject::Sphere(Sphere::new());
        let i = Intersection::new(&s, 3.5);
        assert_eq!(i.object, &s);
        assert_eq!(i.t, 3.5);
    }

    #[test]
    fn test_create_intersections() {
        let s = WorldObject::Sphere(Sphere::new());
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let xs: Vec<Intersection> = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn test_hit_all_positive() {
        let s = WorldObject::Sphere(Sphere::new());
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let xs: Vec<Intersection> = vec![i2, i1];
        assert_eq!(hit(&xs), Some(i1));
    }

    #[test]
    fn test_hit_mixed() {
        let s = WorldObject::Sphere(Sphere::new());
        let i1 = Intersection::new(&s, -1.0);
        let i2 = Intersection::new(&s, 1.0);
        let xs: Vec<Intersection> = vec![i2, i1];
        assert_eq!(hit(&xs), Some(i2));
    }

    #[test]
    fn test_hit_all_negative() {
        let s = WorldObject::Sphere(Sphere::new());
        let i1 = Intersection::new(&s, -2.0);
        let i2 = Intersection::new(&s, -1.0);
        let xs: Vec<Intersection> = vec![i2, i1];
        assert_eq!(hit(&xs), None);
    }

    #[test]
    fn test_hit_returns_lowest_non_negative() {
        let s = WorldObject::Sphere(Sphere::new());
        let i1 = Intersection::new(&s, 5.0);
        let i2 = Intersection::new(&s, 7.0);
        let i3 = Intersection::new(&s, -3.0);
        let i4 = Intersection::new(&s, 2.0);
        let xs: Vec<Intersection> = vec![i1, i2, i3, i4];
        assert_eq!(hit(&xs), Some(i4));
    }
}
