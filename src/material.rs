use crate::color::Color;

pub struct Material {
    pub diffuse: Color,
    pub reflection: f64,
    pub _specular: Color,
    pub power: f64,
}
