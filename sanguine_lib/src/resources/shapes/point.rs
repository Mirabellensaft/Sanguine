use rand::{thread_rng, Rng};
use svg::node::element::Circle as CirclePath;

use crate::resources::layout::grid::Field;

/// This module contains types related to shapes that show up in the rendered or plotted image.
/// Everything is hard coded to generate black lines of 1px width, as this is the only relevant
/// setting for the plotter

/// A Point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    /// Value on the x axis
    pub x: f32,
    /// Value on the y axis
    pub y: f32,
}

impl Point {
    /// Creates a new point with a given x and y value.
    pub fn new(x: f32, y: f32) -> Self {
        let point = Point { x: x, y: y };

        point
    }
    /// Creates a random Point within a field
    pub fn random_coordinate(field: &Field, margin: i32) -> Self {
        let mut rng = thread_rng();
        println!("random field: x{}, y{}, w{}, h{}, margin {}", field.x, field.y, field.column_width, field.row_height, margin);
        let horizontal: std::ops::Range<i32> = std::ops::Range {
            start: field.x + margin,
            end: field.x + field.column_width - margin,
        };
        println!("random X start:{}, end: {}", field.x+margin, field.x+field.column_width-margin);
        let vertical: std::ops::Range<i32> = std::ops::Range {
            start: field.y + margin,
            end: field.y + field.row_height - margin,
        };
        println!("random Y start:{}, end: {}", field.y+margin, field.y+field.row_height-margin);
        println!("random hor: {:?}, vert: {:?}", horizontal, vertical);
        // println!("h {:?}, v {:?}", horizontal, vertical);
        Point::new(
            rng.gen_range(horizontal) as f32,
            rng.gen_range(vertical) as f32,
        )
    }

    /// Calculates the distance to another point.
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn draw(&self) -> CirclePath {
        let circle = CirclePath::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("cx", self.x)
            .set("cy", self.y)
            .set("r", 1);
        circle
    }
}
