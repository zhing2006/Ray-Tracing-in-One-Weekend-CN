pub mod vec3;
pub mod color;
pub mod ray;

use vec3::{
  Vec3,
  Point3,
};
use color::Color;
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
  let unit_direction = vec3::unit_vector(r.direction());
  let a = 0.5 * (unit_direction.y() + 1.0);
  (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
  // Image
  let aspect_ratio = 16.0 / 9.0;
  let image_width = 400;

  // 计算图像高度，并确保至少为1。
  let image_height = image_width / aspect_ratio as i32;
  let image_height = if image_height < 1 { 1 } else { image_height };

  // Camera
  let focal_length = 1.0;
  let viewport_height = 2.0;
  let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
  let camera_center = Point3::default();

  // 计算水平和垂直视口边缘上的向量。
  let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
  let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

  // 计算从像素到像素的水平和垂直增量向量。
  let pixel_delta_u = viewport_u / image_width as f64;
  let pixel_delta_v = viewport_v / image_height as f64;

  // 计算左上角像素的位置。
  let viewport_upper_left = camera_center
    - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
  let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

  // Render
  println!("P3\n{} {}\n255", image_width, image_height);
  let stdout = std::io::stdout();

  for j in 0..image_height {
    eprintln!("\rScanlines remaining: {}", image_height - j);
    for i in 0..image_width {
      let pixel_center = pixel00_loc + i as f64 * pixel_delta_u + j as f64 * pixel_delta_v;
      let ray_direction = pixel_center - camera_center;
      let r = Ray::new(&camera_center, &ray_direction);

      let pixel_color = ray_color(&r);
      pixel_color.write_color(&mut stdout.lock()).unwrap();
    }
  }

  eprintln!("\nDone.");
}
