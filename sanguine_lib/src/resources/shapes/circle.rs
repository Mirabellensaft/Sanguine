use crate::resources::shapes::{line::Line, point::Point};
use svg::node::element::Circle as CirclePath;

use super::Shape;

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
        let mut diff = 10000000000.0;
        // println!("\n\nstep {}", step);
        // println!("line slope {:?}", line.slope());
        if self.center.x == line.start.x {
            // println!("x=x");
            if self.center.y > line.start.y {
                let point = Point::new(self.center.x, self.center.y - self.radius);
                return Some(point);
            } else {
                let point = Point::new(self.center.x, self.center.y + self.radius);
                return Some(point);
            }
        }

        let iter_min = smallest_x(self.center.x, line.start.x);
        let iter_max = 300 * ((1.0 / step) as i32);

        for i in (0..iter_max).map(|x| x as f32 * step) {
            let x = iter_min + i;

            if let Some(point_1) = line.return_point_on_line(x) {
                // println!("x: {}, y: {}", point_1.x, point_1.y);

                // line starts from the outside, left of the center of the circle
                if self.center.x > line.start.x {
                    // circle contains the point
                    if self.contains(point_1) {
                        // distance from the point to the center is smaller then radius - 0.5 aka the
                        // step over the circle boundary to the inside was too big
                        if self.center.distance_to(&point_1) < self.radius - 0.5 {
                            return self.intersection(line, step / 10.0);

                        // it was not too big but is inside, so we return the point as it is good enough.
                        } else {
                            // println!("left to center");
                            let point = Point {
                                x: x as f32,
                                y: point_1.y,
                            };
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
                            return Some(point);
                            // if let Some(point) = line.return_point_on_line(x - step){
                            //     return Some(point)
                            // }
                        } else {
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

/// Helper function that returns a valid range between two values.
fn range(x_1: f32, x_2: f32) -> std::ops::Range<i32> {
    if x_1 > x_2 {
        let range = std::ops::Range {
            start: x_2 as i32,
            end: x_1 as i32,
        };
        return range;
    } else {
        let range = std::ops::Range {
            start: x_1 as i32,
            end: x_2 as i32,
        };

        return range;
    };
}

/// Helper function that returns the lower of two values
pub fn smallest_x(x_1: f32, x_2: f32) -> f32 {
    if x_1 > x_2 {
        x_2
    } else {
        x_1
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
