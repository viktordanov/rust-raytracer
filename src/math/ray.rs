use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    pub fn reflect(&self, normal: &Vec3) -> Ray {
        Ray::new(
            self.origin,
            self.direction - *normal * 2.0 * self.direction.dot(*normal),
        )
    }
}
