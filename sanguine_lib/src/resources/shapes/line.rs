use svg::node::element::path::Data;
use svg::node::element::Path;
use rand::seq::SliceRandom;

use crate::resources::shapes::{path, point::Point};

use super::{Shape, is_x_range_larger, range};

/// This module contains types related to shapes that show up in the rendered or plotted image.
/// Everything is hard coded to generate black lines of 1px width, as this is the only relevant
/// setting for the plotter

/// A line between two points.

#[derive(Copy, Clone, Debug, PartialEq)]
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

    /// Returns the point on a line for a given x value. Returns none if there
    /// is no point for that x value, usually the case when the line is vertical.
    pub fn return_point_from_x(&self, x: f32) -> Option<Point> {
        if let Some(y_i) = self.y_intercept() {
            if let Some(i) = self.slope() {
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

    pub fn return_point_from_y(&self, y: f32) -> Option<Point> {
        // println!("point from y on line: {:?}", self);
        if let Some(y_i) = self.y_intercept() {
            if let Some(i) = self.slope() {
                let x = (y - y_i)/i;
                
                let point = Point { x: x, y: y };
                return Some(point)
            } else {
                let point = Point { x: self.start.x, y: y };
                return Some(point)
            }
        } else {
            let point = Point { x: self.start.x, y: y };
                return Some(point)
        }
    }

    /// Calculates the slope of a line, returns none the line is vertical
    pub fn slope(&self) -> Option<f32> {
        let d_x = self.start.x - self.end.x;
        let d_y = self.start.y - self.end.y;

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
        if let Some(i) = self.slope() {
            Some(self.end.y - (i * self.end.x))
        } else {
            None
        }
    }

    /// Returns an amount of random, non duplicate points on a line
    pub fn random_points(&self, amount: usize) -> Vec<Point> {
        println!("random points on line");

        let mut points_on_line = Vec::new();
        if is_x_range_larger(self.start.x, self.end.x, self.start.y, self.end.y) {
            // println!("x range is larger");
            let chosen_values = return_chosen_value(self.start.x, self.end.x, amount);
            // make y values
            for x in chosen_values {

                if let Some(point) = self.return_point_from_x(x as f32) {
                    println!("point: {:?}", point);
                    points_on_line.push(point);
                } else {
                    println!("No point for this x");
                }
            }
        } else {
            // println!("y range is larger");
            let chosen_values = return_chosen_value(self.start.y, self.end.y, amount);
            // make y values
            for y in chosen_values {

                if let Some(point) = self.return_point_from_y(y as f32) {
                    println!("point: {:?}", point);
                    points_on_line.push(point);
                } else {
                    println!("No point for this y");
                }
            }
        }
        points_on_line
    }

    pub fn equal(&self, other: Line) -> bool {
        if *self == other {
            true
        } else if self.start == other.end && self.end == other.start {
            true
        } else {
            false
        }
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

impl Shape for Line {
    fn return_center(&self) -> Point {
        unimplemented!()
    }

    fn contains(&self, point: Point) -> bool {
        unimplemented!()
    }

    /// Returns the point where two lines intersect, returns None, if they don't intersect.
    fn intersection(&self, other: Line, step: f32) -> Option<Point> {
        let mut diff = 10000000000.0;
        println!("step {}", step);

        // statt 0, start von start.x, was genau muss der hächste wert hier sein? x max?
        // geht collision mit dem kreis nicht auch so und besser?????

        // compare slopes for easy "None" exit for parallel lines.

        for x in (0..300 * ((1.0 / step) as i32)).map(|x| x as f32 * step) {
            if let Some(point_1) = self.return_point_from_x(x as f32) {
                if let Some(point_2) = other.return_point_from_x(x as f32) {
                    println!("x: {}, y_1: {}, y_2: {}", x, point_1.y, point_2.y);

                    if point_1.y < point_2.y + 2.0 && point_1.y > point_2.y - 2.0 {
                        let point = Point {
                            x: x as f32,
                            y: point_1.y,
                        };
                        return Some(point);
                    } else {
                        if diff > (point_2.y - point_1.y).abs() {
                            diff = (point_2.y - point_1.y).abs();
                        } else {
                            // neuer startpunkt, davon neue linie, die dann als other geht
                            return self.intersection(other, step / 10.0);
                        }
                    }
                }
            }
        }
        None
    }
}


// Helpers

fn return_chosen_value(start_v: f32, end_v: f32, amount: usize) -> Vec<i32>{
    let mut p_values =Vec::new();
    let mut chosen_values =  Vec::new();
            
    let mut rng = &mut rand::thread_rng();
    for value in range(start_v,end_v).step_by(2) {
        p_values.push(value);
        if amount < p_values.len() {
            chosen_values = p_values.choose_multiple(&mut rng, amount).cloned().collect();
        } else {
            let amount = p_values.len()/2;
            chosen_values = p_values.choose_multiple(&mut rng, amount).cloned().collect();
        }
    }
    chosen_values
}

#[cfg(test)]

#[test]
fn random_on_line_1() {
    let line_1 = Line::new(Point::new(1.0, 5.0),Point::new(1.0, 300.0));
    let points = line_1.random_points(10);
    println!("{:?}", points);
    assert_eq!(points.len(), 10);
}
#[test]
fn random_on_line_2() {
    let line_1 = Line::new(Point::new(1.0, 5.0),Point::new(3000.0, 5.0));
    let points = line_1.random_points(10);
    println!("{:?}", points);
    assert_eq!(points.len(), 10);
}

#[test]
fn random_on_line_3() {
    let line_1 = Line::new(Point::new(1.0, 5.0),Point::new(2.0, 300.0));
    let points = line_1.random_points(10);
    println!("{:?}", points);
    assert_eq!(points.len(), 10);
}