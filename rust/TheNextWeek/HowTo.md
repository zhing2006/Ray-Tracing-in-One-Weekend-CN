# 使用Rust实现RayTracingTheNextWeek

## 初始化

使用`cargo`创建工程

    cargo new --name the_next_week TheNextWeek



## 运动模糊

### SpaceTime光线追踪简介

```rust
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
+   tm: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig,
            dir,
+           tm: 0.0,
        }
    }

+   pub fn new_with_time(orig: Point3, dir: Vec3, tm: f64) -> Self {
+       Self {
+           orig,
+           dir,
+           tm,
+       }
+   }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

+   pub fn time(&self) -> f64 {
+       self.tm
+   }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
```
Listing 1: [ray.rs] 带有时间信息的光线

由于Rust不允许函数重名，因此不同的构造函数之后都以new_with_xxx方式命名。


### 更新相机以模拟运动模糊

```rust
impl Camera {
      fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.
        let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
+       let ray_time = rtweekend::random_double();

+       Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }
}
```
Listing 2: [camera.rs] 带有时间信息的相机

```rust
pub struct Sphere {
+   center1: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
+   is_moving: bool,
+   center_vec: Vec3,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
+           center1: center,
            radius,
            mat: material,
+           is_moving: false,
+           center_vec: Vec3::default(),
        }
    }

+   pub fn new_with_center2(center1: Point3, center2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
+       Self {
+           center1,
+           radius,
+           mat: material,
+           is_moving: true,
+           center_vec: center2 - center1,
+       }
+   }
+
+   fn sphere_center(&self, time: f64) -> Point3 {
+       self.center1 + self.center_vec * time
+   }

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        ...

        hit_record.t = root;
        hit_record.p = r.at(hit_record.t);
+       let outward_normal = (hit_record.p - self.center1) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
        hit_record.mat = Some(Rc::clone(&self.mat));

        true
    }
}
```
Listing 3: [sphere.rs] 移动的球体

```rust
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
+       let center = if self.is_moving { self.sphere_center(r.time()) } else { self.center1 };
+       let oc = center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        ...
    }
}
```
Listing 4: [sphere.rs] 移动的球体的碰撞函数


### 追踪光线交点的时间

```rust
impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // 捕捉退化的散射方向
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

+       *scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
+       *scattered = Ray::new_with_time(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo;
        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        ...
+       *scattered = Ray::new_with_time(rec.p, direction, r_in.time());
        true
    }
}
```
Listing 5: [material.rs] 在 material::scatter() 方法中处理光线时间


### 将所有内容放在一起

```rust
fn main() {
    ...

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

+               let center2 = center + vec3::Vec3::new(0.0, rtweekend::random_double_range(0.0, 0.5), 0.0);
                world.add(Rc::new(
+                   Sphere::new_with_center2(center, center2, 0.2, sphere_material)
                ));
            }
        }
    }

    ...
}
```
Listing 6: [main.rs] 上一本书的最终场景，但是球会移动

![图像 1: 弹跳的球](../../images/img-2.01-bouncing-spheres.png)


## 包围体层次结构

### 光线与AABB的相交

```rust
+#[derive(Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

+impl Default for Interval {
+   fn default() -> Self {
+       Self {
+           min: rtweekend::INFINITY,
+           max: -rtweekend::INFINITY,
+       }
+   }
+}

impl Interval {
    ...
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

+   pub fn expand(&self, delta: f64) -> Interval {
+       let padding = delta / 2.0;
+       Interval::new(self.min - padding, self.max + padding)
+   }

    ...
}
```
Listing 7: [interval.rs] interval::expand() 方法

```rust
use super::vec3::Point3;
use super::interval::Interval;
use super::ray::Ray;

#[derive(Default, Clone)]  // 默认的AABB是空的，因为区间默认是空的。
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        Self {
            x: (*x).clone(),
            y: (*y).clone(),
            z: (*z).clone(),
        }
    }

    pub fn new_with_point(a: &Point3, b: &Point3) -> Self {
        // 将两个点a和b视为包围盒的极值，这样我们不需要特定的最小/最大坐标顺序。
        let mut _self = Self {
            x: Interval::new((a[0]).min(b[0]), (a[0]).max(b[0])),
            y: Interval::new((a[1]).min(b[1]), (a[1]).max(b[1])),
            z: Interval::new((a[2]).min(b[2]), (a[2]).max(b[2])),
        };
        _self.pad_to_minimums();
        _self
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        for a in 0..3 {
            let t0 = ((self.axis(a).min - r.origin()[a]) / r.direction()[a]).min(
                (self.axis(a).max - r.origin()[a]) / r.direction()[a],
            );
            let t1 = ((self.axis(a).min - r.origin()[a]) / r.direction()[a]).max(
                (self.axis(a).max - r.origin()[a]) / r.direction()[a],
            );
            ray_t.min = t0.max(ray_t.min);
            ray_t.max = t1.min(ray_t.max);
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    fn pad_to_minimums(&mut self) {
        // 调整AABB，使得没有一边比某个delta更窄，如果需要的话进行填充。
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z.expand(delta);
        }
    }
}
```
Listing 8: [aabb.rs] 轴对齐包围盒类


### 优化的AABB相交方法

```rust
impl Aabb {
    ...
    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        for a in 0..3 {
            let inv0 = 1.0 / r.direction()[a];
            let orig = r.origin()[a];

            let mut t0 = (self.axis(a).min - orig) * inv0;
            let mut t1 = (self.axis(a).max - orig) * inv0;

            if inv0 < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > ray_t.min {
                ray_t.min = t0;
            }
            if t1 < ray_t.max {
                ray_t.max = t1;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }
    ...
}
```
Listing 9: [aabb.rs] 可选的优化的AABB相交函数


### 构建可击中物体的包围盒

```rust
pub trait Hittable {
  fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
  fn bounding_box(&self) -> &Aabb;
}
```
Listing 10: [hittable.rs] 具有包围盒的可击中物体类

原文中bounding_box返回值，但因为我没有为Aabb实现Copy Trait，因此这里改为返回引用。

```rust
pub struct Sphere {
    center1: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
+   bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
+       let rvec = Vec3::new(radius, radius, radius);
        Self {
            center1: center,
            radius,
            mat: material,
            is_moving: false,
            center_vec: Vec3::default(),
+           bbox: Aabb::new_with_point(&(center - rvec), &(center + rvec)),
        }
    }
    ...
}

impl Hittable for Sphere {
    ...

+   pub fn bounding_box(&self) -> &Aabb {
+       &self.bbox
+   }
    ...
}
```
Listing 11: [sphere.rs] 具有包围盒的球体类

