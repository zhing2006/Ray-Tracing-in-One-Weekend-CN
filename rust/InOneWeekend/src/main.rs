pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod rtweekend;
pub mod interval;
pub mod camera;

use std::rc::Rc;

use vec3::Point3;
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;

fn main() {
  // World
  let mut world = HittableList::default();
  world.add(Rc::new(Sphere::new(
    &Point3::new(0.0, 0.0, -1.0),
    0.5,
  )));
  world.add(Rc::new(Sphere::new(
    &Point3::new(0.0, -100.5, -1.0),
    100.0,
  )));

  // Camera
  let mut cam = Camera::default();
  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 100;

  // Render
  cam.render(&world);
}
