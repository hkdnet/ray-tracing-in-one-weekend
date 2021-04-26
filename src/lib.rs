use rand::random;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;
pub enum Point3Index {
    X,
    Y,
    Z,
}
pub enum ColorIndex {
    R,
    G,
    B,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    fn random(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        )
    }
}
fn random_double(min: f64, max: f64) -> f64 {
    let scale = max - min;
    let r = random::<f64>();
    min + r * scale
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}
impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

use auto_ops::impl_op_ex;

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });

impl std::ops::Index<Point3Index> for Point3 {
    type Output = f64;

    fn index(&self, idx: Point3Index) -> &Self::Output {
        match idx {
            Point3Index::X => &self.x,
            Point3Index::Y => &self.y,
            Point3Index::Z => &self.z,
        }
    }
}
impl std::ops::Index<ColorIndex> for Color {
    type Output = f64;

    fn index(&self, idx: ColorIndex) -> &Self::Output {
        match idx {
            ColorIndex::R => &self.x,
            ColorIndex::G => &self.y,
            ColorIndex::B => &self.z,
        }
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl std::ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl std::ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.x) as i32,
            (255.999 * self.y) as i32,
            (255.999 * self.z) as i32
        )
    }
}

#[cfg(test)]
mod test_vec3 {
    use super::Point3Index;
    use super::Vec3;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(&v1 + &v2, Vec3::new(5., 7., 9.));
        assert_eq!(&v2 + &v1, Vec3::new(5., 7., 9.));
        let v3 = Vec3::new(1., 1., 1.);
        assert_eq!(&v1 + &v3, Vec3::new(2., 3., 4.));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(4., 5., 6.);
        let v2 = Vec3::new(1., 2., 3.);
        assert_eq!(&v1 - &v2, Vec3::new(3., 3., 3.));
        assert_eq!(&v2 - &v1, Vec3::new(-3., -3., -3.));
        let v3 = Vec3::new(1., 1., 1.);
        assert_eq!(&v1 - &v3, Vec3::new(3., 4., 5.));
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1., 2., 3.);
        assert_eq!(&v1 * 2., Vec3::new(2., 4., 6.));

        let v2 = Vec3::new(4., 5., 6.);
        assert_eq!(&v1 * &v2, Vec3::new(4., 10., 18.));
    }

    #[test]
    fn test_index() {
        let v = Vec3::new(1., 2., 3.);
        assert!((v[Point3Index::X] - 1.).abs() < f64::EPSILON);
        assert!((v[Point3Index::Y] - 2.).abs() < f64::EPSILON);
        assert!((v[Point3Index::Z] - 3.).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(1., 2., 3.);
        v1 += &v2;
        assert_eq!(v1, Vec3::new(2., 4., 6.));
        assert_eq!(v2, Vec3::new(1., 2., 3.));
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3::new(1., 2., 3.);
        v *= 2.;
        assert_eq!(v, Vec3::new(2., 4., 6.));
    }
    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(2., 4., 6.);
        v /= 2.;
        assert_eq!(v, Vec3::new(1., 2., 3.));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(2., 4., 6.);
        assert!((v.length() - 56f64.sqrt()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_display() {
        let v = Vec3::new(0.1, 0.2, 0.3);
        assert_eq!(format!("{}", v), "25 51 76");
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(1., 2., 3.);
        let ans = v1.dot(&v2);
        assert!((ans - 14.).abs() < f64::EPSILON);
    }
}

pub struct Ray {
    orig: Box<Point3>,
    dir: Box<Vec3>,
}

impl Ray {
    pub fn new(orig: Box<Point3>, dir: Box<Vec3>) -> Self {
        Ray { orig, dir }
    }
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        let dir = self.dir.as_ref();
        let delta = dir * t;
        let orig = self.orig.as_ref();
        orig + delta
    }
}

fn random_in_unit_sphere() -> Point3 {
    loop {
        let p = Vec3::random(-1., 1.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}

// Looks like that this is a toy function.
// We can't distinguish the color of ray from the ray itself.
// For now, the color is deternined by y when it doesn't hit anything.
// It's a blended color between white(0, 0, 0) at the bottom and light blue(0.5, 0.7, 1.0) at the top.
pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(rec) = world.hit(ray, 0., f64::INFINITY) {
        let target = &rec.p + rec.normal + random_in_unit_sphere();
        let dir = target - &rec.p;
        let boxed_p = Box::new(rec.p);
        let c = ray_color(&Ray::new(boxed_p, Box::new(dir)), world, depth - 1);
        return c * 0.5;
    }

    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    let unit = Color::new(1., 1., 1.);
    let c1 = &unit * (1. - t);
    let color_base = Color::new(0.5, 0.7, 1.0);
    let c2 = &color_base * t;
    &c1 + &c2
}

pub fn hit_sphere(center: &Vec3, rad: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - rad * rad;

    let discriminant = half_b.powf(2.) - a * c;
    if discriminant < 0. {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}

#[cfg(test)]
mod test_ray {
    use super::*;

    #[test]
    fn test_at() {
        let p = Point3::new(0., 0., 0.);
        let d = Vec3::new(1., 2., 3.);
        let r = Ray::new(Box::new(p), Box::new(d));

        let p1 = r.at(1.);
        assert_eq!(p1, Vec3::new(1., 2., 3.));
        let p2 = r.at(2.);
        assert_eq!(p2, Vec3::new(2., 4., 6.));
    }
}

pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(t: f64, p: Point3, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            t,
            p,
            normal,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b.powf(2.) - a * c;

        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Search nearer root
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                // we don't care that hit
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (&p - &self.center) / self.radius;
        Some(HitRecord::new(root, p, ray, outward_normal))
    }
}

#[derive(Default)]
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        HittableList { list: vec![] }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.list.push(hittable);
    }

    pub fn clear(&mut self) {
        self.list.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest = t_max;
        let mut ret = None;
        for boxed in self.list.iter() {
            let hittable = boxed.as_ref();
            if let Some(rec) = hittable.hit(ray, t_min, closest) {
                closest = rec.t;
                ret = Some(rec);
            }
        }
        ret
    }
}

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point3::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            &origin - &horizontal / 2. - &vertical / 2. - Vec3::new(0., 0., focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let dir =
            &self.lower_left_corner + &self.horizontal * u + &self.vertical * v - &self.origin;
        let cloned = self.origin.clone();
        Ray::new(Box::new(cloned), Box::new(dir))
    }
}
