mod math;
mod pixel;
mod png;

use glam::Vec3;
use math::camera::{Camera};
use math::object::{HitRecord, Hittable, World};
use math::ray::Ray;
use math::sphere::Sphere;
use pixel::{Persistable, Pixel};
use rand::prelude::*;

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

    let camera = Camera::new(
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    );

    let mut image = vec![Pixel::new(0, 0, 0); width * height];

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere_big = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = World::new(vec![&sphere, &sphere_big]);

    const ANTIALIASING_FACTOR: u32 = 100;
    let mut rng = thread_rng();

    for j in (0..height).rev() {
        for i in 0..width {
            let mut cumulative_color: Vec3 = Vec3::ZERO;
            for _ in 0..ANTIALIASING_FACTOR {
                let i: f32 = i as f32 + rng.gen::<f32>() - 0.5;
                let j: f32 = j as f32 + rng.gen::<f32>() - 0.5;
                let u = i / width as f32;
                let v = j / height as f32;
                let r = camera.cast_ray(u, v);
                let mut color = color(&r);

                if let Some(rec) = world.hit(&r, 0.0, std::f32::MAX) {
                    color = 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
                }
                cumulative_color += color;
            }

            cumulative_color /= ANTIALIASING_FACTOR as f32;

            let ir = (255.99 * cumulative_color.x) as i32;
            let ig = (255.99 * cumulative_color.y) as i32;
            let ib = (255.99 * cumulative_color.z) as i32;

            let index = (height - 1 - j as usize) * width + i as usize;
            image[index].set(ir as u8, ig as u8, ib as u8);
        }
    }

    image
        .save(
            "image.png",
            width.try_into().unwrap(),
            height.try_into().unwrap(),
        )
        .unwrap();
}
