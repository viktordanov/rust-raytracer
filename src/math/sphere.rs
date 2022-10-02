use crate::math::object::{HitRecord, Hittable};
use crate::math::ray::Ray;
use glam::Vec3;

pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
}

impl Sphere {
	pub fn new(center: Vec3, radius: f32) -> Self {
		Self { center, radius }
	}
}

impl Hittable for Sphere {
	fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
		let oc = ray.origin - self.center;
		let a = ray.direction.dot(ray.direction);
		let b = oc.dot(ray.direction);
		let c = oc.dot(oc) - self.radius * self.radius;
		let discriminant = b * b - a * c;
		if discriminant > 0.0 {
			let temp = (-b - discriminant.sqrt()) / a;
			if temp < t_max && temp > t_min {
				let p = ray.point_at_parameter(temp);
				let normal = (p - self.center) / self.radius;
				return Some(HitRecord::new(temp, p, normal));
			}
			let temp = (-b + discriminant.sqrt()) / a;
			if temp < t_max && temp > t_min {
				let p = ray.point_at_parameter(temp);
				let normal = (p - self.center) / self.radius;
				return Some(HitRecord::new(temp, p, normal));
			}
		}
		None
	}
}