#[derive(Clone, Copy)]
pub struct Vec3 { pub x: f64, pub y: f64, pub z: f64 }

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Vec3 { x, y, z } }
    pub fn dot(&self, other: &Vec3) -> f64 { self.x * other.x + self.y * other.y + self.z * other.z }
    pub fn normalize(&self) -> Vec3 {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3::new(self.x / len, self.y / len, self.z / len)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 { Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z) }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 { Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z) }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f64) -> Vec3 { Vec3::new(self.x * t, self.y * t, self.z * t) }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 { Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z) }
}