```rust
impl Sphere {
    ...
    pub fn new_with_center2(center1: Point3, center2: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
+       let rvec = Vec3::new(radius, radius, radius);
+       let box1 = Aabb::new_with_point(&(center1 - rvec), &(center1 + rvec));
+       let box2 = Aabb::new_with_point(&(center2 - rvec), &(center2 + rvec));
        Self {
            center1,
            radius,
            mat: material,
            is_moving: true,
            center_vec: center2 - center1,
+           bbox: Aabb::new_with_box(&box1, &box2),
        }
    }
    ...
}
```
Listing 12: [sphere.rs] 具有包围盒的移动球体类

```rust
impl Interval {
    ...
+   pub fn new_with_interval(a: &Interval, b: &Interval) -> Self {
+       Self {
+           min: a.min.min(b.min),
+           max: a.max.max(b.max),
+       }
+   }
    ...
}
```
Listing 13: [interval.rs] 从两个区间构造区间的构造函数

```rust
impl Aabb {
    ...
+   pub fn new_with_box(box0: &Aabb, box1: &Aabb) -> Self {
+       Self {
+           x: Interval::new_with_interval(&box0.x, &box1.x),
+           y: Interval::new_with_interval(&box0.y, &box1.y),
+           z: Interval::new_with_interval(&box0.z, &box1.z),
+       }
+   }
    ...
}
```
Listing 14: [aabb.rs] 从两个AABB输入构造AABB


### 创建物体列表的包围盒

```rust
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
+   bbox: Aabb,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
+           bbox: Aabb::default(),
        }
    }

    ...

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
+       self.bbox = Aabb::new_with_box(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    ...

+   fn bounding_box(&self) -> &Aabb {
+       &self.bbox
+   }
}
```
Listing 15: [hittable_list.rs] 具有包围盒的可击中物体列表


### BVH节点类

```rust
use std::rc::Rc;

use super::hittable::{
    Hittable,
    HitRecord,
};
use super::hittable_list::HittableList;
use super::ray::Ray;
use super::interval::Interval;
use super::aabb::Aabb;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &HittableList) -> Self {
        let len = list.objects.len();
        Self::new_with_hitables(&mut list.objects, 0, len)
    }

    pub fn new_with_hitables(src_objects: &Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        // TODO
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut ray_t = ray_t.clone();
        if !self.bbox.hit(r, &mut ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, &ray_t, rec);
        let ray_t = Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
        let hit_right = self.right.hit(r, &ray_t, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> &Aabb {
        &elf.bbox
    }
}
```
Listing 16: [bvh.rs] 包围体层次结构


### 分割BVH体积

```rust
impl BvhNode {
    ...
    pub fn new_with_hitables(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let axis = rtweekend::random_int(0, 2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let objects = src_objects;

        let object_span = end - start;

        if object_span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bbox: (*objects[start].bounding_box()).clone(),
            }
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
                Self {
                    left: objects[start].clone(),
                    right: objects[start + 1].clone(),
                    bbox: Aabb::new_with_box(
                        objects[start].bounding_box(),
                        objects[start + 1].bounding_box(),
                    ),
                }
            } else {
                Self {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
                    bbox: Aabb::new_with_box(
                        objects[start + 1].bounding_box(),
                        objects[start].bounding_box(),
                    ),
                }
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            let left = Rc::new(Self::new_with_hitables(objects, start, mid));
            let right = Rc::new(Self::new_with_hitables(objects, mid, end));
            let bbox = Aabb::new_with_box(left.bounding_box(), right.bounding_box());
            Self {
                left,
                right,
                bbox,
            }
        }
    }
    ...
}
```
Listing 17: [bvh.rs] 包围体层次结构节点

这里与原C++代码略作修改直接在if进行中返回Self，更符合Rust的使用习惯。

```rust
pub fn random_int(min: i32, max: i32) -> i32 {
    // Returns a random integer in [min,max].
    random_double_range(min as f64, (max + 1) as f64) as i32
}
```
Listing 18: [rtweekend.rs] 返回指定范围内的随机整数的函数


### 盒子比较函数

```rust
impl BvhNode {
    ...
    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: usize) -> std::cmp::Ordering {
        a.bounding_box().axis(axis_index).min.partial_cmp(&b.bounding_box().axis(axis_index).min).unwrap()
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}
```
Listing 19: [bvh.rs] BVH 比较函数，X 轴

由于Rust的比较函数和C++不同，是返回Ordering类型，这里相关比较排序代码修改较多。

```rust
fn main() {
    ...

+   let world = HittableList::new(Rc::new(BvhNode::new(&mut world)));

    // Camera
    ...
}
```
Listing 20: [main.rs] 随机球体，使用 BVH


### 另一个 BVH 优化

```rust
impl BvhNode {
  ...

  pub fn new_with_hitables(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
+   // 构建源对象范围的边界框。
+   let mut bbox = Aabb::default();
+   src_objects[start..end].iter().for_each(|obj| {
+       bbox = Aabb::new_with_box(&bbox, obj.bounding_box());
+   });

    ...
```
Listing 21: [bvh.rs] 构建 BVH 对象范围的边界框

这里循环是使用C++的for方式，还是Rust更习惯的iter方式，见仁见智。本文使用iter方式。

```rust
    pub fn new_with_hitables(src_objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
+       let axis = bbox.longest_axis();
        ...
        if object_span == 1 {
            Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
+               bbox,
            }
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == std::cmp::Ordering::Less {
                Self {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
+               bbox,
                }
            } else {
                Self {
                    left: objects[start + 1].clone(),
                    right: objects[start].clone(),
+                   bbox,
                }
            }
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + object_span / 2;
            let left = Rc::new(Self::new_with_hitables(objects, start, mid));
            let right = Rc::new(Self::new_with_hitables(objects, mid, end));
            Self {
                left,
                right,
+               bbox,
            }
        }
    }
```
Listing 22: [bvh.rs] 构建 BVH 对象范围的边界框

```rust
impl Aabb {
    ...

+   pub fn longest_axis(&self) -> usize {
+       // 返回边界框的最长轴的索引。
+       if self.x.size() > self.y.size() {
+           if self.x.size() > self.z.size() {
+               0
+           } else {
+               2
+           }
+       } else if self.y.size() > self.z.size() {
+           1
+       } else {
+           2
+       }
+   }

    ...
}

+pub const EMPTY: Aabb = Aabb {
+   x: interval::EMPTY,
+   y: interval::EMPTY,
+   z: interval::EMPTY,
+};
+pub const UNIVERSE: Aabb = Aabb {
+   x: interval::UNIVERSE,
+   y: interval::UNIVERSE,
+   z: interval::UNIVERSE,
+};
```
Listing 23: [aabb.rs] 新的 aabb 常量和 longest_axis() 函数



