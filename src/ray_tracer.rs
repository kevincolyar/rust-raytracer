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
                let mut smallest_t = f64::MAX;

                let mut current_intersection = None;
                let mut current_obj = None;

                for obj in scene.objects.iter() {
                    let i = obj.intersection(&ray, t);
                    if i.success && (i.t < smallest_t) {
                        smallest_t = i.t;
                        current_intersection = Some(i);
                        current_obj = Some(obj);
                    }
                }

                if current_intersection.is_none() { break }

                let intersection = current_intersection.unwrap();

                let temp = intersection.normal.dot(&intersection.normal);
                if temp == 0.0 { break }

                let temp = 1.0 / temp.sqrt();
                let intersection_normal_temp = intersection.normal.multiply(temp);

                'lights: for light in &scene.lights {
                    let dist = light.position.subtract(&intersection.position);
                    if intersection.normal.dot(&dist) <= 0.0 { continue }

                    t = dist.dot(&dist).sqrt();
                    if t <= 0.0 { continue }

                    let light_ray = Ray {
                        position: intersection.position.clone(),
                        direction: dist.multiply(t).normalized()
                    };

                    // Detect Shadows
                    // for obj in &scene.objects {
                    //     let intersection = obj.intersection(&light_ray, t);

                    //     if intersection.success {
                    //         break 'lights;
                    //     }
                    // }

                    lambert(&mut pixel, &ray, &light_ray, &light, &intersection.normal, &current_obj.unwrap().material, coef);
                    phong(&mut pixel, &ray, &light_ray, &light, &intersection.normal, &current_obj.unwrap().material, coef);
               }

                // Reflections
                coef = coef * current_obj.unwrap().material.reflection;
                let reflection = 2.0 * ray.direction.dot(&intersection_normal_temp);

                ray.position = intersection.position.clone();
                ray.direction = ray.direction.subtract(&intersection.normal.multiply(reflection));

                level = level + 1;

                if (coef <= 0.0) || (level >= 10) { // TODO: Magic number
                    break
                }
            }

            img.put_pixel(x as u32, y as u32, Rgb([(pixel.r * 255.0) as u8, (pixel.g * 255.0) as u8, (pixel.b * 255.0) as u8]));
        }
    }
}
