use image::{RgbImage, Rgb, ImageBuffer};

struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn subtract(&self, v: &Vector) -> Vector {
        Vector { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
    }

    pub fn add(&self, v: &Vector) -> Vector {
        Vector { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }

    pub fn multiply(&self, s: f64) -> Vector {
        Vector { x: self.x * s, y: self.y * s, z: self.z * s }
    }

    pub fn normalized(&self) -> Vector {
        let m = (self.x * self.x + self.y * self.y + self.z + self.z).sqrt();

        Vector { x: self.x / m, y: self.y / m, z: self.z / m }
    }

    pub fn dot(&self, v: &Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

struct Color {
    r: f64,
    g: f64,
    b: f64,
}

struct Light {
    position: Vector,
    color: Color
}

struct Material {
    diffuse: Color,
    reflection: f64,
    _specular: Color,
    power: f64,
}

struct Ray {
    position: Vector,
    direction: Vector,
}

struct Scene {
    objects: Vec<Sphere>,
    lights: Vec<Light>,
    eye: Vector,
    _plane: f64,
}

struct Sphere {
    position: Vector,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn intersection(&self, ray: &Ray, t: f64) -> Intersection {
        let dist = self.position.subtract(&ray.position);
        let b = ray.direction.dot(&dist);
        let d = b * b - dist.dot(&dist) + self.radius * self.radius;

        if d < 0.0 {
            return Intersection { t: t, success: false }
        }

        let t0 = b - d.sqrt();
        let t1 = b + d.sqrt();
        let mut t = t;
        let mut success = false;

        if (t0 > 0.1) && (t0 < t) {
            t = t0;
            success = true;
        }

        if (t1 > 0.1) && (t1 < t)  {
            t = t1;
            success = true;
        }
        Intersection { t: t, success: success }
    }
}

struct Intersection {
    t: f64,
    success: bool,
}


fn render(scene: &Scene, img: &mut RgbImage){
    let width = 500.0;
    let height = 500.0;

    for x in 0..width as i64 {
        for y in 0..height as i64 {
            let mut coef  = 1.0;
            let mut level = 0;
            let mut pixel = Color { r: 0.0, g: 0.0, b: 0.0 };
            let mut ray = Ray {
                position: Vector { x: scene.eye.x, y: scene.eye.y, z: scene.eye.z},
                direction: Vector { x: x as f64 - width / 2.0, y: height / 2.0 - y as f64, z: 0.0 }.subtract(&scene.eye).normalized()
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

fn main() {
    let white = Color { r: 1.0, g: 1.0, b: 1.0 };
    let purple = Color { r: 1.0, g: 0.0, b: 1.0 };

    // Create objects
    let objects = vec![
        Sphere {
            position: Vector { x: 100.0, y: 100.0, z: 0.0 },
            radius: 100.0,
            material: Material { diffuse: purple, reflection: 0.5, _specular: white, power: 60.0 }
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
            color: Color { r: 0.0, g: 1.0, b: 0.0 }
        }
    ];

    // Create eye
    let eye = Vector { x: 0.0, y: 0.0, z: 500.0 };

    // Create plane
    let plane = 100.0;

    // Create scene
    let scene = Scene { objects: objects, lights: lights, eye: eye, _plane: plane };

    // Create image
    let mut img: RgbImage = ImageBuffer::new(500, 500);

    // Ray tracer renders image
    render(&scene, &mut img);

    img.save("test.png").unwrap();
}