## 纹理映射

### 常量颜色纹理

```rust
use super::vec3::Point3;
use super::color::Color;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self {
            color_value,
        }
    }

    pub fn new_with_rgb(red: f64, green: f64, blue: f64) -> Self {
        Self {
            color_value: Color::new(red, green, blue),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}
```
Listing 24: [texture.rs] 纹理类

```rust
#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
+   pub u: f64,
+   pub v: f64,
    pub front_face: bool,
}
```
Listing 25: [hittable.rs] 将 u,v 坐标添加到 hit_record


### 实体纹理：棋盘格纹理

```rust
pub struct CheckerTexture {
    inv_scale: f64,
    even: Rc<dyn Texture>,
    odd: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_with_color(scale: f64, c1: Color, c2: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Rc::new(SolidColor::new(c1)),
            odd: Rc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let x_integer = (self.inv_scale * p.x()).floor() as i32;
        let y_integer = (self.inv_scale * p.y()).floor() as i32;
        let z_integer = (self.inv_scale * p.z()).floor() as i32;

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
```
Listing 26: [texture.rs] 棋盘格纹理

```rust
pub struct Lambertian {
+   pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Self {
+           albedo: Rc::new(SolidColor::new(a)),
        }
    }

+   pub fn new_with_texture(a: Rc<dyn Texture>) -> Self {
+       Self {
+           albedo: a,
+       }
+   }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // 捕捉退化的散射方向
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time());
+        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}
```
Listing 27: [material.rs] 带有纹理的朗伯材质

```rust
fn main() {
    ...
+   let checker: Rc<dyn Texture> = Rc::new(
+       CheckerTexture::new_with_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9))
+   );
    let ground_material: Rc<dyn Material> = Rc::new(
+       Lambertian::new_with_texture(Rc::clone(&checker))
    );
    world.add(Rc::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));
    ...
}
```
Listing 28: [main.rs] 使用棋盘格纹理

![图像 2：棋盘格地面上的球体](../../images/img-2.02-checker-ground.png)


### 渲染实体棋盘格纹理

```rust
fn random_spheres() {
    ...
    let ground_material: Rc<dyn Material> = Rc::new(
        Lambertian::new(color::Color::new(0.5, 0.5, 0.5))
    );
    world.add(Rc::new(
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)
    ));
    ...
}

fn main() {
    random_spheres();
}
```
Listing 29: [main.rs] 主函数调用选定的场景

```rust
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

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 2 {
        1 => random_spheres(),
        2 => two_spheres(),
        _ => (),
    }
}
```
Listing 30: [main.rs] 两个带纹理的球体

![图像 3：棋盘格球体](../../images/img-2.03-checker-spheres.png)


### 球体的纹理坐标

```rust
impl Sphere {
    ...

+    fn get_sphere_uv(p: Point3x) -> (f64, f64) {
+       // p: a given point on the sphere of radius one, centered at the origin.
+       // u: returned value [0,1] of angle around the Y axis from X=-1.
+       // v: returned value [0,1] of angle from Y=-1 to Y=+1.
+       //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
+       //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
+       //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
+
+       let theta = (-p.y()).acos();
+       let phi = (-p.z()).atan2(p.x()) + rtweekend::PI;
+
+       (phi / (2.0 * rtweekend::PI), theta / rtweekend::PI)
+   }
}
```
Listing 31: [sphere.rs] get_sphere_uv 函数

这里u, v的返回通过-> (f64, f64)实现更符合Rust习惯。

```rust
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        ...
        hit_record.p = r.at(hit_record.t);
        let outward_normal = (hit_record.p - self.center1) / self.radius;
        hit_record.set_face_normal(r, outward_normal);
+       (hit_record.u, hit_record.v) = Self::get_sphere_uv(outward_normal);
        hit_record.mat = Some(Rc::clone(&self.mat));

        true
    }
    ...
}
```
Listing 32: [sphere.rs] 从命中点获取球体的UV坐标


### 访问纹理图像数据

修改Cargo.toml文件，引入stb_image库。

```toml
[package]
name = "the_next_week"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0"
stb_image = "0.2"
```

```rust
use stb_image::image;

pub const BYTES_PER_PIXEL: usize = 3;
static MAGENTA: [u8; BYTES_PER_PIXEL] = [255, 0, 255];

#[derive(Default)]
pub struct RtwImage {
    data: Vec<u8>,
    image_width: usize,
    image_height: usize,
    bytes_per_scanline: usize,
}

impl RtwImage {
    pub fn new(image_filename: &str) -> Self {
        // 从指定的文件加载图像数据。如果定义了 RTW_IMAGES 环境变量，则仅在该目录中查找图像文件。
        // 如果未找到图像，则首先从当前目录，然后在 images/ 子目录中，然后在父级的 images/ 子目录中，
        // 依此类推，最多向上搜索六级。如果图像加载失败，width() 和 height() 将返回 0。

        let filename = image_filename;
        let imagedir = std::env::var("RTW_IMAGES").unwrap_or_else(|_| String::from("images"));

        let mut _self = Self::default();
        if !imagedir.is_empty() && _self.load(&format!("{}/{}", imagedir, filename)) {
            return _self;
        }
        if _self.load(filename) {
            return _self;
        }
        if _self.load(&format!("images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../../../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../../../../images/{}", filename)) {
            return _self;
        }
        if _self.load(&format!("../../../../../../images/{}", filename)) {
            return _self;
        }
        panic!("ERROR: Could not load image file \"{}\".", filename);
    }

    pub fn load(&mut self, filename: &str) -> bool {
        // 从给定的文件名加载图像数据。如果加载成功，返回 true。
        let load_result = image::load_with_depth(
            filename,
            BYTES_PER_PIXEL,
            false,
        );
        match load_result {
            image::LoadResult::Error(_) => {
                false
            },
            image::LoadResult::ImageU8(image) => {
                assert_eq!(image.depth, BYTES_PER_PIXEL);
                self.data = image.data;
                self.image_width = image.width;
                self.image_height = image.height;
                self.bytes_per_scanline = image.depth /* 原始每像素组件数的虚拟输出参数 */ * image.width;
                true
            },
            image::LoadResult::ImageF32(_) => {
                false
            },
        }
    }

    pub fn width(&self) -> usize {
        if self.data.is_empty() { 0 } else { self.image_width }
    }
    pub fn height(&self) -> usize {
        if self.data.is_empty() { 0 } else { self.image_height }
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8] {
        // 返回坐标为 x,y 的像素的三个字节的地址（如果没有数据，则返回品红色）。
        if self.data.is_empty() {
            &MAGENTA
        } else {
            let x = Self::clamp(x, 0, self.image_width);
            let y = Self::clamp(y, 0, self.image_height);

            &self.data[(y * self.bytes_per_scanline) + (x * BYTES_PER_PIXEL)..(y * self.bytes_per_scanline) + (x * BYTES_PER_PIXEL) + BYTES_PER_PIXEL]
        }
    }

    fn clamp(x: usize, low: usize, high: usize) -> usize {
        if x < low {
            return low;
        }
        if x < high {
            return x;
        }
        high - 1
    }
}
```
Listing 33: [rtw_stb_image.rs] rtw_image 辅助类

