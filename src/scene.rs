use crate::vector::Vector;
use crate::light::Light;
use crate::object::Object;

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub eye: Vector,
    pub plane: f64,
}
