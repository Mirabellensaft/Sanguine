use svg::node::element::{Path, Circle};
use svg::node::element::path::Data;


use crate::lib::{layout, random_numbers};

fn path(data: Data) -> Path {
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    path
}

pub fn distorted_square(field: layout::Field) -> Path {
    let data = Data::new()
        .move_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .close();

    let path = path(data);
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

pub fn line(start_x: i32, start_y:i32, end_x: i32, end_y: i32) -> Path {
    let data = Data::new()
        .move_to((start_x, start_y))
        .line_to((end_x, end_y));

    let path = path(data);
    path
}