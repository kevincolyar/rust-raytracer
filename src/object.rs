use crate::vector::Vector;
use crate::material::Material;
use crate::intersection::Intersection;
use crate::ray::Ray;

pub enum ObjectType {
    Sphere,
    Plane
}

pub struct Object {
    pub position: Vector,
    pub material: Material,
    pub object_type: ObjectType,
    pub radius: f64,   // TODO: For Sphere
    pub normal: Vector // TODO: For Plane
}

impl Object {
    pub fn intersection(&self, ray: &Ray, t: f64) -> Intersection {
        match self.object_type {
            ObjectType::Sphere => { self.intersection_sphere(ray, t) },
            ObjectType::Plane => { self.intersection_plane(ray, t) }
        }
    }

    fn intersection_plane(&self, ray: &Ray, t: f64) -> Intersection {
        // let denom = self.normal.dot(&ray.direction);
        let denom = ray.direction.dot(&self.normal);

        if denom.abs() > 0.000001 {
            let tmp_t = self.position.subtract(&ray.position).dot(&self.normal) / denom;
            if tmp_t >= 0.000001 {
                let position = ray.position.add(&ray.direction.multiply(tmp_t));
                return Intersection { t: tmp_t, success: true, normal: self.normal.clone(), position: position }
            }
        }

        Intersection { t: t, success: false, normal: Vector::default(), position: Vector::default() }
    }

    fn intersection_sphere(&self, ray: &Ray, t: f64) -> Intersection {
        let dist = self.position.subtract(&ray.position);
        let b = ray.direction.dot(&dist);
        let d = b * b - dist.dot(&dist) + self.radius * self.radius;

        if d < 0.0 {
            return Intersection { t: t, success: false, normal: Vector::default(), position: Vector::default() }
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


        let position = ray.position.add(&ray.direction.normalized().multiply(t));
        let normal = position.subtract(&self.position).normalized();

        Intersection { t: t, success: success, normal: normal, position: position  }
    }
}
