use svg::node::element::{Path, Circle as CirclePath};
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

#[derive(Copy, Clone)]

pub struct Point{
    pub x: f32,
    pub y: f32,
}

impl Point {

    pub fn new(x: f32, y: f32) -> Self {
        let point = Point {
            x: x,
            y: y,
        };

        point
    }
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        let line = Line {
            start: start,
            end: end,
        };

        line
    }

    pub fn slope(&self) -> f32 {


        let d_x = self.start.x - self.end.x; 
        let d_y = self.start.y - self.end.y;

               
        let m = d_x / d_y;
        m
    }

    pub fn y_intercept(&self) -> f32 {

        let b = self.start.y as f32 - self.slope() * self.start.x as f32;
        b

    }

    pub fn return_point_on_line(&self, x: f32) -> Point {
        
        let y = self.slope() * self.start.x  as f32 + self.y_intercept();
        
        let point = Point {
            x: x,
            y: y,
        };
        point

    }

    pub fn draw(&self) -> Path {
        let data = Data::new()
            .move_to((self.start.x, self.start.y))
            .line_to((self.end.x, self.end.y));
    
        let path = path(data);
        path
    }


}

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {

    pub fn new(center: Point, radius: i32) -> Self {
        let circle = Circle {
            center: center,
            radius: radius,
        };
        circle
    }
    pub fn draw(&self) -> CirclePath {
        let circle = CirclePath::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("cx", self.center.x)
                    .set("cy", self.center.y)
                    .set("r", self.radius);
        circle
    }


}