注意此处Rust的stb_image库的使用和C的版本区别较大（做了封装），因此代码与原文相比修改较大。
另原文此处代码也不是很优雅，本文Rust也仅仅是实现功能，也未作“美化”。

```rust
pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::new(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        // 如果没有纹理数据，则返回固定的青色作为调试辅助。
        if self.image.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // 将输入的纹理坐标限制在 [0,1] x [1,0] 范围内
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = (u * self.image.width() as f64) as usize;
        let j = (v * self.image.height() as f64) as usize;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            color_scale * pixel[0] as f64,
            color_scale * pixel[1] as f64,
            color_scale * pixel[2] as f64,
        )
    }
}
```
Listing 34: [texture.rs] 图像纹理类

此处Rust的f64已有clamp函数，因此没有使用原文的interval中的clamp。


### 渲染图像纹理

```rust
fn earth() {
    let earth_texture: Rc<dyn Texture> = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface: Rc<dyn Material> = Rc::new(Lambertian::new_with_texture(Rc::clone(&earth_texture)));
    let globe = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut cam = Camera::default();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&HittableList::new(globe));
}

fn main() {
    match 3 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        _ => (),
    }
}
```
Listing 35: [main.rs] 使用 stb_image 加载图像

![图像 5：贴有地球贴图的球体](../../images/img-2.05-earth-sphere.png)


## Perlin噪声

```rust
use super::vec3::Point3;
use super::rtweekend;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranfloat = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranfloat.push(rtweekend::random_double());
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();
        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x()) as i32) & 255;
        let j = ((4.0 * p.y()) as i32) & 255;
        let k = ((4.0 * p.z()) as i32) & 255;

        self.ranfloat[
            self.perm_x[i] as usize ^
            self.perm_y[j] as usize ^
            self.perm_z[k] as usize
        ]
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            p.push(i as i32);
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }

    fn permute(p: &mut [i32], n: usize) {
        for i in (0..n).rev() {
            let target = rtweekend::random_int(0, i as i32);
            p.swap(i, target as usize);
        }
    }
}
```
Listing 36: [perlin.rs] Perlin纹理类和函数

```rust
#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
```
Listing 37: [texture.rs] 噪声纹理

```rust
fn two_perlin_spheres() {
    let mut world = HittableList::default();

    let pertext: Rc<dyn Texture> = Rc::new(NoiseTexture::default());
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

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
    match 4 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        _ => (),
    }
}
```
Listing 38: [main.rs] 带有两个Perlin纹理球体的场景

![图像 9：哈希随机纹理](../../images/img-2.09-hash-random.png)


### 平滑结果

```rust
impl Perlin {
+   pub fn noise(&self, p: Point3) -> f64 {
+       let u = p.x() - p.x().floor();
+       let v = p.y() - p.y().floor();
+       let w = p.z() - p.z().floor();
+
+       let i = p.x().floor() as i32;
+       let j = p.y().floor() as i32;
+       let k = p.z().floor() as i32;
+       let mut c = [[[0.0; 2]; 2]; 2];
+
+       (0..2).for_each(|di| {
+           (0..2).for_each(|dj| {
+               (0..2).for_each(|dk| {
+                   c[di][dj][dk] = self.ranfloat[
+                       self.perm_x[((i + di as i32) & 255) as usize] as usize ^
+                       self.perm_y[((j + dj as i32) & 255) as usize] as usize ^
+                       self.perm_z[((k + dk as i32) & 255) as usize] as usize
+                   ];
+               })
+           })
+       });
+
+       Self::trilinear_interp(&c, u, v, w)
+   }
+   ...
+   fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
+       let mut accum = 0.0;
+       (0..2).for_each(|i| {
+           (0..2).for_each(|j| {
+               (0..2).for_each(|k| {
+                   accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u)) *
+                           (j as f64 * v + (1 - j) as f64 * (1.0 - v)) *
+                           (k as f64 * w + (1 - k) as f64 * (1.0 - w)) * c[i][j][k];
+               })
+           })
+       });
+       accum
+   }
}
```
Listing 39: [perlin.rs] 带有三线性插值的Perlin纹理

![图像 10：带有三线性插值的Perlin纹理](../../images/img-2.10-perlin-trilerp.png)


### 改进的Hermite平滑

```rust
impl Perlin {
    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
+       let u = u * u * (3.0 - 2.0 * u);
+       let v = v * v * (3.0 - 2.0 * v);
+       let w = w * w * (3.0 - 2.0 * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        ...
    }
}
```
Listing 40: [perlin.rs] 带有Hermite平滑的Perlin纹理

![图像 11：Perlin纹理，三线性插值，平滑](../../images/img-2.11-perlin-trilerp-smooth.png)


### 调整频率

```rust
pub struct NoiseTexture {
    noise: Perlin,
+   scale: f64,
}

+impl Default for NoiseTexture {
+   fn default() -> Self {
+       Self {
+           noise: Perlin::default(),
+           scale: 1.0,
+       }
+   }
+}

+impl NoiseTexture {
+   pub fn new(scale: f64) -> Self {
+       Self {
+           noise: Perlin::default(),
+           scale,
+       }
+   }
+}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
+       Color::new(1.0, 1.0, 1.0) * self.noise.noise(self.scale * p)
    }
}
```
Listing 41: [texture.rs] 带有缩放的平滑Perlin纹理

