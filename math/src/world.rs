use crate::intersect::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
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
    objects: Vec<WorldObject>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn intersect<'a>(&'a self, r: &Ray) -> Vec<Intersection<'a>> {
        vec![]
    }
}
