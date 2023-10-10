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
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod rtw_stb_image;
pub mod perlin;
pub mod quad;

use std::rc::Rc;

use vec3::Point3;
use color::Color;
use sphere::Sphere;
use hittable_list::HittableList;
use camera::Camera;
use material::{
  Material,
  Lambertian,
  Metal,
  Dielectric,
  DiffuseLight,
};
use bvh::BvhNode;
use texture::{
  Texture,
  CheckerTexture,
  ImageTexture,
  NoiseTexture,
};
use quad::Quad;

fn random_spheres() {
  // World
  let mut world = HittableList::default();

  let ground_material: Rc<dyn Material> = Rc::new(
    Lambertian::new(color::Color::new(0.5, 0.5, 0.5))
  );
  world.add(Rc::new(
    Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
  ));

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = rtweekend::random_double();
      let center = Point3::new(
        a as f64 + 0.9 * rtweekend::random_double(),
        0.2,
        b as f64 + 0.9 * rtweekend::random_double(),
      );

      if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
        let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
          // diffuse
          let albedo = Color::random() * Color::random();
          Rc::new(Lambertian::new(albedo))
        } else if choose_mat < 0.95 {
          // metal
          let albedo = Color::random_range(0.5, 1.0);
          let fuzz = rtweekend::random_double_range(0.0, 0.5);
          Rc::new(Metal::new(albedo, fuzz))
        } else {
          // glass
          Rc::new(Dielectric::new(1.5))
        };

        let center2 = center + vec3::Vec3::new(0.0, rtweekend::random_double_range(0.0, 0.5), 0.0);
        world.add(Rc::new(
          Sphere::new_with_center2(center, center2, 0.2, sphere_material)
        ));
      }
    }
  }

  let material1: Rc<dyn Material> = Rc::new(
    Dielectric::new(1.5)
  );
  world.add(Rc::new(
    Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)
  ));

  let material2: Rc<dyn Material> = Rc::new(
    Lambertian::new(Color::new(0.4, 0.2, 0.1))
  );
  world.add(Rc::new(
    Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)
  ));

  let material3: Rc<dyn Material> = Rc::new(
    Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)
  );
  world.add(Rc::new(
    Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)
  ));

  let world = HittableList::new(Rc::new(BvhNode::new(&mut world)));

  // Camera
  let mut cam = Camera::default();
  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 50;
  cam.max_depth = 10;
  cam.background = Color::new(0.7, 0.8, 1.0);

  cam.vfov = 20.0;
  cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
  cam.lookat = Point3::new(0.0, 0.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.6;
  cam.focus_dist = 10.0;

  // Render
  cam.render(&world);
}