```rust
fn two_perlin_spheres() {
    let mut world = HittableList::default();

+   let pertext: Rc<dyn Texture> = Rc::new(NoiseTexture::new(4.0));
    ...
}
```
Listing 42: [main.rs] 带有缩放的Perlin纹理球体

![图像 12：Perlin纹理，更高的频率](../../images/img-2.12-perlin-hifreq.png)


### 在格点上使用随机向量

```rust
impl Perlin {
    pub fn noise(&self, p: Point3) -> f64 {
+-      let u = p.x() - p.x().floor();
+-      let v = p.y() - p.y().floor();
+-      let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
+       let mut c = [[[Vec3::default(); 2]; 2]; 2];

        (0..2).for_each(|di| {
            (0..2).for_each(|dj| {
                (0..2).for_each(|dk| {
+                   c[di][dj][dk] = self.ranvec[
                        self.perm_x[((i + di as i32) & 255) as usize] as usize ^
                        self.perm_y[((j + dj as i32) & 255) as usize] as usize ^
                        self.perm_z[((k + dk as i32) & 255) as usize] as usize
                    ];
                })
            })
        });

        Self::trilinear_interp(&c, u, v, w)
    }
  ...
}
```
Listing 44: [perlin.rs] 带有新的noise()方法的Perlin类

```rust
impl Perlin {
    ...
+   fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
+       let uu = u * u * (3.0 - 2.0 * u);
+       let vv = v * v * (3.0 - 2.0 * v);
+       let ww = w * w * (3.0 - 2.0 * w);
+       let mut accum = 0.0;

+       (0..2).for_each(|i| {
+           (0..2).for_each(|j| {
+               (0..2).for_each(|k| {
+                   let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
+                   accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
+                          * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
+                          * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
+                          * vec3::dot(c[i][j][k], weight_v);
+               })
+           })
+       });
+       accum
+   }
}
```
Listing 45: [perlin.rs] 迄今为止的Perlin插值函数

```rust
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
+       Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(self.scale * p))
    }
}
```
Listing 46: [texture.rs] 带有缩放的平滑Perlin纹理

![图像 13：Perlin纹理，偏离整数值](../../images/img-2.13-perlin-shift.png)


### 介绍湍流

```rust
impl Perlin {
    ...
+   pub fn turb(&self, p: Point3, depth: i32) -> f64 {
+       let mut accum = 0.0;
+       let mut temp_p = p;
+       let mut weight = 1.0;

+       for _ in 0..depth {
+           accum += weight * self.noise(temp_p);
+           weight *= 0.5;
+           temp_p *= 2.0;
+       }

+       accum.abs()
+   }
    ...
}
```
Listing 47: [perlin.rs] 湍流函数

```rust
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        let s = self.scale * p;
        Color::new(1.0, 1.0, 1.0) * self.noise.turb(s, 7)
    }
}
```
Listing 48: [texture.rs] 具有湍流的噪声纹理

![图像 14：具有湍流的 Perlin 纹理](../../images/img-2.14-perlin-turb.png)


### 调整相位

```rust
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        let s = self.scale * p;
+       Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (s.z() + 10.0 * self.noise.turb(s, 7)).sin())
    }
}
```
Listing 49: [texture.rs] 具有大理石纹理的噪声纹理

![图像 15：Perlin 噪声，大理石纹理](../../images/img-2.15-perlin-marble.png)



## 四边形

### 定义四边形

```rust
impl Aabb {
    ...
    pub fn pad(&self) -> Self {
        // 返回一个没有边小于某个 delta 的 AABB，如果需要则填充。
        let delta = 0.0001;
        let new_x = if self.x.size() < delta {
            x.expand(delta)
        } else {
            self.x.clone()
        };
        let new_y = if self.y.size() < delta {
            y.expand(delta)
        } else {
            self.y.clone()
        };
        let new_z = if self.z.size() < delta {
            z.expand(delta)
        } else {
            self.z.clone()
        };
        Self {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }
    ...
}
```
Listing 50: [aabb.rs] 新的 aabb::pad() 方法

```rust
use std::rc::Rc;

use super::vec3::{
    Vec3,
    Point3,
};
use super::material::Material;
use super::aabb::Aabb;
use super::hittable::{
    HitRecord,
    Hittable,
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Quad {
  pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        Self {
            q,
            u,
            v,
            mat,
            bbox: Aabb::new_with_point(
                &q, &(q + u + v)
            ),
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &crate::ray::Ray, ray_t: &crate::interval::Interval, hit_record: &mut HitRecord) -> bool {
        // TODO
        false
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
```
Listing 51: [quad.rs] 二维四边形（平行四边形）类

考虑到Rust的习惯，这里并没有实现C++版本的set_bounding_box，而是直接在new里计算了bbox。


### 寻找包含给定四边形的平面

```rust
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
+   normal: Vec3,
+   d: f64,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Quad {
  pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
+       let n = vec3::cross(u, v);
+       let normal = vec3::unit_vector(n);
        Self {
            q,
            u,
            v,
+           normal,
+           d: vec3::dot(normal, q),
            mat,
            bbox: Aabb::new_with_point(
                &q, &(q + u + v)
            ),
        }
    }
}
```
Listing 52: [quad.rs] 缓存平面值

```rust
impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
+       let denom = vec3::dot(self.normal, r.direction());

+       // 如果射线与平面平行，则没有相交。
+       if denom.abs() < 1e-8 {
+           return false;
+       }

+       // 如果相交点参数 t 在射线区间之外，则返回 false。
+       let t = (self.d - vec3::dot(self.normal, r.origin())) / denom;
+       if !ray_t.contains(t) {
+           return false;
+       }

+       let intersection = r.at(t);

+       rec.t = t;
+       rec.p = intersection;
+       rec.mat = Some(Rc::clone(&self.mat));
+       rec.set_face_normal(r, self.normal);

+       true
    }
    ...
}
```
Listing 53: [quad.rs] 用于无限平面的 hit() 方法


### 平面上的点的定位

```rust
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
+   w: Vec3,
    normal: Vec3,
    d: f64,
    mat: Rc<dyn Material>,
    bbox: Aabb,
}

impl Quad {
  pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Self {
        let n = vec3::cross(u, v);
        let normal = vec3::unit_vector(n);
        Self {
            q,
            u,
            v,
+           w: n / vec3::dot(n, n),
            normal,
            d: vec3::dot(normal, q),
            mat,
            bbox: Aabb::new_with_point(
                &q, &(q + u + v)
            ),
        }
    }
}
```
Listing 54: [quad.rs] 缓存四边形的 w 值


