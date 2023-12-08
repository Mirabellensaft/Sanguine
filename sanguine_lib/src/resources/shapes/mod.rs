use svg::node::element::path::Data;
use svg::node::element::Path;

use crate::resources::{
    layout,
    shapes::{line::Line, point::Point},
};

pub mod circle;
pub mod ellipse;
pub mod line;
pub mod point;
/// This module contains types related to shapes that show up in the rendered or plotted image.
/// Everything is hard coded to generate black lines of 1px width, as this is the only relevant
/// setting for the plotter

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
pub fn distorted_square(field: layout::Field) -> Path {
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

pub trait Shape {
    fn contains(&self, point: Point) -> bool;
    fn intersection(&self, line: Line, step: f32) -> Option<Point>;
    fn return_center(&self) -> Point;
}
