pub mod vec3;
pub mod color;

fn main() {
  // Image
  let image_width = 256;
  let image_height = 256;

  // Render
  println!("P3\n{} {}\n255", image_width, image_height);
  let stdout = std::io::stdout();

  for j in 0..image_height {
    eprintln!("\rScanlines remaining: {}", image_height - j);
    for i in 0..image_width {
      let pixel_color = color::Color::new(
        i as f64 / (image_width - 1) as f64,
        j as f64 / (image_height - 1) as f64,
        0.0);
      pixel_color.write_color(&mut stdout.lock()).unwrap();
    }
  }

  eprintln!("\nDone.");
}
