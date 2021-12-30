use image::{RgbImage, Rgb};

use crate::vector::Vector;
use crate::color::Color;
use crate::material::Material;
use crate::light::Light;
use crate::scene::Scene;
use crate::ray::Ray;
use crate::intersection::Intersection;

pub fn render(scene: &Scene, img: &mut RgbImage){
    let width = img.width();
    let height = img.height();

    for x in 0..width as i64 {
        for y in 0..height as i64 {
            let mut coef  = 1.0;
            let mut level = 0;
            let mut pixel = Color { r: 0.0, g: 0.0, b: 0.0 };
            let mut ray = Ray {
                position: Vector { x: scene.eye.x, y: scene.eye.y, z: scene.eye.z},
                direction: Vector { x: x as f64 - (width as f64 / 2.0), y: (height as f64 / 2.0) - y as f64, z: scene.plane }.subtract(&scene.eye).normalized()
            };

            loop {
                let mut t = 20000.0;

                let intersections: Vec<Intersection> = scene.objects
                                                            .iter()
                                                            .map(|sphere| sphere.intersection(&ray, t))
                                                            .filter(|intersection| intersection.success).collect();

                if intersections.len() < 1 {
                    break;
                }

                let intersection = &intersections[0];
                let current_sphere = &scene.objects[0]; // TODO: Needs to match up with intersection

                t = intersection.t;

                let intersection_position = ray.position.add(&ray.direction.multiply(t));
                let mut intersection_normal = intersection_position.subtract(&current_sphere.position);

                let temp = intersection_normal.dot(&intersection_normal);
                if temp == 0.0 { break; }

                let temp = 1.0 / temp.sqrt();
                intersection_normal = intersection_normal.multiply(temp);

                for light in &scene.lights {
                    let dist = light.position.subtract(&intersection_position);
                    if intersection_normal.dot(&dist) <= 0.0 { continue }

                    t = dist.dot(&dist).sqrt();
                    if t <= 0.0 { continue }

                    let light_ray = Ray {
                        position: Vector { x: intersection_position.x, y: intersection_position.y, z: intersection_position.z},
                        direction: dist.multiply(1.0/t)
                    };

                    let in_shadow = false;

                    // Detect Shadows
                    // for sphere in &scene.objects {
                    //     let intersection = current_sphere.intersection(&light_ray, t);

                    //     if intersection.success {
                    //         in_shadow = true;
                    //         t = intersection.t;
                    //     }

                    //     if in_shadow { break }
                    //   }

                    if in_shadow == false {
                        lambert(&mut pixel, &ray, &light_ray, &light, &intersection_normal, &current_sphere.material, coef);
                        phong(&mut pixel, &ray, &light_ray, &light, &intersection_normal, &current_sphere.material, coef);
                    }
               }

                coef = coef * current_sphere.material.reflection;
                let reflection = 2.0 * ray.direction.dot(&intersection_normal);

                ray.position = intersection_position;
                ray.direction = ray.direction.subtract(&intersection_normal.multiply(reflection));

                level = level + 1;

                if (coef <= 0.0) || (level >= 10) {
                    break
                }
            }

            img.put_pixel(x as u32, y as u32, Rgb([(pixel.r * 255.0) as u8, (pixel.g * 255.0) as u8, (pixel.b * 255.0) as u8]));
        }
    }
}

fn lambert(pixel: &mut Color, _ray: &Ray, light_ray: &Ray, light: &Light, intersection_normal: &Vector, material: &Material, coef: f64){
    let lambert = light_ray.direction.dot(intersection_normal) * coef;
    pixel.r = pixel.r + lambert * light.color.r * material.diffuse.r;
    pixel.g = pixel.g + lambert * light.color.g * material.diffuse.g;
    pixel.b = pixel.b + lambert * light.color.b * material.diffuse.b;
}

fn phong(pixel: &mut Color, ray: &Ray, light_ray: &Ray, light: &Light, intersection_normal: &Vector, material: &Material, coef: f64){
    let reflet = 2.0 * light_ray.direction.dot(intersection_normal);
    let phong_direction = light_ray.direction.subtract(&intersection_normal.multiply(reflet));
    let phong_term = phong_direction.dot(&ray.direction).max(0.0);
    let phong_term = 1.0 * phong_term.powf(material.power) * coef;

    pixel.r = pixel.r + phong_term * light.color.r;
    pixel.g = pixel.g + phong_term * light.color.g;
    pixel.b = pixel.b + phong_term * light.color.b;
}
