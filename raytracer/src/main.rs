use rayon::prelude::*;
use raytracer::material::Material;
use raytracer::objects::object::Object;
use raytracer::objects::plane::Plane;
use raytracer::objects::sphere::Sphere;
use raytracer::raytracing::physics::cast_ray;
use raytracer::vec3::Vec3;
use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let greenish = Material::new(1.0, [0.9, 0.5, 0.1, 0.0], Vec3::new(0.1, 0.4, 0.2), 120.0);
    let glass = Material::new(1.5, [0.0, 0.9, 0.1, 0.8], Vec3::new(0.6, 0.7, 0.8), 125.0);
    let red_rubber = Material::new(1.0, [1.4, 0.3, 0.0, 0.0], Vec3::new(0.3, 0.1, 0.1), 10.0);
    let mirror = Material::new(1.0, [0.0, 16.0, 0.8, 0.0], Vec3::new(1.0, 1.0, 1.0), 1425.0);

    let objects: Vec<Arc<dyn Object + Sync + Send>> = vec![
        Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, -16.0), 2.0, greenish)),
        Arc::new(Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, glass)),
        Arc::new(Sphere::new(Vec3::new(4.0, -0.5, -18.0), 3.0, red_rubber)),
        Arc::new(Sphere::new(Vec3::new(0.0, 12.0, -38.0), 10.0, mirror)),
        Arc::new(Plane::new(
            Vec3::new(0.0, -5.0, -15.0),
            Vec3::new(0.0, 1.0, 0.0),
            10.0,
        )),
    ];

    let lights: Vec<Vec3> = vec![
        Vec3::new(-20.0, 20.0, 20.0),
        Vec3::new(30.0, 50.0, -25.0),
        Vec3::new(30.0, 20.0, 30.0),
    ];

    let scale_factor = 1;

    let width = 3840 * scale_factor;
    let height = 2160 * scale_factor;
    let fov = (60.0 / 180.0) * PI;

    let mut buffer: Vec<u8> = vec![0; width * height * 3];

    let start = Instant::now();

    buffer
        .par_iter_mut()
        .chunks(3)
        .enumerate()
        .for_each(|(index, mut val)| {
            let dir_x = ((index % width) as f32 + 0.5) - width as f32 / 2.0;
            let dir_y = -((index / width) as f32 + 0.5) + height as f32 / 2.0;
            let dir_z = -(height as f32) / (2.0 * (fov / 2.0).tan());

            let vec = cast_ray(
                Vec3::default(),
                Vec3::new(dir_x, dir_y, dir_z).norm(),
                &lights,
                &objects,
                0,
            );

            *val[0] = (vec.x() * 255.0) as u8;
            *val[1] = (vec.y() * 255.0) as u8;
            *val[2] = (vec.z() * 255.0) as u8;
        });

    let duration = start.elapsed();
    println!("Time elapsed in raytracing: {:?}", duration);

    // let start = Instant::now();

    image::save_buffer(
        "image.png",
        buffer.as_slice(),
        width as u32,
        height as u32,
        image::ColorType::Rgb8,
    )
    .unwrap();

    // let duration = start.elapsed();
    // println!("Time elapsed in bufer: {:?}", duration);
}
