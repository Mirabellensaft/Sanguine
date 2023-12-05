use svg::node::element::{path::Data, Ellipse};
use svg::node::element::Path;

use crate::resources::shapes::{path, point::Point};

use super::Shape;


/// This module contains types related to shapes that show up in the rendered or plotted image.
/// Everything is hard coded to generate black lines of 1px width, as this is the only relevant 
/// setting for the plotter

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

        if let Some(i) = self.slope()  {
            Some(self.end.y - (i * self.end.x))
        } else {
            None
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
}

