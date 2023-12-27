use crate::resources::shapes::{line::Line, point::Point, Shape};
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

    pub fn new_random(line: Line, amount: usize) -> Self {
        OneSide(line.random_points(amount))
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

    pub fn equal(&self, line: &Line) -> bool {
        println!("cell equal line");
        for point in &self.0 {
            if line.contains(*point) {
                println!("true");
                true;
            } else {
                return false;
            }
        }
        false
    }

    pub fn number_of_points(&self) -> usize {
        self.0.len()
    }
}
