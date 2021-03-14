use std::ops::{Add, Mul, Sub};

pub struct Point3D<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x: x, y: y, z: z }
    }
    pub fn mag_sqr(&self) -> T
    where
        T: Copy + Add + Mul + Add<Output = T> + Mul<Output = T>,
    {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
    pub fn eucli_dist_sqr(&self, other: &Point3D<T>) -> T
    where
        T: Copy + Add + Mul + Sub + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mag_sqr_test() {
        let p = Point3D::<f32>::new(6.0, 8.0, 10.0);
        assert_eq!(p.mag_sqr(), 200.0);
    }
    #[test]
    fn eucli_dist_sqr_test() {
        let p = Point3D::<f32>::new(1.0, 2.0, 3.0);
        let q = Point3D::<f32>::new(4.0, 5.0, 6.0);
        assert_eq!(p.eucli_dist_sqr(&q), 27.0);
    }
}
