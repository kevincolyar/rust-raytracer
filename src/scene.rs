use crate::vector::Vector;
use crate::sphere::Sphere;
use crate::light::Light;

pub struct Scene {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub eye: Vector,
    pub _plane: f64,
}
