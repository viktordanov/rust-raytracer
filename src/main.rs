mod math;
mod pixel;
mod png;

use glam::Vec3;
use math::ray::Ray;
use math::sphere::Sphere;
use math::object::{Hittable, HitRecord};
use pixel::{Pixel, Persistable};


fn color(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    let blue = Vec3::new(0.5, 0.7, 1.0);
    white * (1.0 - t) + blue * t
}

fn main() {
    let width = 600;
    let height = 300;
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let mut image = vec![Pixel::new(0, 0, 0); width * height];

    let binding = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let objects: Vec<&dyn Hittable> = vec![&binding];

    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f32 / width as f32;
            let v = j as f32 / height as f32;
            let r = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);

            let mut color = color(&r);
            for obj in objects.iter() {
                if let Some(rec) = obj.hit(&r, 0.0, std::f32::MAX) {
                    color = 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
                }
            }

            let ir = (255.99 * color.x) as i32;
            let ig = (255.99 * color.y) as i32;
            let ib = (255.99 * color.z) as i32;
        
            let index = (j * width + i) as usize;
            image[index].set(ir as u8, ig as u8, ib as u8);
        }
    }

    image.save("image.png", width.try_into().unwrap(), height.try_into().unwrap()).unwrap();
}
