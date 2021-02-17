const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    // print ppm
    // header
    println!("P3\n{} {}]\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let w_base = (IMAGE_WIDTH - 1) as f64;
    let h_base = (IMAGE_HEIGHT - 1) as f64;

    for j in 0..IMAGE_HEIGHT {
        let j = IMAGE_HEIGHT - 1 - j;
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / w_base;
            let g = j as f64 / h_base;
            let b = 0.25f64;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
