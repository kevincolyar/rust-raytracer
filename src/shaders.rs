use crate::vector::Vector;
use crate::ray::Ray;
use crate::color::Color;
use crate::material::Material;
use crate::light::Light;

pub fn lambert(pixel: &mut Color, _ray: &Ray, light_ray: &Ray, light: &Light, intersection_normal: &Vector, material: &Material, coef: f64){
    let lambert = light_ray.direction.dot(intersection_normal) * coef;
    pixel.r = pixel.r + lambert * light.color.r * material.diffuse.r;
    pixel.g = pixel.g + lambert * light.color.g * material.diffuse.g;
    pixel.b = pixel.b + lambert * light.color.b * material.diffuse.b;
}

pub fn phong(pixel: &mut Color, ray: &Ray, light_ray: &Ray, light: &Light, intersection_normal: &Vector, material: &Material, coef: f64){
    let reflet = 2.0 * light_ray.direction.dot(intersection_normal);
    let phong_direction = light_ray.direction.subtract(&intersection_normal.multiply(reflet));
    let phong_term = phong_direction.dot(&ray.direction).max(0.0);
    let phong_term = 1.0 * phong_term.powf(material.power) * coef;

    pixel.r = pixel.r + phong_term * light.color.r;
    pixel.g = pixel.g + phong_term * light.color.g;
    pixel.b = pixel.b + phong_term * light.color.b;
}
