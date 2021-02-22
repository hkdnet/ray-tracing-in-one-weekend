#[derive(Debug, PartialEq, PartialOrd)]
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
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z + rhs.z
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
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
mod test {
    use super::Point3Index;
    use super::Vec3;

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(4f64, 5f64, 6f64);
        assert_eq!(&v1 + &v2, Vec3::new(5f64, 7f64, 9f64));
        assert_eq!(&v2 + &v1, Vec3::new(5f64, 7f64, 9f64));
        let v3 = Vec3::new(1f64, 1f64, 1f64);
        assert_eq!(&v1 + &v3, Vec3::new(2f64, 3f64, 4f64));
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(4f64, 5f64, 6f64);
        let v2 = Vec3::new(1f64, 2f64, 3f64);
        assert_eq!(&v1 - &v2, Vec3::new(3f64, 3f64, 3f64));
        assert_eq!(&v2 - &v1, Vec3::new(-3f64, -3f64, -3f64));
        let v3 = Vec3::new(1f64, 1f64, 1f64);
        assert_eq!(&v1 - &v3, Vec3::new(3f64, 4f64, 5f64));
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1f64, 2f64, 3f64);
        assert_eq!(&v1 * 2f64, Vec3::new(2f64, 4f64, 6f64));

        let v2 = Vec3::new(4f64, 5f64, 6f64);
        assert_eq!(&v1 * &v2, Vec3::new(4f64, 10f64, 18f64));
    }

    #[test]
    fn test_index() {
        let v = Vec3::new(1f64, 2f64, 3f64);
        assert!((v[Point3Index::X] - 1f64).abs() < f64::EPSILON);
        assert!((v[Point3Index::Y] - 2f64).abs() < f64::EPSILON);
        assert!((v[Point3Index::Z] - 3f64).abs() < f64::EPSILON);
    }

    #[test]
    fn test_add_assign() {
        let mut v1 = Vec3::new(1f64, 2f64, 3f64);
        let v2 = Vec3::new(1f64, 2f64, 3f64);
        v1 += &v2;
        assert_eq!(v1, Vec3::new(2f64, 4f64, 6f64));
        assert_eq!(v2, Vec3::new(1f64, 2f64, 3f64));
    }

    #[test]
    fn test_mul_assign() {
        let mut v = Vec3::new(1f64, 2f64, 3f64);
        v *= 2f64;
        assert_eq!(v, Vec3::new(2f64, 4f64, 6f64));
    }
    #[test]
    fn test_div_assign() {
        let mut v = Vec3::new(2f64, 4f64, 6f64);
        v /= 2f64;
        assert_eq!(v, Vec3::new(1f64, 2f64, 3f64));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(2f64, 4f64, 6f64);
        assert!((v.length() - 56f64.sqrt()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_display() {
        let v = Vec3::new(0.1f64, 0.2f64, 0.3f64);
        assert_eq!(format!("{}", v), "25 51 76");
    }
}
