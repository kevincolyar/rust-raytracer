use crate::vector::Vector;
use crate::material::Material;
use crate::intersection::Intersection;
use crate::ray::Ray;

pub struct Sphere {
    pub position: Vector,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub fn intersection(&self, ray: &Ray, t: f64) -> Intersection {
        let dist = self.position.subtract(&ray.position);
        let b = ray.direction.dot(&dist);
        let d = b * b - dist.dot(&dist) + self.radius * self.radius;

        if d < 0.0 {
            return Intersection { t: t, success: false }
        }


        let t0 = b - d.sqrt();
        let t1 = b + d.sqrt();
        let mut t = t;
        let mut success = false;

        if (t0 > 0.1) && (t0 < t) {
            t = t0;
            success = true;
        }

        if (t1 > 0.1) && (t1 < t)  {
            t = t1;
            success = true;
        }

        Intersection { t: t, success: success }
    }
}
