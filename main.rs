mod vec3;
mod sphere;
mod raytracer;
mod bmp_writer;

use vec3::Vec3;
use sphere::Sphere;
use raytracer::trace;
use bmp_writer::write_bmp;

fn main() -> std::io::Result<()> {
    let width = 800;
    let height = 600;
    let mut buffer = vec![0u8; width * height * 3];

    let camera = Vec3::new(0.0, 3.0, -6.0);
    let spheres = vec![
        Sphere::new(Vec3::new(-2.2, 1.0, 1.0), 1.0, Vec3::new(1.0, 0.3, 0.8)),  // Pink
        Sphere::new(Vec3::new(0.0, 1.5, 0.0), 1.5, Vec3::new(0.3, 1.0, 0.5)),   // Green
        Sphere::new(Vec3::new(2.2, 1.0, 1.0), 1.0, Vec3::new(0.8, 0.8, 0.2)),   // Yellow
    ];

    for y in 0..height {
        for x in 0..width {
            let u = (x as f64) / (width as f64);
            let v = (y as f64) / (height as f64);
            let dir = Vec3::new(u - 0.5, 0.5 - v, 1.0).normalize();
            let color = trace(&camera, &dir, &spheres, 0);
            let idx = ((height - 1 - y) * width + x) * 3;
            buffer[idx] = (color.z * 255.0) as u8;
            buffer[idx + 1] = (color.y * 255.0) as u8;
            buffer[idx + 2] = (color.x * 255.0) as u8;
        }
    }

    write_bmp("output.bmp", &buffer, width as u32, height as u32)
}
