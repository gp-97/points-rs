use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

pub struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
    pub fn mag_sqr(&self) -> T
    where
        T: Copy + Add + Mul + Add<Output = T> + Mul<Output = T>,
    {
        (self.x * self.x) + (self.y * self.y)
    }
    pub fn eucli_dist_sqr(&self, other: &Point2D<T>) -> T
    where
        T: Copy + Add + Mul + Sub + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }
    pub fn slope(&self, other: &Point2D<T>) -> Result<T, String>
    where
        T: Copy + Sub + Div + Sub<Output = T> + Div<Output = T> + PartialEq,
    {
        let del_y = self.y - other.y;
        let del_x = self.x - other.x;
        if self.y == other.y {
            Ok(del_y / del_x)
        } else {
            Err("Slope = Inf".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mag_sqr_test() {
        let p = Point2D::<f32>::new(6.0, 8.0);
        assert_eq!(p.mag_sqr(), 100.0);
    }
    #[test]
    fn eucli_dist_sqr_test() {
        let p = Point2D::<f32>::new(1.0, 2.0);
        let q = Point2D::<f32>::new(2.0, 3.0);
        assert_eq!(p.eucli_dist_sqr(&q), 2.0);
    }
    #[test]
    fn slope_test() {
        let p = Point2D::<f32>::new(0.0, 0.0);
        let q = Point2D::<f32>::new(0.0, 0.0);
        let slope = match p.slope(&q) {
            Ok(res) => res,
            Err(e) => panic!("{}", e),
        };
        println!("{}", slope);
        // assert_eq!(p.slope(&q), 1.0);
    }
}
