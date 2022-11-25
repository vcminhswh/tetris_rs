use std::ops::{Neg, Add, Mul, Sub, AddAssign, SubAssign, MulAssign};

use num_traits::Num as Number;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
pub struct Vec2<T: Number> {
    pub x: T,
    pub y: T
}

impl<T: Copy + Number> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn dot_product(&self, rhs: &Vec2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Copy + Number + Neg<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
         Vec2 {
             x: self.x + rhs.x,
             y: self.y + rhs.y
         }
    }
}

impl<T: Copy + Number + Neg<Output = T>> Add<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: T) -> Vec2<T> {
         Vec2 {
             x: self.x + rhs,
             y: self.y + rhs
         }
    }
}

impl<T: Copy + Number + Neg<Output = T>> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
         Vec2 {
             x: self.x - rhs.x,
             y: self.y - rhs.y
         }
    }
}

impl<T: Copy + Number + Neg<Output = T>> Sub<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: T) -> Vec2<T> {
         Vec2 {
             x: self.x - rhs,
             y: self.y - rhs
         }
    }
}

impl<T: Copy + Number + Neg<Output = T>> Mul<T> for Vec2<T> {
   type Output = Vec2<T>; 

   fn mul(self, rhs: T) -> Vec2<T> {
       Vec2 {
           x: self.x * rhs,
           y: self.y * rhs
       }
   }
}


impl<T: Copy + Number + Neg<Output = T>> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        *self = Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T: Copy + Number + Neg<Output = T>> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        *self = Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<T: Copy + Number + Neg<Output = T>> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Vec2 {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a_vec = Vec2::new(1,2);
        let b_vec = Vec2::new(3,4);
        let c_vec = a_vec + b_vec;
        assert_eq!(c_vec.x, 4);
        assert_eq!(c_vec.y, 6);
    }

}
