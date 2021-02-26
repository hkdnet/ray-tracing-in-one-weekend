use hkdray::{ray_color, Point3, Ray, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT / ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0; // screen is where at z = -1.

// x is right, y is up, z is *back* (to be consistent with right-handed coordinate system)
fn main() {
    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner: Point3 =
        &(&(&origin - &(&horizontal / 2.)) - &(&vertical / 2.)) - &Vec3::new(0., 0., FOCAL_LENGTH);

    // print ppm
    // header
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let w_base = (IMAGE_WIDTH - 1) as f64;
    let h_base = (IMAGE_HEIGHT - 1) as f64;

    for j in 0..IMAGE_HEIGHT {
        let j = IMAGE_HEIGHT - 1 - j;
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / w_base;
            let v = (j as f64) / h_base;
            let d = &lower_left_corner;
            let point_v = -&origin;
            let point_v = point_v + &horizontal * u;
            let point_v = point_v + &vertical * v;
            let d = d + point_v;
            let color = ray_color(&Ray::new(&origin, &d));
            println!("{}", color);
        }
    }

    eprintln!("\nDone.");
}
