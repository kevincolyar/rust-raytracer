use crate::vector::Vector;

pub struct Intersection {
    pub t: f64,
    pub success: bool,
    pub normal: Vector,
    pub position: Vector
}
