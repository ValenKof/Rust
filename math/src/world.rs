use crate::intersect::Intersect;
use crate::material::Material;
use crate::ray::Ray;
use crate::tuple::Tuple;

pub trait WorldObject {
    fn normal_at(&self, world_point: Tuple) -> Tuple;
    fn material_at(&self, world_point: Tuple) -> Material;
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
}

impl<'a> Intersect for &'a World {
    type Output = Option<&'a dyn WorldObject>;

    fn intersect(self, _r: &Ray) -> Self::Output {
        for obj in self.objects.iter() {}
        None
    }
}
