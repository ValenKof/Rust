use crate::color::Color;
use crate::intersect::{hit, Intersection};
use crate::light::PointLight;
use crate::lighting;
use crate::material::Material;
use crate::point::{point, Point};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transforms;
use crate::vector::{dot, Vector};

pub trait Intersect {
    fn intersect<'a>(&'a self, r: &Ray) -> Vec<Intersection<'a>>;
    fn normal_at(&self, world_point: Point) -> Vector;
    fn material_at(&self, world_point: Point) -> Material;
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

    fn normal_at(&self, world_point: Point) -> Vector {
        match self {
            WorldObject::Sphere(s) => s.normal_at(world_point),
        }
    }

    fn material_at(&self, _world_point: Point) -> Material {
        match self {
            WorldObject::Sphere(s) => s.material(),
        }
    }
}

pub struct Computations<'a> {
    pub object: &'a WorldObject,
    pub point: Point,
    pub eye_vec: Vector,
    pub normal_vec: Vector,
    pub is_inside: bool,
}

impl<'a> Computations<'a> {
    pub fn new(i: Intersection<'a>, r: &Ray) -> Self {
        let point = r.position(i.t);
        let mut normal_vec = i.object.normal_at(point);
        let eye_vec = -r.direction;
        let is_inside = dot(normal_vec, eye_vec) < 0.0;
        if is_inside {
            normal_vec = -normal_vec;
        }
        Self {
            object: i.object,
            point,
            eye_vec,
            normal_vec,
            is_inside,
        }
    }
}

pub struct World {
    pub objects: Vec<WorldObject>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn intersect<'a>(&'a self, r: &Ray) -> Vec<Intersection<'a>> {
        let mut xs = vec![];
        for object in self.objects.iter() {
            xs.extend(object.intersect(r).into_iter());
        }
        xs.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
        xs
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        let mut c = Color::black();
        for light in &self.lights {
            c = c + lighting::phong(
                comps.object.material_at(comps.point),
                light,
                comps.point,
                comps.eye_vec,
                comps.normal_vec,
            );
        }
        c
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        let xs = self.intersect(r);
        if let Some(x) = hit(&xs) {
            self.shade_hit(Computations::new(x, r))
        } else {
            Color::black()
        }
    }
}

pub fn default_world() -> World {
    let mut w = World::new();
    w.lights = vec![PointLight::new(
        point(-10., -10., -10.),
        Color::new(1., 1., 1.),
    )];
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
    use crate::point::{point, Point};
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::test_utils::*;
    use crate::transforms;
    use crate::vector::{vector, Vector};

    #[test]
    fn test_create_world() {
        let w = World::new();
        assert_eq!(w.objects, vec![]);
        assert_eq!(w.lights, vec![]);
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
        assert_eq!(w.lights, vec![light]);
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

    #[test]
    fn test_intersection_outside() {
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = WorldObject::Sphere(Sphere::new());
        let i = Intersection::new(&s, 4.);
        let comps = Computations::new(i, &r);
        assert_eq!(comps.object, &s);
        assert_eq!(comps.point, point(0, 0, -1));
        assert_eq!(comps.eye_vec, vector(0, 0, -1));
        assert_eq!(comps.normal_vec, vector(0, 0, -1));
        assert_eq!(comps.is_inside, false);
    }

    #[test]
    fn test_intersection_inside() {
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = WorldObject::Sphere(Sphere::new());
        let i = Intersection::new(&s, 1.);
        let comps = Computations::new(i, &r);
        assert_eq!(comps.object, &s);
        assert_eq!(comps.point, point(0, 0, 1));
        assert_eq!(comps.eye_vec, vector(0, 0, -1));
        assert_eq!(comps.normal_vec, vector(0, 0, -1));
        assert_eq!(comps.is_inside, true);
    }

    #[test]
    fn test_shading_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        let s = &w.objects[0];
        let i = Intersection::new(s, 4.);
        let comps = Computations::new(i, &r);
        assert_near!(w.shade_hit(comps), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shading_intersection_from_inside() {
        let mut w = default_world();
        w.lights = vec![PointLight::new(point(0., 0.25, 0.), Color::new(1., 1., 1.))];
        let r = Ray::new(Point::new(0., 0., 0.), Vector::new(0., 0., 1.));
        let s = &w.objects[1];
        let i = Intersection::new(s, 0.5);
        let comps = Computations::new(i, &r);
        assert_near!(w.shade_hit(comps), Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_color_when_ray_misses() {
        let w = default_world();
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 1., 0.));
        assert_eq!(w.color_at(&r), Color::new(0., 0., 0.));
    }

    #[test]
    fn test_color_when_ray_hits() {
        let w = default_world();
        let r = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));
        assert_near!(w.color_at(&r), Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_color_with_intersection_behind_ray() {
        let mut w = default_world();

        for object in &mut w.objects {
            if let WorldObject::Sphere(s) = object {
                let mut m = s.material();
                m.ambient = 1.0;
                s.set_material(m);
            }
        }

        let r = Ray::new(Point::new(0., 0., 0.75), Vector::new(0., 0., -1.));
        assert_near!(
            w.color_at(&r),
            w.objects[1].material_at(point(0, 0, 0)).color
        );
    }
}