### 内部测试使用UV坐标的交点

```rust
impl Quad {
    ...

+   pub fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
+       // 给定平面坐标中的击中点，如果它在基元之外，则返回false，否则设置击中记录的UV坐标并返回true。
+       if !(0.0..=1.0).contains(&a) || !(0.0..=1.0).contains(&b) {
+           return false;
+       }
+
+       rec.u = a;
+       rec.v = b;
+
+       true
+   }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let denom = vec3::dot(self.normal, r.direction());

        // 如果射线与平面平行，则没有相交。
        if denom.abs() < 1e-8 {
            return false;
        }

        // 如果相交点参数 t 在射线区间之外，则返回 false。
        let t = (self.d - vec3::dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

+       // 使用平面坐标确定击中点是否在平面形状内部。
+       let intersection = r.at(t);
+       let planar_hitpt_vector = intersection - self.q;
+       let alpha = vec3::dot(self.w, vec3::cross(planar_hitpt_vector, self.v));
+       let beta = vec3::dot(self.w, vec3::cross(self.u, planar_hitpt_vector));
+
+       if !self.is_interior(alpha, beta, rec) {
+           return false;
+       }
+
+       // 光线击中了2D形状；设置剩余的击中记录并返回true。
        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Rc::clone(&self.mat));
        rec.set_face_normal(r, self.normal);

        true
    }

      ...
}
```
Listing 55: [quad.rs] 最终的quad类

```rust
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

    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
  match 5 {
    1 => random_spheres(),
    2 => two_spheres(),
    3 => earth(),
    4 => two_perlin_spheres(),
    5 => quads(),
    _ => (),
  }
}
```
Listing 56: [main.rs] 包含四边形的新场景

![图 16：四边形](../../images/img-2.16-quads.png)



## 光源

### 发光材料

```rust
pub struct DiffuseLight {
    pub emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(a: Rc<dyn Texture>) -> Self {
        Self {
            emit: a,
        }
    }

    pub fn new_with_color(c: Color) -> Self {
        Self {
            emit: Rc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: vec3::Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
```
Listing 57: [material.rs] A diffuse light class

```rust
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
+   fn emitted(&self, _u: f64, _v: f64, _p: vec3::Point3) -> Color {
+       Color::new(0.0, 0.0, 0.0)
+   }
}
```
Listing 58: [material.rs] New emitted function in class material


### 向光线颜色函数添加背景色

```rust
pub struct Camera {
    pub aspect_ratio: f64,  // Ratio of image width over height
    pub image_width: i32,   // Rendered image width in pixel count
    pub samples_per_pixel: usize, // Count of random samples for each pixel
    pub max_depth: i32,     // Maximum number of ray bounces into scene
+   pub background: Color,  // Background color for rays that miss
    ...
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
+           background: Color::default(),
            ...
        }
    }
}

impl Camera {
    ...

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        // 如果我们超过了光线反弹限制，就不再收集光线。
        if depth <= 0 {
            return Color::default();
        }

+       // 如果光线没有击中了世界中的任何东西，则返回背景颜色。
+       if !world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
+           return self.background;
+       }
+
+       if let Some(mat) = rec.mat.clone() {
+           let mut scattered = Ray::default();
+           let mut attenuation = Color::default();
+           let color_from_emission = mat.emitted(rec.u, rec.v, rec.p);
+           if !mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
+               return color_from_emission;
+           }
+
+           let color_from_scatter = attenuation * self.ray_color(&scattered,  depth - 1, world);
+
+           color_from_emission + color_from_scatter
+       } else {
+           Color::default()
+       }
+   }
}
```
Listing 59: [camera.rs] ray_color function with background and emitting materials

```rust
fn random_spheres() {
    ...
    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
+   cam.background = Color::new(0.7, 0.8, 1.0);
    ...
}

fn two_spheres() {
    ...
    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
+   cam.background = Color::new(0.7, 0.8, 1.0);
    ...
}

fn earth() {
    ...
    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
+   cam.background = Color::new(0.7, 0.8, 1.0);
    ...
}

fn two_perlin_spheres() {
    ...
    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
+   cam.background = Color::new(0.7, 0.8, 1.0);
    ...
}

fn quads() {
    ...
    // Camera
    let mut cam = Camera::default();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 10;
+   cam.background = Color::new(0.7, 0.8, 1.0);
    ...
}
```
Listing 60: [main.rs] Specifying new background color


### 将对象转化为光源

```rust
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

    let difflight = Rc::new(DiffuseLight::new_with_color(Color::new(4.0, 4.0, 4.0)));
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

fn main() {
    match 6 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        _ => (),
    }
}
```
Listing 61: [main.rs] 一个简单的矩形光源

![Image 17: 场景中的矩形光源](../../images/img-2.17-rect-light.png)

```rust
fn simple_light() {
    ...
+   let difflight: Rc<dyn Material> = Rc::new(DiffuseLight::new_with_color(Color::new(4.0, 4.0, 4.0)));
+   world.add(Rc::new(
+       Sphere::new(
+           Point3::new(0.0, 7.0, 0.0),
+           2.0,
+           Rc::clone(&difflight)
+       )
+   ));
    world.add(Rc::new(
        Quad::new(
            Point3::new(3.0, 1.0, -2.0),
            vec3::Vec3::new(2.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 2.0, 0.0),
            difflight
        )
    ));
    ...
}
```
Listing 62: [main.rs] 一个简单的矩形光源加上发光的球体

![Image 18: 场景中的矩形和球体光源](../../images/img-2.18-rect-sphere-light.png)


### 创建一个空的“康奈尔盒”

```rust
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
}
```
Listing 63: [main.rs] 康奈尔盒场景，空的

![图 19: 空的康奈尔盒](../../images/img-2.19-cornell-empty.png)



## 实例

```rust
pub fn make_box(a: Point3, b: Point3, mat: Rc<dyn Material>) -> Rc<HittableList> {
    // 返回一个包含两个对角顶点a和b的3D盒子（六个面）。

    let mut sides = HittableList::default();

    // 构造两个对角顶点，具有最小和最大的坐标。
    let min = Point3::new(
        a.x().min(b.x()),
        a.y().min(b.y()),
        a.z().min(b.z()),
    );
    let max = Point3::new(
        a.x().max(b.x()),
        a.y().max(b.y()),
        a.z().max(b.z()),
    );

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Rc::new(
        Quad::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, Rc::clone(&mat))
    ));
    sides.add(Rc::new(
        Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz, dy, Rc::clone(&mat))
    ));
    sides.add(Rc::new(
        Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx, dy, Rc::clone(&mat))
    ));
    sides.add(Rc::new(
        Quad::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, Rc::clone(&mat))
    ));
    sides.add(Rc::new(
        Quad::new(Point3::new(min.x(), max.y(), max.z()), dx, -dz, Rc::clone(&mat))
    ));
    sides.add(Rc::new(
        Quad::new(Point3::new(min.x(), min.y(), min.z()), dx, dz, Rc::clone(&mat))
    ));

    Rc::new(sides)
}
```
Listing 64: [quad.rs] 一个盒子对象

