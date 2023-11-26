use svg::node::element::path::Data;
use svg::node::element::{Circle as CirclePath, Path};
use rand::{thread_rng, Rng};

use crate::resources::layout;

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
    pub fn random_coordinate(field: &layout::Field, margin: i32) -> Self {
        let mut rng = thread_rng();

        let horizontal: std::ops::Range<i32> = std::ops::Range {
            start: field.x + margin,
            end: field.x + field.column_width - margin,
        };

        let vertical: std::ops::Range<i32> = std::ops::Range {
            start: field.y + margin,
            end: field.y + field.row_height - margin,
        };
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
}

/// A line between two points.
pub struct Line {
    /// starting point
    pub start: Point,
    /// end point
    pub end: Point,
}

impl Line {
    /// Creates a new line between two given points.
    pub fn new(start: Point, end: Point) -> Self {
        let line: Line = Line {
            start: start,
            end: end,
        };

        line
    }

    /// Calculates the slope of a line, returns none the line is vertical
    pub fn slope(&self) -> Option<f32> {

        let d_x = self.start.x - self.end.x;
        let d_y = self.start.y - self.end.y;
        
        // println!("d_y: {}", d_y);
        // println!("d_x: {}", d_x);

        if d_x == 0.0 {
            None
        } else if d_y == 0.0 {
            Some(0.0)
        } else {
            Some(d_y / d_x)
        }
    }

    /// Returns the y intercept of a line. Returns None if the line is vertical and 
    /// does not cross the y-axis.
    pub fn y_intercept(&self) -> Option<f32> {

        if let Some(i) = self.slope()  {
            Some(self.end.y - (i * self.end.x))
        } else {
            None
        }
    }

    /// Returns the point on a line for a given x value. Returns none if there 
    /// is no point for that x value, usually the case when the line is vertical.
    pub fn return_point_on_line(&self, x: f32) -> Option<Point> {
        
        if let Some(y_i) = self.y_intercept() {
            if let Some(i) = self.slope()  {
                let y = (i * x) + y_i;
                let point = Point { x: x, y: y };
                Some(point)
            } else {
                None
            }

        } else {
            None
        }
    }

    /// Returns the point where two lines intersect, returns None, if they don't intersect.
    pub fn intersection(&self, other: Line, step: f32) -> Option<Point> {

        let mut diff = 10000000000.0;
        println!("step {}", step);

        // statt 0, start von start.x, was genau muss der hächste wert hier sein? x max?
        // geht collision mit dem kreis nicht auch so und besser?????

        // compare slopes for easy "None" exit for parallel lines.
    
        for x in (0..300 * ((1.0/step) as i32)).map(|x| x as f32 * step){
            if let Some(point_1) = self.return_point_on_line(x as f32) {
               
                if let Some(point_2)  =  other.return_point_on_line(x as f32) {
                    println!("x: {}, y_1: {}, y_2: {}", x, point_1.y, point_2.y);

                  
                    if point_1.y < point_2.y + 2.0 && point_1.y > point_2.y - 2.0  {
                        let point = Point { x: x as f32, y: point_1.y };
                        return Some(point)
                    } else {

                        if diff > (point_2.y - point_1.y).abs() {
                            diff = (point_2.y - point_1.y).abs();

                        } else {
                            // neuer startpunkt, davon neue linie, die dann als other geht
                            return self.intersection(other, step/10.0);
                        } 
                    }
                }     
            }
        }
        None
    }

    /// Creates an svg path for a line
    pub fn draw(&self) -> Path {
        let data = Data::new()
            .move_to((self.start.x, self.start.y))
            .line_to((self.end.x, self.end.y));

        let path = path(data);
        path
    }
}

/// A circle around a center point with a radius.
pub struct Circle {
    /// The center point
    pub center: Point,
    /// The radius
    pub radius: f32,
}

impl Circle {
    /// creates a new circle around a center point with a radius.
    pub fn new(center: Point, radius: f32) -> Self {
        let circle = Circle {
            center: center,
            radius: radius,
        };
        circle
    }

    /// Returns true, if a given point is within the area of the circle.
    pub fn contains(&self, point: Point) -> bool {

        self.center.distance_to(&point) <= self.radius
    }

    /// Returns the point where a line from the starting point to the center of 
    /// the circle hits the circle boundary. 
    /// 
    /// ⚠️ If the provided line does not go through the center of the circle, 
    /// the point returned will be off!
    pub fn intersection(&self, line: Line, step: f32) -> Option<Point> {

        let mut diff = 10000000000.0;
        // println!("\n\nstep {}", step);
        // println!("line slope {:?}", line.slope());
        if self.center.x == line.start.x {
            // println!("x=x");
            if self.center.y > line.start.y {
                let point = Point::new(self.center.x, self.center.y - self.radius);
                return Some(point)
            } else {
                let point = Point::new(self.center.x, self.center.y + self.radius);
                return Some(point)
            }
        }

        let iter_min = smallest_x(self.center.x, line.start.x);
        let iter_max = 300 * ((1.0/step) as i32);
    
        for i in (0..iter_max).map(|x| x as f32 * step){

            let x = iter_min + i;
            
            if let Some(point_1) = line.return_point_on_line(x) {
                // println!("x: {}, y: {}", point_1.x, point_1.y);

                if self.center.x > line.start.x {
                    if self.contains(point_1) {
                        if self.center.distance_to(&point_1) < self.radius - 0.5 {
                            return self.intersection(line, step/10.0);
                        } else {
                            // println!("left to center");
                            let point = Point { x: x as f32, y: point_1.y };
                            return Some(point)
                        }
                    } else {
                        if diff > self.center.distance_to(&point_1) {
                            diff = self.center.distance_to(&point_1)
                        } else {
                            // neuer startpunkt, davon neue linie, die dann als other geht
                            return self.intersection(line, step/10.0);
                        }
                    }
                } else if self.center.x < line.start.x {
                    // println!("center to right");
                    if self.contains(point_1) == false {
                        if self.center.distance_to(&point_1) < self.radius + 0.5 {
                            // println!("out of circle");
                            let point = Point { x: x as f32, y: point_1.y };
                            return Some(point)
                            // if let Some(point) = line.return_point_on_line(x - step){
                            //     return Some(point)
                            // }                           
                        } else {
                            return self.intersection(line, step/10.0);
                        }
                    } 
                } else {
  
                }
            }     
        }
        None
    }

    /// Creates an svg path for the circle
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

/// Helper function that returns a valid range between two values.
fn range(x_1: f32, x_2: f32) -> std::ops::Range<i32> {

    if x_1 > x_2 {
        let range = std::ops::Range {
            start: x_2 as i32,
            end: x_1 as i32,
        };
        return range
    } else {
        let range = std::ops::Range {
            start: x_1 as i32,
            end: x_2 as i32,
        };

        return range
    };
}

/// Helper function that returns the lower of two values
fn smallest_x(x_1: f32, x_2: f32) -> f32 {
    if x_1 > x_2 {
        x_2 
    } else {
        x_1
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

