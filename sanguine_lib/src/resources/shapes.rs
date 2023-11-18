use svg::node::element::{Path, Circle as CirclePath};
use svg::node::element::path::Data;


use crate::resources::{layout, random_numbers};


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

#[derive(Copy, Clone, Debug, PartialEq)]

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

    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
   
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        let line: Line = Line {
            start: start,
            end: end,
        };

        line
    }

    pub fn slope(&self) -> f32 {

        let mut m = 0.0;

        let d_x = self.start.x - self.end.x; 
        let d_y = self.start.y - self.end.y;

        // println!("d_x: {}", d_x);
        // println!("d_y: {}", d_y);
        

        if d_x == 0.0 {
            m = 0.1;
        } if d_y == 0.0 {
            m = 0.0; 
        } else {
            m = d_y / d_x;
        };
               

        // println!("m: {}", m);
        m
    }

    pub fn y_intercept(&self) -> f32 {

        let b = self.end.y  - (self.slope() * self.end.x);
        // println!("b: {} = y: {} - m: {} * x: {}", b, self.end.y, self.slope(), self.end.x);

        b

    }

    pub fn return_point_on_line(&self, x: f32) -> Point {
        
        let y = (self.slope() * x) + self.y_intercept();
        // println!("y: {} = m: {}*  x: {} + b: {}", y, self.slope(), self.end.x, self.y_intercept());
        
        
        let point = Point {
            x: x,
            y: y,
        };
        // println!("point: {:?}", point);
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
    radius: f32,
}

impl Circle {

    pub fn new(center: Point, radius: f32) -> Self {
        let circle = Circle {
            center: center,
            radius: radius,
        };
        circle
    }

    pub fn contains(&self, point: Point) -> bool {
        // println!("{} {}",self.center.distance_to(&point), self.radius );
        self.center.distance_to(&point) <= self.radius

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

#[cfg(test)]
use crate::resources;
    #[test]

    fn distance_test() {
        let point_1 = resources::shapes::Point::new(1.0, 10.0);
        let point_2 = resources::shapes::Point::new(10.0, 10.0);

        assert_eq!(point_1.distance_to(&point_2), 9.0);
    }
    #[test]
    fn circle_1() {
        let point_1 = resources::shapes::Point::new(5.0, 5.0);
        let point_2 = resources::shapes::Point::new(1.0, 1.0);
        let circle = resources::shapes::Circle::new(point_1, 2.0);

        assert_eq!(circle.contains(point_2), false);
    }
    #[test]
    fn circle_2() {
        let point_1 = resources::shapes::Point::new(5.0, 5.0);
        let point_2 = resources::shapes::Point::new(4.0, 4.0);
        let circle = resources::shapes::Circle::new(point_1, 2.0);

        assert_eq!(circle.contains(point_2), true);
    }
    #[test]
    fn circle_3() {
        let point_1 = resources::shapes::Point::new(5.0, 5.0);
        let point_2 = resources::shapes::Point::new(3.0, 5.0);
        let circle = resources::shapes::Circle::new(point_1, 2.0);

        assert_eq!(circle.contains(point_2), true);
    }

    #[test]
    fn circle_4() {
        let point_1 = resources::shapes::Point::new(5.0, 5.0);
        let point_2 = resources::shapes::Point::new(3.0, 5.0);
        let circle = resources::shapes::Circle::new(point_1, 1.9);

        assert_eq!(circle.contains(point_2), false);
    }

    #[test]
    fn point_on_line() {
        let point_1 = resources::shapes::Point::new(2.0, 7.0);
        let point_2 = resources::shapes::Point::new(10.0, 7.0);
        let point_3 = resources::shapes::Point::new(5.0, 7.0);
        let line = resources::shapes::Line::new(point_1, point_2);

        assert_eq!(line.return_point_on_line(5.0), point_3);
    }



