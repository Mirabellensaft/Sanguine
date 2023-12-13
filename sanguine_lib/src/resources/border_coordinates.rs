use crate::resources::{layout::{Layout, grid::Grid}, shapes::point::Point};
use rand::{thread_rng, Rng};

use super::layout::grid::Field;

/// This module contains a bunch of functions that create random coordinates on field borders.
///
/// It's currently limited to fields with 4 sides.
#[derive(Clone, Debug, PartialEq)]
pub struct OneSide(pub Vec<Point>);

impl OneSide {
    fn new() -> Self {
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

    pub fn sort_points_in_line_up(&mut self) {
        let mut execution = 0;

        if self.0[2].x == self.0[3].x {
            for point in 1..self.0.len() {
                if self.0[point].y < self.0[point - 1].y {
                    self.0.swap(point, point - 1);
                    execution += 1;
                }
            }
        } else if self.0[2].y == self.0[3].y {
            for point in 1..self.0.len() {
                if self.0[point].x < self.0[point - 1].x {
                    self.0.swap(point, point - 1);
                    execution += 1;
                }
            }
        } else {
            println!("points not in line")
        };

        if execution != 0 {
            self.sort_points_in_line_up()
        }
    }

    pub fn sort_points_in_line_down(&mut self) {
        let mut execution = 0;

        if self.0[2].x == self.0[3].x {
            for point in 1..self.0.len() {
                if self.0[point].y > self.0[point - 1].y {
                    self.0.swap(point, point - 1);
                    execution += 1;
                }
            }
        } else if self.0[2].y == self.0[3].y {
            for point in 1..self.0.len() {
                if self.0[point].x > self.0[point - 1].x {
                    self.0.swap(point, point - 1);
                    execution += 1;
                }
            }
        } else {
            println!("points not in line")
        };

        if execution != 0 {
            self.sort_points_in_line_down()
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

#[derive(Clone, Debug, PartialEq)]
pub struct BorderCoordinates(pub Vec<OneSide>);

impl BorderCoordinates {
    /// Returns a vec of 4 vecs with a given number of random points, so you have random points
    /// around the edge of a field.
    fn new(field: Field, amount: usize) -> Self {
        let mut sides = Vec::new();

        let mut coordinates = OneSide::new();
        let x = array_of_values(field.x, field.column_width, 1);
        for i in 0..amount {
            coordinates.0.push(Point::new(x[i], field.y as f32));
            // println!("coordinates:{:?}", coordinates[side][i]);
        }
        coordinates.sort_points_in_line_up();
        // coordinates.chaos_sort();
        sides.push(coordinates);

        let mut coordinates = OneSide::new();
        let y = array_of_values(field.y, field.row_height, 1);
        for i in 0..amount {
            coordinates.0.push(Point::new(field.x as f32, y[i]));
            // println!("coordinates:{:?}", coordinates[side][i]);
        }
        coordinates.sort_points_in_line_up();
        sides.push(coordinates);

        let mut coordinates = OneSide::new();
        let x = array_of_values(field.x, field.column_width, 1);
        for i in 0..amount {
            coordinates
                .0
                .push(Point::new(x[i], field.y as f32 + field.row_height as f32));
            // println!("coordinates:{:?}", coordinates[side][i]);
        }
        coordinates.sort_points_in_line_up();
        sides.push(coordinates);

        let mut coordinates = OneSide::new();
        let y = array_of_values(field.y, field.row_height, 1);
        for i in 0..amount {
            coordinates
                .0
                .push(Point::new(field.x as f32 + field.column_width as f32, y[i]));
            // println!("coordinates:{:?}", coordinates[side][i]);
        }
        coordinates.sort_points_in_line_up();
        sides.push(coordinates);
        BorderCoordinates(sides)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct AllBorderCoordinates(pub Vec<Vec<BorderCoordinates>>);

impl AllBorderCoordinates {
    /// Returns a Vector of edge points for an entire work of art.
    pub fn new(work: &Grid, amount: usize) -> Self {
        let mut vec = Vec::new();

        for row in 0..work.get_rows() {
            let mut inner = Vec::new();
            for col in 0..work.get_columns() {
                inner.push(BorderCoordinates::new(
                    work.get_fields()[row][col],
                    amount,
                ));
            }
            vec.push(inner);
        }
        AllBorderCoordinates(vec)
    }

    /// Tesselation, makes sure that points on the edges of a field are corresponding with the neighboring edge
    /// 0 = top
    /// 1 = left
    /// 2 = bottom
    /// 3 = right
    pub fn tesselate(&mut self) {
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                if row == 0 && col != 0 {
                    self.0[row][col].0.remove(1);
                    let copy_side = self.0[row][col - 1].0[3].copy_side();
                    self.0[row][col].0.insert(1, copy_side);
                } else if row != 0 && col == 0 {
                    self.0[row][col].0.remove(0);
                    let copy_side = self.0[row - 1][col].0[2].copy_side();
                    self.0[row][col].0.insert(0, copy_side);
                } else if row != 0 && col != 0 {
                    self.0[row][col].0.remove(1);
                    let copy_side = self.0[row][col - 1].0[3].copy_side();
                    self.0[row][col].0.insert(1, copy_side);

                    self.0[row][col].0.remove(0);
                    let copy_side = self.0[row - 1][col].0[2].copy_side();
                    self.0[row][col].0.insert(0, copy_side);
                }
            }
        }
    }

    pub fn slight_chaos(&mut self) {
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                for side in 0..self.0[row][col].0.len() {
                    self.0[row][col].0[side].chaos_sort();
                }
            }
        }
    }
}

/// Creates a random value that can be used as x or y value, of the other one is fixed.
fn random_value_on_side(start_value: i32, distance: i32, margin: i32) -> f32 {
    let mut rng = thread_rng();

    let range: std::ops::Range<i32> = std::ops::Range {
        start: start_value + margin,
        end: start_value + distance - margin,
    };

    rng.gen_range(range) as f32
}

/// Crates an array of 10 values to be used as x or y values, if the other one is fixed.
fn array_of_values(start_value: i32, distance: i32, margin: i32) -> [f32; 10] {
    let mut array = [0.0; 10];
    for i in 0..10 {
        array[i] = random_value_on_side(start_value, distance, margin)
    }
    array = correct_distance(array, start_value, distance, margin);
    array
}

/// Replaces duplicate values in a value array.
fn repl_duplicates(
    mut array: [f32; 10],
    start_value: i32,
    distance: i32,
    margin: i32,
) -> [f32; 10] {
    for i in 1..=10 {
        if array[i..].contains(&array[i - 1]) {
            array[i - 1] = random_value_on_side(start_value, distance, margin);
            array = repl_duplicates(array, start_value, distance, margin);
            array = correct_distance(array, start_value, distance, margin);
        }
    }
    array
}

/// Replaces values in an array if they are too close together.
fn correct_distance(
    mut array: [f32; 10],
    start_value: i32,
    distance: i32,
    margin: i32,
) -> [f32; 10] {
    for i in 0..10 {
        for j in 0..10 {
            if i != j {
                if (array[i] - array[j]).abs() < 3.0 {
                    array[i] = random_value_on_side(start_value, distance, margin);
                }
            }
        }
    }
    array = repl_duplicates(array, start_value, distance, margin);
    array
}

// pub fn point_on_circle(cx: i32, cy: i32, radius: i32) -> (i32, i32) {

// }

// assert_eq!(has_dup(&[1, 2, 3, 2, 5, 6]), true);
//     assert_eq!(has_dup(&[1, 2, 3, 4, 5, 6]), false);

// #[cfg(test)]
// use crate::resources;
// #[test]

// fn duplicates() {
//     println!("{:?}", check_duplicates_repl([1, 2, 3, 2, 5, 6, 7, 8, 9, 10]));
// }
