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

    pub fn refract(&self, normal: &Vec3, ni_over_nt: f32) -> Option<Ray> {
        let uv = self.direction.normalize();
        let n = normal.normalize();
        let dt = uv.dot(n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some(Ray::new(
                self.origin,
                ni_over_nt * (uv - n * dt) - n * discriminant.sqrt(),
            ))
        } else {
            None
        }
    }
}
