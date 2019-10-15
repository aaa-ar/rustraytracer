use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(self) -> f32 {
        self.x
    }

    pub fn y(self) -> f32 {
        self.y
    }

    pub fn z(self) -> f32 {
        self.z
    }

    pub fn length(self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(self) -> f32 {
        self.dot(&self)
    }

    pub fn dot(self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn sqrt(self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn unit_vector(other: Vec3) -> Vec3 {
        other / other.length()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = *self + other;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        self + -other
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self += -other;
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = *self * other;
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        *self = *self * other;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        *self = *self / other;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn test_neg() {
        let i = Vec3::new(1.0, 2.0, 3.0);
        let j = Vec3::new(-1.0, -2.0, -3.0);

        assert_eq!(-(-i), i);
        assert_eq!(-i, j);
        assert_eq!(i, -j);
    }

    #[test]
    pub fn test_add() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        let l = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(i + j + k, l);
        assert_eq!(i + i + j + j + k + k, l + l);
    }

    #[test]
    pub fn test_add_assign() {
        let mut i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        let l = Vec3::new(1.0, 1.0, 1.0);

        i += j;
        i += k;

        assert_eq!(i, l);
    }

    #[test]
    pub fn test_sub() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        let l = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(-i - j - k, -l);
        assert_eq!(-i - i - j - j - k - k, -l - l);
    }

    #[test]
    pub fn test_sub_assign() {
        let mut i = Vec3::new(-1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        let l = Vec3::new(-1.0, -1.0, -1.0);

        i -= j;
        i -= k;

        assert_eq!(i, l);
    }

    #[test]
    pub fn test_scalar_mul() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        let l = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(i * 2.0 + j * 2.0 + k * 2.0, l * 2.0);
        assert_eq!(-i * 2.0 - j * 2.0 - k * 2.0, -l * 2.0);
    }

    #[test]
    pub fn test_scalar_mul_assign() {
        let mut i = Vec3::new(1.0, 0.0, 0.0);
        let mut j = Vec3::new(0.0, 1.0, 0.0);
        let mut k = Vec3::new(0.0, 0.0, 1.0);
        let l = Vec3::new(2.0, 3.0, 5.0);

        i *= 2.0;
        j *= 3.0;
        k *= 5.0;

        assert_eq!(i + j + k, l);
    }

    #[test]
    pub fn test_scalar_div() {
        let i = Vec3::new(2.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 2.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 2.0);
        let l = Vec3::new(2.0, 2.0, 2.0);

        assert_eq!(i / 2.0 + j / 2.0 + k / 2.0, l / 2.0);
        assert_eq!(-i / 2.0 - j / 2.0 - k / 2.0, -l / 2.0);
    }

    #[test]
    pub fn test_scalar_div_assign() {
        let mut i = Vec3::new(1.0, 0.0, 0.0);
        let mut j = Vec3::new(0.0, 1.0, 0.0);
        let mut k = Vec3::new(0.0, 0.0, 1.0);
        let l = Vec3::new(2.0, 3.0, 5.0);

        i *= 2.0;
        j *= 3.0;
        k *= 5.0;

        assert_eq!(i + j + k, l);
    }

    #[test]
    pub fn test_hadamard_mul() {
        let i = Vec3::new(2.0, 1.0, 2.0);
        let j = Vec3::new(2.0, 2.0, 1.0);
        let k = Vec3::new(1.0, 2.0, 2.0);
        let l = Vec3::new(4.0, 4.0, 4.0);

        assert_eq!(i * j * k, l);
    }

    #[test]
    pub fn test_hadamard_mul_assign() {
        let mut i = Vec3::new(2.0, 1.0, 2.0);
        let j = Vec3::new(2.0, 2.0, 1.0);
        let k = Vec3::new(1.0, 2.0, 2.0);
        let l = Vec3::new(4.0, 4.0, 4.0);

        i *= j;
        i *= k;

        assert_eq!(i, l);
    }

    #[test]
    pub fn test_length() {
        let i = Vec3::new(3.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 3.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 3.0);

        assert_eq!(i.length(), 3.0);
        assert_eq!(j.length(), 3.0);
        assert_eq!(k.length(), 3.0);

        assert_eq!((-i).length(), 3.0);
        assert_eq!((-j).length(), 3.0);
        assert_eq!((-k).length(), 3.0);

        assert_eq!((i + j + k).length(), 27.0_f32.sqrt());
        assert_eq!((-i - j - k).length(), 27.0_f32.sqrt());
    }

    #[test]
    pub fn test_squared_length() {
        let i = Vec3::new(3.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 3.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 3.0);

        assert_eq!(i.squared_length(), 9.0);
        assert_eq!(j.squared_length(), 9.0);
        assert_eq!(k.squared_length(), 9.0);

        assert_eq!((-i).squared_length(), 9.0);
        assert_eq!((-j).squared_length(), 9.0);

        assert_eq!((-k).squared_length(), 9.0);
        assert_eq!((i + j + k).squared_length(), 27.0);
        assert_eq!((-i - j - k).squared_length(), 27.0);
    }

    #[test]
    fn test_dot_product() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(i.dot(&j), 0.0);
        assert_eq!(j.dot(&k), 0.0);
        assert_eq!(k.dot(&i), 0.0);

        assert_eq!(j.dot(&i), 0.0);
        assert_eq!(k.dot(&j), 0.0);
        assert_eq!(i.dot(&k), 0.0);

        assert_eq!((i * 2.0).dot(&(j * 2.0)), 0.0);
        assert_eq!((j * 2.0).dot(&(k * 2.0)), 0.0);
        assert_eq!((k * 2.0).dot(&(i * 2.0)), 0.0);
        assert_eq!((i + j + k).dot(&(i * 2.0)), 2.0);
    }

    #[test]
    pub fn test_cross_product() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(i.cross(&j), k);
        assert_eq!(j.cross(&k), i);
        assert_eq!(k.cross(&i), j);

        assert_eq!(j.cross(&i), -k);
        assert_eq!(k.cross(&j), -i);
        assert_eq!(i.cross(&k), -j);

        assert_eq!((i * 2.0).cross(&(j * 2.0)), k * 4.0);
        assert_eq!((j * 2.0).cross(&(k * 2.0)), i * 4.0);
        assert_eq!((k * 2.0).cross(&(i * 2.0)), j * 4.0);
    }
}
