pub mod vector;

pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn subtract(&self, v: Vector) -> Vector {
        Vector { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
    }

    pub fn normalized(&self) -> Vector {
        let m = (self.x * self.x + self.y * self.y + self.z + self.z).sqrt();

        Vector { x: self.x / m, y: self.y / m, z: self.z / m }
    }

    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}
