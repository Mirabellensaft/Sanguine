use crate::resources::{layout::grid::Field, shapes::point::Point};
use rand::{thread_rng, Rng};

use super::one_side::OneSide;

/// This module contains a bunch of functions that create random coordinates on field borders.
///
/// It's currently limited to fields with 4 sides.

#[derive(Clone, Debug, PartialEq)]
pub struct CellBorderCoords(pub Vec<OneSide>);

impl CellBorderCoords {
    /// Returns a vec of 4 vecs with a given number of random points, so you have random points
    /// around the edge of a field.
    ///
    pub fn new(field: Field, amount: usize) -> Self {
        let mut sides = Vec::new();

        let mut coordinates = OneSide::new();
        let x = array_of_values(field.x, field.column_width, 1);
        for i in 0..amount {
            coordinates.0.push(Point::new(x[i], field.y as f32));
            // println!("coordinates:{:?}", coordinates[side][i]);
        }
        coordinates.sort_points_on_line();
        // coordinates.chaos_sort();
        sides.push(coordinates);

        let mut coordinates = OneSide::new();
        let y = array_of_values(field.y, field.row_height, 1);
        for i in 0..amount {
            coordinates.0.push(Point::new(field.x as f32, y[i]));
            // println!("coordinates:{:?}", coordinates[side][i]);
        }
        coordinates.sort_points_on_line();
        sides.push(coordinates);

        let mut coordinates = OneSide::new();
        let x = array_of_values(field.x, field.column_width, 1);
        for i in 0..amount {
            coordinates
                .0
                .push(Point::new(x[i], field.y as f32 + field.row_height as f32));
            // println!("coordinates:{:?}", coordinates[side][i]);
        }
        coordinates.sort_points_on_line();
        sides.push(coordinates);

        let mut coordinates = OneSide::new();
        let y = array_of_values(field.y, field.row_height, 1);
        for i in 0..amount {
            coordinates
                .0
                .push(Point::new(field.x as f32 + field.column_width as f32, y[i]));
            // println!("coordinates:{:?}", coordinates[side][i]);
        }
        coordinates.sort_points_on_line();
        sides.push(coordinates);
        CellBorderCoords(sides)
    }

    pub fn new_empty() -> Self {
        let mut sides = Vec::new();

        let coordinates = OneSide::new();
        sides.push(coordinates);
        CellBorderCoords(sides)
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
