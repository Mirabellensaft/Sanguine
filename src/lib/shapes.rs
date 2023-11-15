use svg::node::element::{Path, Circle};
use svg::node::{self, element::path::Data};
use svg::{Document, Node};

use crate::lib::{layout, random_numbers};

pub fn distorted_square(field: layout::Field) -> Path {
    let data = Data::new()
        .move_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);
    path
}

pub fn circle(cx: i32, cy: i32, radius: i32) -> Circle {
    let circle = Circle::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("cx", cx)
                .set("cy", cy)
                .set("r", radius);
    circle
}