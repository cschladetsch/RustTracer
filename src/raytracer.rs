use crate::vec3::Vec3;
use crate::sphere::Sphere;
use rand::Rng;

pub fn trace(origin: &Vec3, dir: &Vec3, spheres: &[Sphere], depth: u32, max_depth: u32, rng: &mut impl Rng) -> Vec3 {
    if depth >= max_depth {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    let mut closest_t = std::f64::MAX;
    let mut closest_sphere = None;

    for sphere in spheres {
        if let Some(t) = sphere.intersect(origin, dir) {
            if t < closest_t {
                closest_t = t;
                closest_sphere = Some(sphere);
            }
        }
    }

    match closest_sphere {
        Some(sphere) => {
            let hit_point = *origin + *dir * closest_t;
            let normal = (hit_point - sphere.center).normalize();
            let reflect_dir = *dir - normal * (2.0 * dir.dot(&normal));
            let reflect_origin = hit_point + normal * 0.001;
            let reflect_color = trace(&reflect_origin, &reflect_dir, spheres, depth + 1, max_depth, rng);

            // Soft shadows, ambient occlusion, and specular highlights
            let light_pos = Vec3::new(10.0, 10.0, -10.0);
            let shadow_samples = 32;
            let mut shadow_intensity = 0.0;
            let mut ao_intensity = 0.0;

            for _ in 0..shadow_samples {
                let light_offset = Vec3::new(
                    rng.gen_range(-2.0..2.0),
                    rng.gen_range(-2.0..2.0),
                    rng.gen_range(-2.0..2.0)
                );
                let light_dir = (light_pos + light_offset - hit_point).normalize();
                let shadow_origin = hit_point + normal * 0.001;
                
                if !is_occluded(&shadow_origin, &light_dir, spheres) {
                    shadow_intensity += 1.0;
                }

                let ao_dir = random_unit_vector(rng);
                if !is_occluded(&shadow_origin, &ao_dir, spheres) {
                    ao_intensity += 1.0;
                }
            }
            shadow_intensity /= shadow_samples as f64;
            ao_intensity /= shadow_samples as f64;

            let light_dir = (light_pos - hit_point).normalize();
            let light_intensity = normal.dot(&light_dir).max(0.0) * shadow_intensity;
            
            // Specular highlight
            let view_dir = (*origin - hit_point).normalize();
            let half_dir = (light_dir + view_dir).normalize();
            let specular = normal.dot(&half_dir).max(0.0).powf(32.0) * shadow_intensity;

            let base_color = sphere.color * light_intensity.max(0.2);
            let specular_color = Vec3::new(1.0, 1.0, 1.0) * specular * 0.5; // White specular highlight
            
            (base_color * 0.6 + reflect_color * 0.3 + specular_color) * ao_intensity.max(0.5)
        }
        None => {
            let t = 0.5 * (dir.y + 1.0);
            if dir.y < 0.0 {
                let scale = 0.3125;  // Quadrupled the size of checkers (1.25 / 4)
                let x = (origin.x + dir.x * (-origin.y / dir.y)).abs();
                let z = (origin.z + dir.z * (-origin.y / dir.y)).abs();
                let checker = (x * scale).floor() as i32 + (z * scale).floor() as i32;
                if checker % 2 == 0 {
                    Vec3::new(0.05, 0.05, 0.05)  // Darker grey
                } else {
                    Vec3::new(0.95, 0.95, 0.95)  // Lighter grey
                }
            } else {
                // More vibrant sky gradient
                Vec3::new(0.4, 0.6, 1.0) * (1.0 - t) + Vec3::new(0.1, 0.1, 0.3) * t
            }
        }
    }
}

fn is_occluded(origin: &Vec3, dir: &Vec3, spheres: &[Sphere]) -> bool {
    for sphere in spheres {
        if let Some(_) = sphere.intersect(origin, dir) {
            return true;
        }
    }
    false
}

fn random_unit_vector(rng: &mut impl Rng) -> Vec3 {
    loop {
        let v = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        if v.dot(&v) < 1.0 {
            return v.normalize();
        }
    }
}