```rust
fn cornell_box() {
    ...
    world.add(make_box(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        Rc::clone(&white)
    ));
    world.add(make_box(
        Point3::new(265.0, 0.0, 295.0),
        Point3::new(430.0, 330.0, 460.0),
        Rc::clone(&white)
    ));
    ...
}
```
Listing 65: [main.rs] 添加盒子对象

![图 20: 包含两个方块的康奈尔盒](../../images/img-2.20-cornell-blocks.png)


### 实例平移

```rust
pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // 将光线向后移动偏移量
        let offset_r = Ray::new_with_time(r.origin() - self.offset, r.direction(), r.time());

        // 确定在偏移光线上是否存在交点（如果有，确定在哪里）
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // 将交点向前移动偏移量
        rec.p += self.offset;

        true
    }
}
```
Listing 66: [hittable.rs] 可击中的平移击中函数

```rust
+impl Translate {
+   pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
+       let bbox = object.bounding_box() + offset;
+       Self {
+           object,
+           offset,
+           bbox,
+       }
+   }
+}

impl Hittable for Translate {
    ...

+   fn bounding_box(&self) -> &Aabb {
+      &self.bbox
+   }
}
```
Listing 67: [hittable.rs] 可击中的平移类

```rust
impl std::ops::Add<Vec3> for &Aabb {
    type Output = Aabb;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb {
            x: &self.x + rhs.x(),
            y: &self.y + rhs.y(),
            z: &self.z + rhs.z(),
        }
    }
}

impl std::ops::Add<&Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, rhs: &Aabb) -> Self::Output {
        Aabb {
            x: self.x() + &rhs.x,
            y: self.y() + &rhs.y,
            z: self.z() + &rhs.z,
        }
    }
}
```
Listing 68: [aabb.rs] aabb + offset运算符

```rust
impl std::ops::Add<f64> for &Interval {
    type Output = Interval;

    fn add(self, rhs: f64) -> Self::Output {
        Interval {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}

impl std::ops::Add<&Interval> for f64 {
    type Output = Interval;

    fn add(self, rhs: &Interval) -> Self::Output {
        Interval {
            min: self + rhs.min,
            max: self + rhs.max,
        }
    }
}
```
Listing 69: [interval.rs] interval + displacement运算符


### 实例旋转

```rust
pub struct RotateY {
  object: Rc<dyn Hittable>,
  sin_theta: f64,
  cos_theta: f64,
  bbox: Aabb,
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // 将光线从世界空间变换到对象空间
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new_with_time(origin, direction, r.time());

        // 在对象空间中确定是否存在交点（如果有，确定在哪里）
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // 将交点从对象空间变换到世界空间
        let mut p = rec.p;
        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        // 将法线从对象空间变换到世界空间
        let mut normal = rec.normal;
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        true
    }
}
```
Listing 70: [hittable.rs] 可击中的Y旋转击中函数

```rust
impl RotateY {
+   pub fn new(p: Rc<dyn Hittable>, angle: f64) -> Self {
+       let radians = angle.to_radians();
+       let sin_theta = radians.sin();
+       let cos_theta = radians.cos();
+       let bbox = p.bounding_box();
+
+       let mut min = Point3::new(rtweekend::INFINITY, rtweekend::INFINITY, rtweekend::INFINITY);
+       let mut max = Point3::new(-rtweekend::INFINITY, -rtweekend::INFINITY, -rtweekend::INFINITY);
+
+       (0..2).for_each(|i| {
+           (0..2).for_each(|j| {
+               (0..2).for_each(|k| {
+                   let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
+                   let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
+                   let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;
+
+                   let newx = cos_theta * x + sin_theta * z;
+                   let newz = -sin_theta * x + cos_theta * z;
+
+                   let tester = Vec3::new(newx, y, newz);
+
+                   (0..3).for_each(|c| {
+                       min[c] = min[c].min(tester[c]);
+                       max[c] = max[c].max(tester[c]);
+                   })
+               })
+           })
+       });
+
+       let bbox = Aabb::new_with_point(&min, &max);
+       Self {
+           object: p,
+           sin_theta,
+           cos_theta,
+           bbox,
+       }
+   }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        ...
    }

+   fn bounding_box(&self) -> &Aabb {
+       &self.bbox
+   }
}
```
Listing 71: [hittable.rs] Hittable rotate-Y class

```rust
fn cornell_box() {
    ...
+   let box1 = make_box(
+       Point3::new(0.0, 0.0, 0.0),
+       Vec3::new(165.0, 330.0, 165.0),
+       Rc::clone(&white)
+   );
+   let box1 = Rc::new(RotateY::new(box1, 15.0));
+   let box1 = Rc::new(Translate::new(box1, vec3::Vec3::new(265.0, 0.0, 295.0)));
+   world.add(box1);
+
+   let box2 = make_box(
+       Point3::new(0.0, 0.0, 0.0),
+       Vec3::new(165.0, 165.0, 165.0),
+       Rc::clone(&white)
+   );
=   let box2 = Rc::new(RotateY::new(box2, -18.0));
+   let box2 = Rc::new(Translate::new(box2, vec3::Vec3::new(130.0, 0.0, 65.0)));
+   world.add(box2);
    ...
}
```
Listing 72: [main.rs] 带有Y轴旋转盒子的康奈尔场景

![图 21: 标准的康奈尔盒场景](../../images/img-2.21-cornell-standard.png)



## 体积

### 恒定密度介质

