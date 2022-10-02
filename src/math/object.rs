use glam::Vec3;
use crate::math::ray::Ray;
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