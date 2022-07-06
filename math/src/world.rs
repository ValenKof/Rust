use crate::intersect::Intersection;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum WorldObjectRef<'a> {
    Sphere(&'a Sphere),
}

impl<'a> WorldObjectRef<'a> {
    pub fn intersect(self, r: &Ray) -> Vec<Intersection<'a>> {
        let ts = match self {
            WorldObjectRef::Sphere(s) => s.intersect(r),
        };
        let mut intersections = Vec::with_capacity(ts.len());
        for t in ts {
            intersections.push(Intersection::new(self, t));
        }
        intersections
    }

    pub fn normal_at(self, world_point: Tuple) -> Tuple {
        match self {
            WorldObjectRef::Sphere(s) => s.normal_at(world_point),
        }
    }

    pub fn material_at(self, world_point: Tuple) -> Material {
        match self {
            WorldObjectRef::Sphere(s) => s.material_at(world_point),
        }
    }
}

//pub trait WorldObject: std::fmt::Debug {
//    fn as_ref(&self) -> WorldObjectRef;
//}

pub struct World {
    objects: Vec<f32>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn intersect(&self, r: &Ray) {}
}
