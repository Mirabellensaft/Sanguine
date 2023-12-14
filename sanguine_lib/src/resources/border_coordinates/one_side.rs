use crate::resources::shapes::point::Point;
use rand::{thread_rng, Rng};



/// This module contains a bunch of functions that create random coordinates on field borders.
///
/// It's currently limited to fields with 4 sides.
#[derive(Clone, Debug, PartialEq)]
pub struct OneSide(pub Vec<Point>);

impl OneSide {
    pub fn new() -> Self {
        let side: Vec<Point> = Vec::new();
        OneSide(side)
    }

    pub fn copy_side(&self) -> Self {
        let mut other: Vec<Point> = Vec::new();
        for point in 0..self.0.len() {
            let other_point = self.0[point];
            other.push(other_point);
        }
        OneSide(other)
    }

    pub fn sort_points_on_line(&mut self) {
        let mut execution = 0;

        for point in 1..self.0.len() {
            if self.0[point].x < self.0[point - 1].x {
                self.0.swap(point, point - 1);
                execution += 1;
            }
        }
        for point in 1..self.0.len() {
            if self.0[point].y < self.0[point - 1].y {
                self.0.swap(point, point - 1);
                execution += 1;
            }
        } 

        if execution != 0 {
            self.sort_points_on_line()
        }
    }

    pub fn chaos_sort(&mut self) {
        let mut rng = thread_rng();
        let max_len = self.0.len();

        let mut i = 0;
        for point in 0..max_len - 1 {
            if i != 0 {
                self.0.swap(point, point - i);
                i = 0;
                continue;
            }
            i = rng.gen_range(0..=1);
        }
    }
}
