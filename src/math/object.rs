use crate::math::ray::Ray;
use glam::Vec3;
pub trait Hittable<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p: Vec3, normal: Vec3, material: &'a dyn Material) -> Self {
        Self {
            t,
            p,
            normal,
            material,
        }
    }
}

pub struct World<'a> {
    objects: Vec<&'a dyn Hittable<'a>>,
}

impl<'a> World<'a> {
    pub fn new(objects: Vec<&'a dyn Hittable<'a>>) -> Self {
        Self { objects }
    }

    pub fn color_at(&self, ray: &Ray, depth: i32) -> Vec3 {
        let mut closest_so_far = std::f32::MAX;
        let mut hit_record = None;
        for object in &self.objects {
            if let Some(rec) = object.hit(ray, 0.00001, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }
        if let Some(rec) = hit_record {
            if depth < 50 {
                if let Some(rec) = rec.material.scatter(ray, &rec) {
                    return rec.attenuation * self.color_at(&rec.scattered, depth + 1);
                }
            }
            return Vec3::ZERO;
        }

        // background color
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        let attenuation = self.albedo;
        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray.reflect(&rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected.direction + self.fuzz * random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        if scattered.direction.dot(rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;
        let reflected = ray.reflect(&rec.normal);

        if ray.direction.dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.index_of_refraction;
            cosine =
                self.index_of_refraction * ray.direction.dot(rec.normal) / ray.direction.length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.index_of_refraction;
            cosine = -ray.direction.dot(rec.normal) / ray.direction.length();
        }

        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refracted = ray.refract(&outward_normal, ni_over_nt);
        if let Some(refracted) = refracted {
            let reflect_prob = schlick(cosine, self.index_of_refraction);
            if rand::random::<f32>() < reflect_prob {
                return Some(ScatterRecord {
                    attenuation,
                    scattered: reflected,
                });
            }
            return Some(ScatterRecord {
                attenuation,
                scattered: refracted,
            });
        }
        Some(ScatterRecord {
            attenuation,
            scattered: reflected,
        })
    }
}

// util methods
fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::ZERO;
    loop {
        p = 2.0 * Vec3::new(rand::random(), rand::random(), rand::random())
            - Vec3::new(1.0, 1.0, 1.0);
        if p.length_squared() < 1.0 {
            break;
        }
    }
    p
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
