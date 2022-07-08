use crate::color::Color;
use crate::intersect::Intersection;
use crate::light::PointLight;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transforms;
use crate::tuple::Tuple;

pub trait Intersect {
    fn intersect<'a>(&'a self, r: &Ray) -> Vec<Intersection<'a>>;
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn material_at(&self, world_point: Tuple) -> Material;
}

#[derive(Debug, PartialEq)]
pub enum WorldObject {
    Sphere(Sphere),
}

impl Intersect for WorldObject {
    fn intersect<'a>(&'a self, r: &Ray) -> Vec<Intersection<'a>> {
        let ts = match self {
            WorldObject::Sphere(s) => s.intersect(r),
        };
        let mut intersections = Vec::with_capacity(ts.len());
        for t in ts {
            intersections.push(Intersection::new(self, t));
        }
        intersections
    }

    fn normal_at(&self, world_point: Tuple) -> Tuple {
        match self {
            WorldObject::Sphere(s) => s.normal_at(world_point),
        }
    }

    fn material_at(&self, world_point: Tuple) -> Material {
        match self {
            WorldObject::Sphere(s) => s.material_at(world_point),
        }
    }
}

pub struct World {
    pub objects: Vec<WorldObject>,
    pub light: Option<PointLight>,
}

impl World {
    pub fn new() -> World {
        let objects = vec![];
        let light = None;
        World { objects, light }
    }

    pub fn intersect<'a>(&'a self, r: &Ray) -> Vec<Intersection<'a>> {
        let mut xs = vec![];
        for object in self.objects.iter() {
            xs.extend(object.intersect(r).into_iter());
        }
        xs.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
        xs
    }
}

pub fn default_world() -> World {
    let mut w = World::new();
    w.light = Some(PointLight::new(
        Tuple::point(-10., -10., -10.),
        Color::new(1., 1., 1.),
    ));
    w.objects.push(WorldObject::Sphere({
        let mut s = Sphere::new();
        s.set_material({
            let mut m = Material::new();
            m.color = Color::new(0.8, 1.0, 0.6);
            m.diffuse = 0.7;
            m.specular = 0.2;
            m
        });
        s
    }));
    w.objects.push(WorldObject::Sphere({
        let mut s = Sphere::new();
        s.set_transform(transforms::scaling(0.5, 0.5, 0.5));
        s
    }));
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::light::PointLight;
    use crate::material::Material;
    use crate::sphere::Sphere;
    use crate::test_utils::*;
    use crate::transforms;
    use crate::tuple::{Point, Tuple, Vector};

    #[test]
    fn test_create_world() {
        let w = World::new();
        assert_eq!(w.objects, vec![]);
        assert_eq!(w.light, None);
    }

    #[test]
    fn test_create_default_world() {
        let w = default_world();
        let s1 = WorldObject::Sphere({
            let mut s = Sphere::new();
            s.set_material({
                let mut m = Material::new();
                m.color = Color::new(0.8, 1.0, 0.6);
                m.diffuse = 0.7;
                m.specular = 0.2;
                m
            });
            s
        });
        let s2 = WorldObject::Sphere({
            let mut s = Sphere::new();
            s.set_transform(transforms::scaling(0.5, 0.5, 0.5));
            s
        });
        let light = PointLight::new(point(-10, -10, -10), Color::new(1., 1., 1.));
        assert_eq!(w.light, Some(light));
        assert_eq!(w.objects, vec![s1, s2]);
    }

    #[test]
    fn test_intersect_default_world() {
        let w = default_world();
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let xs = w.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}
