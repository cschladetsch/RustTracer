use crate::vec3::Vec3;

pub struct Sphere { pub center: Vec3, pub radius: f64, pub color: Vec3 }

impl Sphere {
    pub fn new(center: Vec3, radius: f64, color: Vec3) -> Self { Sphere { center, radius, color } }
    pub fn intersect(&self, origin: &Vec3, dir: &Vec3) -> Option<f64> {
        let oc = *origin - self.center;
        let a = dir.dot(dir);
        let b = 2.0 * oc.dot(dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > 0.0 { Some(t) } else { None }
        }
    }
}
