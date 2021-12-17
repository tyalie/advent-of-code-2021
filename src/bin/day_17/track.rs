use num_traits::{PrimInt, sign, Signed};
use core::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Track<T> where T: PrimInt {
    pub position: Point<T>,
    velocity: Vector<T>
}

#[derive(Clone, Copy)]
pub struct Point<T> where T: PrimInt {
    pub x: T, 
    pub y: T
}

#[derive(Clone, Copy)]
pub struct Vector<T> where T: PrimInt {
    pub dx: T, 
    pub dy: T
}

impl<T> Add<Vector<T>> for Point<T> where T: PrimInt {
    type Output = Point<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        Point {
            x: self.x + rhs.dx,
            y: self.y + rhs.dy
        }
    }
}

impl<T> Sub<Vector<T>> for Point<T> where T: PrimInt {
    type Output = Point<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Point {
            x: self.x - rhs.dx,
            y: self.y - rhs.dy
        }
    }
}

impl<T> Vector<T> where T: PrimInt + Signed {
    fn reduce(&self) -> Self {
        Vector { 
            dx: self.dx - sign::signum(self.dx), 
            dy: self.dy - T::one()
        }
    }
}

impl<T> Track<T> where T: PrimInt + Signed {
    pub fn start(velocity: Vector<T>) -> Self {
        Track { velocity, position: Point { x: T::zero(), y: T::zero() } }
    }

    pub fn do_step(&self) -> Self {
        Track {
            position: self.position + self.velocity,
            velocity: self.velocity.reduce()
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.velocity.dx == T::zero() && self.velocity.dy == T::zero()
    }
}
