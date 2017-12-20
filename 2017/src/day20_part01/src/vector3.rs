use std::ops;

pub type Scalar = i64;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    pub fn mannhattan_distance(&self, rhs: &Self) -> Scalar {
        (self.x + rhs.x).abs() +
        (self.y + rhs.y).abs() +
        (self.z + rhs.z).abs()
    }
}

impl<'a> From<&'a [i64]> for Vector3 {
    fn from(slice: &[i64]) -> Self {
        assert!(slice.len() == 3, "invalid number of numbers passed to Vector3::from");
        Self { x: slice[0], y: slice[1], z: slice[2] }
    }
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(mut self, rhs: Vector3) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: Vector3) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::Mul<Scalar> for Vector3 {
    type Output = Vector3;

    fn mul(mut self, rhs: Scalar) -> Self::Output {
        self *= rhs;
        self
    }
}

impl ops::MulAssign<Scalar> for Vector3 {
    fn mul_assign(&mut self, rhs: Scalar) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}