```rust
use std::rc::Rc;

use super::rtweekend;
use super::hittable::{
    Hittable,
    HitRecord,
};
use super::material::{
  Material,
  Isotropic,
};
use super::ray::Ray;
use super::vec3::Vec3;
use super::color::Color;
use super::aabb::Aabb;
use super::texture::Texture;
use super::interval::{
  self,
  Interval,
};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(b: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::new(a)),
        }
    }
    pub fn new_with_color(b: Rc<dyn Hittable>, d: f64, c: Color) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Rc::new(Isotropic::new_with_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && rtweekend::random_double() < 0.00001;

        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, &interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, &Interval::new(rec1.t + 0.0001, rtweekend::INFINITY), &mut rec2) {
            return false;
        }

        if debugging {
            eprintln!("\nray_tmin={} ray_tmax={}", rec1.t, rec2.t);
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rtweekend::random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            eprintln!("hit_distance = {}", hit_distance);
            eprintln!("rec.t = {}", rec.t);
            eprintln!("rec.p = {}", rec.p);
        }

        rec.normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.mat = Some(Rc::clone(&self.phase_function));

        true
    }

    fn bounding_box(&self) -> &Aabb {
        self.boundary.bounding_box()
    }
}
```
Listing 73: [constant_medium.rs] 恒定介质类

```rust
pub struct Isotropic {
    pub albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(a: Rc<dyn Texture>) -> Self {
        Self {
            albedo: a,
        }
    }

    pub fn new_with_color(c: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *scattered = Ray::new_with_time(rec.p, vec3::random_unit_vector(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, rec.p);
        true
    }
}
```
Listing 74: [material.rs] 各向同性类

```rust
fn cornell_smoke() {
    let mut world = HittableList::default();

    let red: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light: Rc<dyn Material> = Rc::new(DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0)));

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
            Point3::new(113.0, 554.0, 127.0),
            vec3::Vec3::new(330.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 305.0),
            light
        )
    ));
    world.add(Rc::new(
        Quad::new(
            Point3::new(0.0, 555.0, 0.0),
            vec3::Vec3::new(555.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 555.0),
            Rc::clone(&white)
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
            Point3::new(0.0, 0.0, 555.0),
            vec3::Vec3::new(555.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 555.0),
            Rc::clone(&white)
        )
    ));

    let box1 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 330.0, 165.0),
        Rc::clone(&white)
    );
    let box1 = Rc::new(RotateY::new(box1, 15.0));
    let box1 = Rc::new(Translate::new(box1, vec3::Vec3::new(265.0, 0.0, 295.0)));

    let box2 = make_box(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(165.0, 165.0, 165.0),
        Rc::clone(&white)
    );
    let box2 = Rc::new(RotateY::new(box2, -18.0));
    let box2 = Rc::new(Translate::new(box2, vec3::Vec3::new(130.0, 0.0, 65.0)));

    world.add(Rc::new(
        ConstantMedium::new_with_color(box1, 0.01, Color::new(0.0, 0.0, 0.0))
    ));
    world.add(Rc::new(
        ConstantMedium::new_with_color(box2, 0.01, Color::new(1.0, 1.0, 1.0))
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
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
    match 8 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        _ => (),
    }
}
```
Listing 75: [main.cc] 带有烟雾的康奈尔盒

![图 22: 带有烟雾块的康奈尔盒](../../images/img-2.22-cornell-smoke.png)



## 一个测试所有新特性的场

```rust
fn final_scene(image_width: usize, samples_per_pixel: usize, max_depth: usize) {
    let mut boxes1 = HittableList::default();
    let ground: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    (0..boxes_per_side).for_each(|i| {
        (0..boxes_per_side).for_each(|j| {
        let w = 100.0;
        let x0 = -1000.0 + i as f64 * w;
        let z0 = -1000.0 + j as f64 * w;
        let y0 = 0.0;
        let x1 = x0 + w;
        let y1 = rtweekend::random_double_range(1.0, 101.0);
        let z1 = z0 + w;

        boxes1.add(
            make_box(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Rc::clone(&ground)
            )
        );
        });
    });

    let mut world = HittableList::default();

    world.add(Rc::new(BvhNode::new(&mut boxes1)));

    let light: Rc<dyn Material> = Rc::new(DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Rc::new(
        Quad::new(
            Point3::new(123.0, 554.0, 147.0),
            vec3::Vec3::new(412.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 0.0, 412.0),
            light
        )
    ));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + vec3::Vec3::new(30.0, 0.0, 0.0);
    let sphere_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Rc::new(
        Sphere::new_with_center2(center1, center2, 50.0, sphere_material)
    ));

    world.add(Rc::new(
        Sphere::new(
            Point3::new(260.0, 150.0, 45.0),
            50.0,
            Rc::new(Dielectric::new(1.5))
        )
    ));
    world.add(Rc::new(
        Sphere::new(
            Point3::new(0.0, 150.0, 145.0),
            50.0,
            Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0))
        )
    ));

    let boundary: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Rc::new(Dielectric::new(1.5))));
    world.add(Rc::clone(&boundary));
    world.add(Rc::new(ConstantMedium::new_with_color(
        Rc::clone(&boundary),
        0.2,
        Color::new(0.2, 0.4, 0.9)
    )));
    let boundary: Rc<dyn Hittable> = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Rc::new(Dielectric::new(1.5))));
    world.add(Rc::new(ConstantMedium::new_with_color(
        Rc::clone(&boundary),
        0.0001,
        Color::new(1.0, 1.0, 1.0)
    )));

    let emat: Rc<dyn Material> = Rc::new(Lambertian::new_with_texture(Rc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Rc::new(
        Sphere::new(
            Point3::new(400.0, 200.0, 400.0),
            100.0,
            emat
        )
    ));
    let pertext = Rc::new(NoiseTexture::new(0.1));
    world.add(Rc::new(
        Sphere::new(
            Point3::new(220.0, 280.0, 300.0),
            80.0,
            Rc::new(Lambertian::new_with_texture(pertext))
        )
    ));

    let mut boxes2 = HittableList::default();
    let white: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    (0..ns).for_each(|_| {
        boxes2.add(
            Rc::new(Sphere::new(
                Point3::random_range(0.0, 165.0),
                10.0,
                Rc::clone(&white)
            ))
        );
    });

    world.add(Rc::new(
        Translate::new(
            Rc::new(RotateY::new(
                Rc::new(BvhNode::new(&mut boxes2)),
                15.0
            )),
            vec3::Vec3::new(-100.0, 270.0, 395.0)
        )
    ));

    let mut cam = Camera::default();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth = max_depth;
    cam.background = Color::default();

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = vec3::Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    cam.render(&world);
}

fn main() {
  match 9 {
        1 => random_spheres(),
        2 => two_spheres(),
        3 => earth(),
        4 => two_perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(200, 200, 10),
        _ => (),
  }
}
```
Listing 76: [main.rs] 最终场景

![图 23: 最终场景](../../images/img-2.23-book2-final.jpg)
