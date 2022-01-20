use image::{RgbImage, ImageBuffer};

// Define modules in our crate
pub mod vector;
pub mod color;
pub mod light;
pub mod material;
pub mod intersection;
pub mod scene;
pub mod ray;
pub mod ray_tracer;
pub mod shaders;
pub mod object;

use crate::vector::Vector;
use crate::color::Color;
use crate::material::Material;
use crate::light::Light;
use crate::scene::Scene;
use crate::object::{Object,ObjectType};
use crate::ray_tracer::render;

fn main() {

    // Create objects
    let objects: Vec<Object> = vec![
        Object {
            object_type: ObjectType::Sphere,
            position: Vector { x: 100.0, y: 100.0, z: 0.0 },
            radius: 100.0,
            normal: Vector::default(), // TODO: Not needed for Sphere
            material: Material { diffuse: Color { r: 1.0, g: 0.0, b: 1.0 }, reflection: 0.5, power: 60.0 }
        },
        Object {
            object_type: ObjectType::Sphere,
            position: Vector { x: -100.0, y: 100.0, z: 0.0 },
            radius: 100.0,
            normal: Vector::default(), // TODO: Not needed for Sphere
            material: Material { diffuse: Color { r: 0.0, g: 0.0, b: 1.0 }, reflection: 0.5, power: 60.0 }
        },
        Object {
            object_type: ObjectType::Sphere,
            position: Vector { x: -100.0, y: -100.0, z: 0.0 },
            radius: 100.0,
            normal: Vector::default(), // TODO: Not needed for Sphere
            material: Material { diffuse: Color { r: 1.0, g: 1.0, b: 1.0 }, reflection: 0.8, power: 60.0 }
        },
        Object {
            object_type: ObjectType::Sphere,
            position: Vector { x: 100.0, y: -100.0, z: 0.0 },
            radius: 100.0,
            normal: Vector::default(), // TODO: Not needed for Sphere
            material: Material { diffuse: Color { r: 0.0, g: 1.0, b: 0.0 }, reflection: 0.2, power: 60.0 }
        },
        // Bottom
        Object {
            object_type: ObjectType::Plane,
            position: Vector { x: 0.0, y: -200.0, z: 0.0 },
            radius: 0.0, // TODO: Not needed for Plane
            normal: Vector { x: 0.0, y: 1.0, z: 0.0 },
            material: Material { diffuse: Color { r: 0.5, g: 0.8, b: 0.8 }, reflection: 0.5, power: 60.0 }
        },
        // Top
        Object {
            object_type: ObjectType::Plane,
            position: Vector { x: 0.0, y: 200.0, z: 0.0 },
            radius: 0.0, // TODO: Not needed for Plane
            normal: Vector { x: 0.0, y: -1.0, z: 0.0 },
            material: Material { diffuse: Color { r: 0.5, g: 0.5, b: 0.5 }, reflection: 0.8, power: 60.0 }
        },

        // Left
        Object {
            object_type: ObjectType::Plane,
            position: Vector { x: -200.0, y: 0.0, z: 0.0 },
            radius: 100.0, // TODO: Not needed for Plane
            normal: Vector { x: 1.0, y: 0.0, z: 0.0 },
            material: Material { diffuse: Color { r: 1.0, g: 0.0, b: 0.0 }, reflection: 0.3, power: 60.0 }
        },
        // Right
        Object {
            object_type: ObjectType::Plane,
            position: Vector { x: 200.0, y: 0.0, z: 0.0 },
            radius: 100.0, // TODO: Not needed for Plane
            normal: Vector { x: -1.0, y: 0.0, z: 0.0 },
            material: Material { diffuse: Color { r: 0.0, g: 0.0, b: 1.0 }, reflection: 0.3, power: 60.0 }
        },
        // Back
        Object {
            object_type: ObjectType::Plane,
            position: Vector { x: 0.0, y: 0.0, z: -500.0 },
            radius: 100.0, // TODO: Not needed for Plane
            normal: Vector { x: 0.0, y: 0.0, z: 1.0 },
            material: Material { diffuse: Color { r: 1.0, g: 1.0, b: 1.0 }, reflection: 0.3, power: 60.0 }
        }

    ];

    // Create lights
    let lights = vec![
        Light {
            position: Vector { x: 500.0, y: 500.0, z: 500.0 },
            color: Color { r: 1.0, g: 1.0, b: 1.0 }
        },
        Light {
            position: Vector { x: -500.0, y: 500.0, z: 500.0 },
            color: Color { r: 1.0, g: 1.0, b: 1.0 }
        }
    ];

    // Create eye
    let eye = Vector { x: 0.0, y: 100.0, z: 500.0 };

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
