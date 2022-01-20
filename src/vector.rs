#[derive(Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn default() -> Self {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn subtract(&self, v: &Vector) -> Vector {
        Vector { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
    }

    pub fn add(&self, v: &Vector) -> Vector {
        Vector { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }

    pub fn multiply(&self, s: f64) -> Vector {
        Vector { x: self.x * s, y: self.y * s, z: self.z * s }
    }

    pub fn normalized(&self) -> Vector {
        let m = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        Vector { x: self.x / m, y: self.y / m, z: self.z / m }
    }

    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}
