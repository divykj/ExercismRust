use std::default::Default;
use std::ops::Add;

pub struct Triangle<T>(T, T, T);

impl<T> Triangle<T>
where
    T: Default + PartialEq + PartialOrd + Copy + Add<T, Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let (x, y, z) = (sides[0], sides[1], sides[2]);
        let default = T::default();
        if x != default && y != default && z != default && x + y > z && y + z > x && z + x > y {
            Some(Triangle(x, y, z))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.1 == self.2
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.2 == self.0
    }
    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }
}
