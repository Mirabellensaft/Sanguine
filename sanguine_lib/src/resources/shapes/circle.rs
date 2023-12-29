use crate::resources::shapes::{line::Line, point::Point};
use svg::node::element::Circle as CirclePath;

use super::{smaller_value, Shape};

/// This module contains types related to shapes that show up in the rendered or plotted image.
/// Everything is hard coded to generate black lines of 1px width, as this is the only relevant
/// setting for the plotter

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

    pub fn debug_draw(&self, color: &str) -> CirclePath {
        let circle = CirclePath::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", 1)
            .set("cx", self.center.x)
            .set("cy", self.center.y)
            .set("r", self.radius);
        circle
    }
}

impl Shape for Circle {
    fn return_center(&self) -> Point {
        self.center
    }

    /// Returns true, if a given point is within the area of the circle.
    fn contains(&self, point: Point) -> bool {
        self.center.distance_to(&point) <= self.radius
    }

    /// Returns the point where a line from the starting point to the center of
    /// the circle hits the circle boundary.
    ///
    /// ⚠️ If the provided line does not go through the center of the circle,
    /// the point returned will be off!
    fn intersection(&self, line: Line, step: f32) -> Option<Point> {
        // println!("Center: {:?}, Line: {:?}, step: {}", self.center, line, step);
        let mut diff = 10000000000.0;
        if step < 0.000001 {
            // println!("step {}", step);
            // println!("step too small");
            return None;
        }
        // println!("line slope {:?}", line.slope());
        if ((self.center.x - 0.2)..=(self.center.x + 0.2)).contains(&line.start.x) {
            // println!("x=x");
            if self.center.y > line.start.y {
                let point = Point::new(self.center.x, self.center.y - self.radius);
                // println!("Endpoint: {:?},", point);
                return Some(point);
            } else {
                let point = Point::new(self.center.x, self.center.y + self.radius);
                // println!("Endpoint: {:?},", point);
                return Some(point);
            }
        }

        let mut iter_min = self.center.x;

        if let Some(min) = smaller_value(self.center.x, line.start.x) {
            iter_min = min.0;
        }

        let iter_max = 800 * ((1.0 / step) as i32);

        for i in (0..iter_max).map(|x| x as f32 * step) {
            let x = iter_min + i;

            if let Some(point_1) = line.return_point_from_x(x) {
                // println!("x: {}, y: {}", point_1.x, point_1.y);

                // line starts from the outside, left of the center of the circle
                if self.center.x > line.start.x {
                    // println!("left to center");
                    // circle contains the point
                    if self.contains(point_1) {
                        // distance from the point to the center is smaller then radius - 0.5 aka the
                        // step over the circle boundary to the inside was too big
                        if self.center.distance_to(&point_1) < self.radius - 0.5 {
                            // println!("recurse step/10: {}", step/10.0);
                            return self.intersection(line, step / 10.0);

                        // it was not too big but is inside, so we return the point as it is good enough.
                        } else {
                            let point = Point {
                                x: x as f32,
                                y: point_1.y,
                            };
                            // println!("Endpoint: {:?},", point);
                            return Some(point);
                        }

                    // the circle does not yet contain the point
                    } else {
                        // if in the previous iteration the distance between center and point was bigger then now
                        // we have not yet overstepped the circle center to the outside of the circle, so we assign
                        // the new difference, and go into the next iteration
                        if diff > self.center.distance_to(&point_1) {
                            diff = self.center.distance_to(&point_1)
                        } else {
                            // we have shot out of the other side of the circle, so we restart with a more fine grained
                            // println!("recurse step/10: {}", step/10.0);
                            return self.intersection(line, step / 10.0);
                        }
                    }
                } else if self.center.x < line.start.x {
                    // println!("center to right");
                    if self.contains(point_1) == false {
                        if self.center.distance_to(&point_1) < self.radius + 0.5 {
                            // println!("out of circle");
                            let point = Point {
                                x: x as f32,
                                y: point_1.y,
                            };
                            // println!("Endpoint: {:?},", point);
                            return Some(point);
                            // if let Some(point) = line.return_point_on_line(x - step){
                            //     return Some(point)
                            // }
                        } else {
                            // println!("recurse step/10: {}", step/10.0);
                            return self.intersection(line, step / 10.0);
                        }
                    }
                } else {
                    // println!("empty else");
                }
            }
        }
        None
    }
}

#[cfg(test)]
#[test]

fn distance_test() {
    let point_1 = Point::new(1.0, 10.0);
    let point_2 = Point::new(10.0, 10.0);

    assert_eq!(point_1.distance_to(&point_2), 9.0);
}
#[test]
fn circle_1() {
    let point_1 = Point::new(5.0, 5.0);
    let point_2 = Point::new(1.0, 1.0);
    let circle = Circle::new(point_1, 2.0);

    assert_eq!(circle.contains(point_2), false);
}
#[test]
fn circle_2() {
    let point_1 = Point::new(5.0, 5.0);
    let point_2 = Point::new(4.0, 4.0);
    let circle = Circle::new(point_1, 2.0);

    assert_eq!(circle.contains(point_2), true);
}
#[test]
fn circle_3() {
    let point_1 = Point::new(5.0, 5.0);
    let point_2 = Point::new(3.0, 5.0);
    let circle = Circle::new(point_1, 2.0);

    assert_eq!(circle.contains(point_2), true);
}

#[test]
fn circle_4() {
    let point_1 = Point::new(5.0, 5.0);
    let point_2 = Point::new(3.0, 5.0);
    let circle = Circle::new(point_1, 1.9);

    assert_eq!(circle.contains(point_2), false);
}
#[test]

fn circle_5() {
    let point_1 = Point::new(2.0, 4.0);
    //center and end of line
    let point_2 = Point::new(2.0, 2.0);
    let circle = Circle::new(point_2, 1.0);
    let line = Line::new(point_1, point_2);

    let expected = Point::new(2.0, 3.0);

    assert_eq!(circle.intersection(line, 1.0).unwrap(), expected);
}

#[test]
fn circle_6() {
    let point_1 = Point::new(2.5, 2.5);
    //center and end of line
    let point_2 = Point::new(10.0, 10.0);
    let circle = Circle::new(point_2, 5.0);
    let line = Line::new(point_1, point_2);

    let expected = Point::new(6.5, 6.5);

    assert_eq!(circle.intersection(line, 1.0).unwrap(), expected);
}
