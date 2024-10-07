mod vec3;
mod sphere;
mod raytracer;
mod bmp_writer;

use vec3::Vec3;
use sphere::Sphere;
use raytracer::trace;
use bmp_writer::write_bmp;
use rand::thread_rng;
use rand::Rng;
use std::env;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let max_bounces = if args.len() > 1 {
        args[1].parse().unwrap_or(10)
    } else {
        10
    };

    let width = 800;
    let height = 600;
    let samples = 16; // Increased for better quality
    let mut buffer = vec![0u8; width * height * 3];

    let camera = Vec3::new(0.0, 5.0, -15.0);
    let spheres = vec![
        Sphere::new(Vec3::new(-4.0, 1.0, 2.0), 1.0, Vec3::new(1.0, 0.2, 0.2)),   // Bright Red
        Sphere::new(Vec3::new(0.0, 3.5, 0.0), 2.5, Vec3::new(0.2, 1.0, 0.2)),    // Bright Green
        Sphere::new(Vec3::new(4.0, 1.5, -2.0), 1.5, Vec3::new(0.2, 0.2, 1.0)),   // Bright Blue
        Sphere::new(Vec3::new(-2.0, 0.5, -3.0), 0.5, Vec3::new(1.0, 1.0, 0.2)),  // Bright Yellow
        Sphere::new(Vec3::new(2.0, 0.7, 3.0), 0.7, Vec3::new(1.0, 0.2, 1.0)),    // Bright Magenta
    ];

    let mut rng = thread_rng();

    for y in 0..height {
        for x in 0..width {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (x as f64 + rng.gen::<f64>()) / (width as f64);
                let v = (y as f64 + rng.gen::<f64>()) / (height as f64);
                let dir = Vec3::new(u - 0.5, 0.5 - v, 1.0).normalize();
                color = color + trace(&camera, &dir, &spheres, 0, max_bounces, &mut rng);
            }
            color = color * (1.0 / samples as f64);
            let idx = ((height - 1 - y) * width + x) * 3;
            buffer[idx] = (color.z.sqrt() * 255.0) as u8;
            buffer[idx + 1] = (color.y.sqrt() * 255.0) as u8;
            buffer[idx + 2] = (color.x.sqrt() * 255.0) as u8;
        }
    }

    write_bmp("output.bmp", &buffer, width as u32, height as u32)
}