fn two_spheres() {
  let mut world = HittableList::default();

  let checker: Rc<dyn Texture> = Rc::new(CheckerTexture::new_with_color(0.8, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

  world.add(Rc::new(
    Sphere::new(
      Point3::new(0.0, -10.0, 0.0),
      10.0,
      Rc::new(Lambertian::new_with_texture(Rc::clone(&checker)))
    )
  ));
  world.add(Rc::new(
    Sphere::new(
      Point3::new(0.0, 10.0, 0.0),
      10.0,
      Rc::new(Lambertian::new_with_texture(Rc::clone(&checker)))
    )
  ));

  let mut cam = Camera::default();

  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 50;
  cam.max_depth = 10;
  cam.background = Color::new(0.7, 0.8, 1.0);

  cam.vfov = 20.0;
  cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
  cam.lookat = Point3::new(0.0, 0.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.0;

  cam.render(&world);
}

fn earth() {
  let earth_texture: Rc<dyn Texture> = Rc::new(ImageTexture::new("earthmap.jpg"));
  let earth_surface: Rc<dyn Material> = Rc::new(Lambertian::new_with_texture(Rc::clone(&earth_texture)));
  let globe = Rc::new(
    Sphere::new(
      Point3::new(0.0, 0.0, 0.0),
      2.0,
      earth_surface
    )
  );

  let mut cam = Camera::default();

  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 50;
  cam.max_depth = 10;
  cam.background = Color::new(0.7, 0.8, 1.0);

  cam.vfov = 20.0;
  cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
  cam.lookat = Point3::new(0.0, 0.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.0;

  cam.render(&HittableList::new(globe));
}

fn two_perlin_spheres() {
  let mut world = HittableList::default();

  let pertext: Rc<dyn Texture> = Rc::new(NoiseTexture::new(4.0));
  world.add(Rc::new(
    Sphere::new(
      Point3::new(0.0, -1000.0, 0.0),
      1000.0,
      Rc::new(Lambertian::new_with_texture(Rc::clone(&pertext)))
    )
  ));
  world.add(Rc::new(
    Sphere::new(
      Point3::new(0.0, 2.0, 0.0),
      2.0,
      Rc::new(Lambertian::new_with_texture(Rc::clone(&pertext)))
    )
  ));

  let mut cam = Camera::default();

  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 50;
  cam.max_depth = 10;
  cam.background = Color::new(0.7, 0.8, 1.0);

  cam.vfov = 20.0;
  cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
  cam.lookat = Point3::new(0.0, 0.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.0;

  cam.render(&world);
}

fn quads() {
  let mut world = HittableList::default();

  // Material
  let left_red: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
  let back_green: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
  let right_blue: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
  let upper_orange: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
  let lower_teal: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

  // Quad
  world.add(
    Rc::new(Quad::new(
      Point3::new(-3.0, -2.0, 5.0),
      vec3::Vec3::new(0.0, 0.0, -4.0),
      vec3::Vec3::new(0.0, 4.0, 0.0),
      left_red
    ))
  );
  world.add(
    Rc::new(Quad::new(
      Point3::new(-2.0, -2.0, 0.0),
      vec3::Vec3::new(4.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 4.0, 0.0),
      back_green
    ))
  );
  world.add(
    Rc::new(Quad::new(
      Point3::new(3.0, -2.0, 1.0),
      vec3::Vec3::new(0.0, 0.0, 4.0),
      vec3::Vec3::new(0.0, 4.0, 0.0),
      right_blue
    ))
  );
  world.add(
    Rc::new(Quad::new(
      Point3::new(-2.0, 3.0, 1.0),
      vec3::Vec3::new(4.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, 4.0),
      upper_orange
    ))
  );
  world.add(
    Rc::new(Quad::new(
      Point3::new(-2.0, -3.0, 5.0),
      vec3::Vec3::new(4.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, -4.0),
      lower_teal
    ))
  );

  let mut cam = Camera::default();

  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 50;
  cam.max_depth = 10;
  cam.background = Color::new(0.7, 0.8, 1.0);

  cam.vfov = 80.0;
  cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
  cam.lookat = Point3::new(0.0, 0.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.0;

  cam.render(&world);
}

fn simple_light() {
  let mut world = HittableList::default();

  let pertext: Rc<dyn Texture> = Rc::new(NoiseTexture::new(4.0));
  world.add(Rc::new(
    Sphere::new(
      Point3::new(0.0, -1000.0, 0.0),
      1000.0,
      Rc::new(Lambertian::new_with_texture(Rc::clone(&pertext)))
    )
  ));
  world.add(Rc::new(
    Sphere::new(
      Point3::new(0.0, 2.0, 0.0),
      2.0,
      Rc::new(Lambertian::new_with_texture(pertext))
    )
  ));

  let difflight: Rc<dyn Material> = Rc::new(DiffuseLight::new_with_color(Color::new(4.0, 4.0, 4.0)));
  world.add(Rc::new(
    Sphere::new(
      Point3::new(0.0, 7.0, 0.0),
      2.0,
      Rc::clone(&difflight)
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(3.0, 1.0, -2.0),
      vec3::Vec3::new(2.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 2.0, 0.0),
      difflight
    )
  ));

  let mut cam = Camera::default();

  cam.aspect_ratio = 16.0 / 9.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 50;
  cam.max_depth = 10;
  cam.background = Color::default();

  cam.vfov = 20.0;
  cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
  cam.lookat = Point3::new(0.0, 2.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.0;

  cam.render(&world);
}

fn cornell_box() {
  let mut world = HittableList::default();

  let red: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
  let white: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
  let green: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
  let light: Rc<dyn Material> = Rc::new(DiffuseLight::new_with_color(Color::new(15.0, 15.0, 15.0)));

  world.add(Rc::new(
    Quad::new(
      Point3::new(555.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 555.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, 555.0),
      green
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(0.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 555.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, 555.0),
      red
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(343.0, 554.0, 332.0),
      vec3::Vec3::new(-130.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, -105.0),
      light
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(0.0, 0.0, 0.0),
      vec3::Vec3::new(555.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, 555.0),
      Rc::clone(&white)
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(555.0, 555.0, 555.0),
      vec3::Vec3::new(-555.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 0.0, -555.0),
      Rc::clone(&white)
    )
  ));
  world.add(Rc::new(
    Quad::new(
      Point3::new(0.0, 0.0, 555.0),
      vec3::Vec3::new(555.0, 0.0, 0.0),
      vec3::Vec3::new(0.0, 555.0, 0.0),
      white
    )
  ));

  let mut cam = Camera::default();

  cam.aspect_ratio = 1.0;
  cam.image_width = 400;
  cam.samples_per_pixel = 50;
  cam.max_depth = 10;
  cam.background = Color::default();

  cam.vfov = 40.0;
  cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
  cam.lookat = Point3::new(278.0, 278.0, 0.0);
  cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

  cam.defocus_angle = 0.0;

  cam.render(&world);
}

fn main() {
  let now = std::time::Instant::now();

  match 7 {
    1 => random_spheres(),
    2 => two_spheres(),
    3 => earth(),
    4 => two_perlin_spheres(),
    5 => quads(),
    6 => simple_light(),
    7 => cornell_box(),
    _ => (),
  }

  let elapsed = now.elapsed();
  eprintln!("Elapsed: {}.{:03}s", elapsed.as_secs(), elapsed.subsec_millis());
}