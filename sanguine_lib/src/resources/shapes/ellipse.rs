use crate::resources::shapes::{line::Line, point::Point};
use svg::node::element;

use super::{circle::smallest_x, Shape};

/// This module contains types related to shapes that show up in the rendered or plotted image.
/// Everything is hard coded to generate black lines of 1px width, as this is the only relevant
/// setting for the plotter

/// A circle around a center point with a radius.
pub struct Ellipse {
    /// The center point
    pub center: Point,
    /// The radius
    pub radius_x: f32,
    pub radius_y: f32,
}

impl Ellipse {
    /// creates a new circle around a center point with a radius.
    pub fn new(center: Point, radius_x: f32, radius_y: f32) -> Self {
        let ellipse = Ellipse {
            center: center,
            radius_x: radius_x,
            radius_y: radius_y,
        };
        ellipse
    }

    /// Creates an svg path for the ellipse
    pub fn draw(&self) -> element::Path {
        unimplemented!()
        // let circle = CirclePath::new()
        //     .set("fill", "none")
        //     .set("stroke", "black")
        //     .set("stroke-width", 1)
        //     .set("cx", self.center.x)
        //     .set("cy", self.center.y)
        //     .set("r", self.radius);
        // circle
    }
}

impl Shape for Ellipse {
    fn return_center(&self) -> Point {
        self.center
    }

    /// Returns true, if a given point is within the area of the ellipse.
    fn contains(&self, point: Point) -> bool {
        // println!("center: {:?} point: {:?}", self.center, point);

        if ((point.x - self.center.x).powi(2) / (self.radius_x.powi(2)))
            + ((point.y - self.center.y).powi(2) / (self.radius_y.powi(2)))
            <= 1.0
        {
            true
        } else {
            // println!("false");
            false
        }
    }

    /// Returns the point where a line from the starting point to the center of
    /// the circle hits the circle boundary.
    ///
    /// ⚠️ If the provided line does not go through the center of the circle,
    /// the point returned will be off!
    fn intersection(&self, line: Line, step: f32) -> Option<Point> {
        println!("step {}", step);

        let mut diff = 10000000000.0;
        // println!("\n\nstep {}", step);
        // println!("line slope {:?}", line.slope());
        if self.center.x == line.start.x {
            println!("x=x");
            if self.center.y > line.start.y {
                let point = Point::new(self.center.x, self.center.y - self.radius_y);
                return Some(point);
            } else {
                let point = Point::new(self.center.x, self.center.y + self.radius_y);
                return Some(point);
            }
        }
        if self.contains(line.start) && self.contains(line.end) {
            println!("Line completely inside");
            return None;
        }
        let iter_min = smallest_x(self.center.x, line.start.x);
        let iter_max = 800 * ((1.0 / step) as i32);

        for i in (0..iter_max).map(|x| x as f32 * step) {
            let x = iter_min + i;

            if let Some(point_1) = line.return_point_on_line(x) {
                // println!("x: {}, y: {}", point_1.x, point_1.y);
                let number = ((point_1.x - self.center.x).powi(2) / (self.radius_x.powi(2)))
                    + ((point_1.y - self.center.y).powi(2) / (self.radius_y.powi(2)));
                println!("i:{}, number: {}", i, number);

                if self.center.x > line.start.x {
                    println!("left to center");
                    if number < 0.6 {
                        println!("too far on the inside");
                        return self.intersection(line, step / 10.0);
                    } else if number > 1.4 {
                        if diff > self.center.distance_to(&point_1) {
                            diff = self.center.distance_to(&point_1);
                            println!("on the outside, dist reassign");
                        } else {
                            // neuer startpunkt, davon neue linie, die dann als other geht
                            println!("restart");
                            return self.intersection(line, step / 10.0);
                        }
                    } else {
                        let point = Point {
                            x: x as f32,
                            y: point_1.y,
                        };
                        return Some(point);
                    }
                } else if self.center.x < line.start.x {
                    println!("center to right");
                    if self.contains(point_1) == false {
                        if number > 0.6 {
                            // println!("out of circle");
                            let point = Point {
                                x: x as f32,
                                y: point_1.y,
                            };
                            return Some(point);
                            // if let Some(point) = line.return_point_on_line(x - step){
                            //     return Some(point)
                            // }
                        } else if number < 1.4 {
                            return self.intersection(line, step / 10.0);
                        }
                    }
                } else {
                }
            }
        }
        None
    }
}
