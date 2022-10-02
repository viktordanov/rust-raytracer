use crate::math::ray::Ray;
use glam::Vec3;
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f32, p: Vec3, normal: Vec3) -> Self {
        Self { t, p, normal }
    }
}

pub struct World<'a> {
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> World<'a> {
    pub fn new(objects: Vec<&'a dyn Hittable>) -> Self {
        Self { objects }
    }
}

impl<'a> Hittable for World<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(result) = object.hit(ray, t_min, t_max) {
                if closest > result.t {
                    closest = result.t;
                    rec = Some(result);
                }
            }
        }

        rec
    }
}
