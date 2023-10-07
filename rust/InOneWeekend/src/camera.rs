use super::rtweekend;
use super::color::Color;
use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::interval::Interval;
use super::vec3::{self, Point3, Vec3};

pub struct Camera {
  pub aspect_ratio: f64,  // Ratio of image width over height
  pub image_width: i32,   // Rendered image width in pixel count
  pub samples_per_pixel: usize, // Count of random samples for each pixel
  pub max_depth: i32,     // Maximum number of ray bounces into scene
  image_height: i32,      // Rendered image height
  center: Point3,         // Camera center
  pixel00_loc: Point3,    // Location of pixel 0, 0
  pixel_delta_u: Vec3,    // Offset to pixel to the right
  pixel_delta_v: Vec3,    // Offset to pixel below
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      aspect_ratio: 1.0,
      image_width: 100,
      samples_per_pixel: 10,
      max_depth: 10,
      image_height: 0,
      center: Point3::default(),
      pixel00_loc: Point3::default(),
      pixel_delta_u: Vec3::default(),
      pixel_delta_v: Vec3::default(),
    }
  }
}

impl Camera {
  pub fn render(&mut self, world: &dyn Hittable) {
    self.initialize();

    println!("P3\n{} {}\n255", self.image_width, self.image_height);
    let stdout = std::io::stdout();

    for j in 0..self.image_height {
      eprintln!("\rScanlines remaining: {}", self.image_height - j);
      for i in 0..self.image_width {
        let mut pixel_color = Color::default();
        for _ in 0..self.samples_per_pixel {
          let r = self.get_ray(i, j);
          pixel_color += Self::ray_color(&r, self.max_depth, world);
        }
        pixel_color.write_color(&mut stdout.lock(), self.samples_per_pixel).unwrap();
      }
    }

    eprintln!("\nDone.");
  }

  fn initialize(&mut self) {
    self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
    self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

    self.center = Point3::default();

    // Determine viewport dimensions.
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

    // 计算水平和垂直视口边缘上的向量。
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // 计算从像素到像素的水平和垂直增量向量。
    self.pixel_delta_u = viewport_u / self.image_width as f64;
    self.pixel_delta_v = viewport_v / self.image_height as f64;

    // 计算左上角像素的位置。
    let viewport_upper_left = self.center
      - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
  }

  fn get_ray(&self, i: i32, j: i32) -> Ray {
    // Get a randomly sampled camera ray for the pixel at location i,j.
    let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
    let pixel_sample = pixel_center + self.pixel_sample_square();

    let ray_origin = self.center;
    let ray_direction = pixel_sample - ray_origin;

    Ray::new(&ray_origin, &ray_direction)
  }

  fn pixel_sample_square(&self) -> Vec3 {
    // Returns a random point in the square surrounding a pixel at the origin.
    let px = -0.5 + rtweekend::random_double();
    let py = -0.5 + rtweekend::random_double();
    px * self.pixel_delta_u + py * self.pixel_delta_v
  }

  fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();

    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
      return Color::default();
    }

    if world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
      let direction = rec.normal + vec3::random_unit_vector();
      return 0.5 * Self::ray_color(&Ray::new(&rec.p, &direction), depth - 1, world);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
  }
}