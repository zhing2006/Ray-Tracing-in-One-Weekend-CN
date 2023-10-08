pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod rtweekend;
pub mod interval;
pub mod camera;
pub mod material;

use std::rc::Rc;

use vec3::Point3;
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;

fn main() {
  // World
  let mut world = HittableList::default();

  let material_ground: Rc<dyn material::Material>
    = Rc::new(material::Lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
  let material_center: Rc<dyn material::Material>
    = Rc::new(material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
  let material_left: Rc<dyn material::Material>
    = Rc::new(material::Dielectric::new(1.5));
  let material_right: Rc<dyn material::Material>
    = Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 0.0));

  world.add(Rc::new(Sphere::new(
    Point3::new(0.0, -100.5, -1.0),
    100.0,
    material_ground,
  )));
  world.add(Rc::new(Sphere::new(
    Point3::new(0.0, 0.0, -1.0),
    0.5,
    material_center,
  )));
  world.add(Rc::new(Sphere::new(
    Point3::new(-1.0, 0.0, -1.0),
    0.5,
    Rc::clone(&material_left),
  )));
  world.add(Rc::new(Sphere::new(
    Point3::new(-1.0, 0.0, -1.0),
    -0.4,
    material_left,
  )));
  world.add(Rc::new(Sphere::new(
    Point3::new(1.0, 0.0, -1.0),
    0.5,
    material_right,
  )));

  // Camera
  let mut cam = Camera::default();
  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 100;
  cam.max_depth = 50;

  cam.vfov = 20.0;
  cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
  cam.lookat = Point3::new(0.0, 0.0, -1.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  // Render
  cam.render(&world);
}
