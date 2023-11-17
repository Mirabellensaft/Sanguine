use svg::node::element::{Path, Circle};
use svg::node::element::path::Data;


use crate::lib::{layout, random_numbers, math};


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
        .move_to((random_numbers::coordinate(&field, 0).x, random_numbers::coordinate(&field, 0).y))
        .line_to((random_numbers::coordinate(&field, 0).x, random_numbers::coordinate(&field, 0).y))
        .line_to((random_numbers::coordinate(&field, 0).x, random_numbers::coordinate(&field, 0).y))
        .line_to((random_numbers::coordinate(&field, 0).x, random_numbers::coordinate(&field, 0).y))
        .close();

    let path = path(data);
    path
}

pub fn circle(center: math::Point, radius: i32) -> Circle {
    let circle = Circle::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("cx", center.x)
                .set("cy", center.y)
                .set("r", radius);
    circle
}

pub fn line(line: math::Line) -> Path {
    let data = Data::new()
        .move_to((line.start.x, line.start.y))
        .line_to((line.end.x, line.end.y));

    let path = path(data);
    path
}