use crate::intersect::{Intersect, Intersection};
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::Tuple;

#[derive(PartialEq)]
pub enum WorldObjectRef<'a> {
    Sphere(&'a Sphere),
}

pub trait WorldObject: std::fmt::Debug {
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn material_at(&self, world_point: Tuple) -> Material;
    fn as_ref(&self) -> WorldObjectRef;
}

pub struct World {
    objects: Vec<Box<dyn WorldObject>>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: Vec::new(),
        }
    }

    pub fn intersect(&self, r: &Ray) {}
}
