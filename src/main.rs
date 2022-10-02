mod math;
mod pixel;
mod png;

use glam::Vec3;
use math::camera::Camera;
use math::object::{World};
use math::sphere::Sphere;
use pixel::{Persistable, Pixel};
use rand::prelude::*;

use crate::math::object::{Lambertian, Dielectric, Metal};

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

    let lambertian_sphere = Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let lambertian_sphere_big = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let metal_sphere = Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.0);
    let dielectric_sphere = Dielectric::new(1.5);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &lambertian_sphere);
    let sphere_big = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, &lambertian_sphere_big);
    let sphere_metal = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &metal_sphere);
    let sphere_metal_2 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.45, &dielectric_sphere);

    let world = World::new(vec![&sphere, &sphere_big, &sphere_metal, &sphere_metal, &sphere_metal_2]);

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

                let color = world.color_at(&r, 0);
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
