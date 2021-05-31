use hkdray::{
    ray_color, Camera, Color, ColorIndex, HittableList, Lambertian, Metal, Point3, Sphere,
};
use rand::random;
use std::rc::Rc;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2.0;
const FOCAL_LENGTH: f64 = 1.0; // screen is where at z = -1.

// x is right, y is up, z is *back* (to be consistent with right-handed coordinate system)
fn main() {
    let samples_per_pixel = 100;
    let camera = Camera::new(ASPECT_RATIO, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    // print ppm
    // header
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let w_base = (IMAGE_WIDTH - 1) as f64;
    let h_base = (IMAGE_HEIGHT - 1) as f64;

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 0., -1.),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    let boxed_world = Box::new(world);
    let max_depth = 50;

    for j in 0..IMAGE_HEIGHT {
        let j = IMAGE_HEIGHT - 1 - j;
        eprint!("\rScanlines remaining: {:03}", j);
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / w_base;
                let v = (j as f64 + random::<f64>()) / h_base;
                let ray = camera.get_ray(u, v);
                color += &ray_color(&ray, boxed_world.as_ref(), max_depth);
            }
            write_color(color, samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}

fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1f64 / (samples_per_pixel as f64);
    let r = pixel_color[ColorIndex::R] * scale;
    let g = pixel_color[ColorIndex::G] * scale;
    let b = pixel_color[ColorIndex::B] * scale;

    // For gamma-correction, sqrt is required.
    let r = r.sqrt().clamp(0., 0.999) * 256.;
    let g = g.sqrt().clamp(0., 0.999) * 256.;
    let b = b.sqrt().clamp(0., 0.999) * 256.;

    println!("{} {} {}", r as i32, g as i32, b as i32)
}
