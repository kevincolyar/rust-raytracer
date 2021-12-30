use image::{RgbImage, ImageBuffer};

// Define modules in our crate
pub mod vector;
pub mod sphere;
pub mod color;
pub mod light;
pub mod material;
pub mod intersection;
pub mod scene;
pub mod ray;
pub mod ray_tracer;
pub mod shaders;

use crate::vector::Vector;
use crate::sphere::Sphere;
use crate::color::Color;
use crate::material::Material;
use crate::light::Light;
use crate::scene::Scene;
use crate::ray_tracer::render;

fn main() {

    // Create objects
    let objects = vec![
        Sphere {
            position: Vector { x: 100.0, y: 100.0, z: 0.0 },
            radius: 100.0,
            material: Material { diffuse: Color { r: 1.0, g: 0.0, b: 1.0 }, reflection: 0.5, power: 60.0 }
        },
        Sphere {
            position: Vector { x: -100.0, y: 100.0, z: 0.0 },
            radius: 100.0,
            material: Material { diffuse: Color { r: 0.0, g: 0.0, b: 1.0 }, reflection: 0.5, power: 60.0 }
        },
        Sphere {
            position: Vector { x: -100.0, y: -100.0, z: 0.0 },
            radius: 100.0,
            material: Material { diffuse: Color { r: 1.0, g: 1.0, b: 1.0 }, reflection: 0.8, power: 60.0 }
        },
        Sphere {
            position: Vector { x: 100.0, y: -100.0, z: 0.0 },
            radius: 100.0,
            material: Material { diffuse: Color { r: 0.0, g: 1.0, b: 0.0 }, reflection: 0.2, power: 60.0 }
        },
    ];

    // Create lights
    let lights = vec![
        Light {
            position: Vector { x: 500.0, y: 500.0, z: 500.0 },
            color: Color { r: 1.0, g: 0.0, b: 1.0 }
        },
        Light {
            position: Vector { x: -500.0, y: 500.0, z: 500.0 },
            color: Color { r: 0.0, g: 1.0, b: 1.0 }
        }
    ];

    // Create eye
    let eye = Vector { x: 0.0, y: 0.0, z: 500.0 };

    // Create plane
    let plane = 100.0;

    // Create scene
    let scene = Scene { objects: objects, lights: lights, eye: eye, plane: plane };

    // Create image
    let mut img: RgbImage = ImageBuffer::new(500, 500);

    // Ray tracer renders image
    render(&scene, &mut img);

    // Save image
    img.save("test.png").unwrap();
}
