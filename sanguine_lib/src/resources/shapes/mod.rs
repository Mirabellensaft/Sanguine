use svg::node::element::path::Data;
use svg::node::element::Path;

use crate::resources::
    shapes::{line::Line, point::Point};

use super::layout::grid::Field;

pub mod circle;
pub mod ellipse;
pub mod line;
pub mod point;
/// This module contains types related to shapes that show up in the rendered or plotted image.
/// Everything is hard coded to generate black lines of 1px width, as this is the only relevant
/// setting for the plotter
pub trait Shape {
    fn contains(&self, point: Point) -> bool;
    fn intersection(&self, line: Line, step: f32) -> Option<Point>;
    fn return_center(&self) -> Point;
}
/// Hard coded path data
fn path(data: Data) -> Path {
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    path
}

/// Creates a random distorted square
pub fn distorted_square(field: Field) -> Path {
    let data = Data::new()
        .move_to((
            Point::random_coordinate(&field, 0).x,
            Point::random_coordinate(&field, 0).y,
        ))
        .line_to((
            Point::random_coordinate(&field, 0).x,
            Point::random_coordinate(&field, 0).y,
        ))
        .line_to((
            Point::random_coordinate(&field, 0).x,
            Point::random_coordinate(&field, 0).y,
        ))
        .line_to((
            Point::random_coordinate(&field, 0).x,
            Point::random_coordinate(&field, 0).y,
        ))
        .close();

    let path = path(data);
    path
}

// Helpers
pub fn is_x_range_larger(x_1: f32, x_2: f32, y_1: f32, y_2: f32) -> bool {
    let mut x_range = 0.0;
    let mut y_range = 0.0;
    if let Some(x) = smaller_value(x_1, x_2) {
        x_range = x.1 - x.0;
    } else {
        return false
    }

    if let Some(y) = smaller_value(y_1, y_2) {
        y_range = y.1 - y.0;
    } else {
        return true;
    }

    if x_range > y_range {
        true
    } else {
        false
    }
}

/// Helper function that returns a valid range between two values.
pub fn range(x_1: f32, x_2: f32) -> std::ops::Range<i32> {
    if x_1 > x_2 {
        let range = std::ops::Range {
            start: x_2 as i32,
            end: x_1 as i32,
        };
        return range;
    } else {
        let range = std::ops::Range {
            start: x_1 as i32,
            end: x_2 as i32,
        };

        return range;
    };
}

/// Helper function that returns a tuple, lower value first. 
/// Returns None if values are the same.
pub fn smaller_value(v_1: f32, v_2: f32) -> Option<(f32, f32)> {
    if v_1 > v_2 {
        Some((v_2, v_1))
    } else if v_1 < v_2{
        Some((v_1, v_2))
    } else {
        None
    }
}