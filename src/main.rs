const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

use hkdray::Color;

fn main() {
    // print ppm
    // header
    println!("P3\n{} {}]\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let w_base = (IMAGE_WIDTH - 1) as f64;
    let h_base = (IMAGE_HEIGHT - 1) as f64;

    for j in 0..IMAGE_HEIGHT {
        let j = IMAGE_HEIGHT - 1 - j;
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let color = Color::new((i as f64) / w_base, (j as f64) / h_base, 0.25);
            println!("{}", color);
        }
    }

    eprintln!("\nDone.");
}
