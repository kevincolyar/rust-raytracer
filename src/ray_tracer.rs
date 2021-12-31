use image::{RgbImage, Rgb};

use crate::vector::Vector;
use crate::color::Color;
use crate::scene::Scene;
use crate::ray::Ray;

use crate::shaders::lambert;
use crate::shaders::phong;

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
                let mut t = 20000.0; // TODO: Magic number

                let mut intersection = None;
                let mut current_sphere = None;

                // TODO: This should find the closest intersection (smallest t), not the first.
                for sphere in scene.objects.iter() {
                    let i = sphere.intersection(&ray, t);
                    if i.success {
                        intersection = Some(i);
                        current_sphere = Some(sphere);
                        break;
                    }
                }

                if intersection.is_none() { break }

                t = intersection.unwrap().t;

                let intersection_position = ray.position.add(&ray.direction.multiply(t));
                let mut intersection_normal = intersection_position.subtract(&current_sphere.unwrap().position);

                let temp = intersection_normal.dot(&intersection_normal);
                if temp == 0.0 { break }

                let temp = 1.0 / temp.sqrt();
                intersection_normal = intersection_normal.multiply(temp);

                'lights: for light in &scene.lights {
                    let dist = light.position.subtract(&intersection_position);
                    if intersection_normal.dot(&dist) <= 0.0 { continue }

                    t = dist.dot(&dist).sqrt();
                    if t <= 0.0 { continue }

                    let light_ray = Ray {
                        position: Vector { x: intersection_position.x, y: intersection_position.y, z: intersection_position.z},
                        direction: dist.multiply(1.0/t)
                    };

                    // Detect Shadows
                    for sphere in &scene.objects {
                        let intersection = sphere.intersection(&light_ray, t);

                        if intersection.success {
                            break 'lights;
                        }
                    }

                    lambert(&mut pixel, &ray, &light_ray, &light, &intersection_normal, &current_sphere.unwrap().material, coef);
                    phong(&mut pixel, &ray, &light_ray, &light, &intersection_normal, &current_sphere.unwrap().material, coef);
               }

                // Reflections
                coef = coef * current_sphere.unwrap().material.reflection;
                let reflection = 2.0 * ray.direction.dot(&intersection_normal);

                ray.position = intersection_position;
                ray.direction = ray.direction.subtract(&intersection_normal.multiply(reflection));

                level = level + 1;

                if (coef <= 0.0) || (level >= 10) { // TODO: Magic number
                    break
                }
            }

            img.put_pixel(x as u32, y as u32, Rgb([(pixel.r * 255.0) as u8, (pixel.g * 255.0) as u8, (pixel.b * 255.0) as u8]));
        }
    }
}
