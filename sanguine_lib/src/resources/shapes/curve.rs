use rand::seq::SliceRandom;
use svg::node::element::path::Data;
use svg::node::element::Path;

use crate::resources::shapes::{path, point::Point};

use super::range;

/// This module contains types related to shapes that show up in the rendered or plotted image.
/// Everything is hard coded to generate black curves of 1px width, as this is the only relevant
/// setting for the plotter

/// A curve between two points.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Curve {
    /// starting point
    pub start: Point,
    /// control point near start
    pub control_point_start: Point,
    /// control point near end
    pub control_point_end: Point,
    /// end point
    pub end: Point,
}

impl Curve {
    /// Creates a new curve between two given points.
    pub fn new(start: Point, cp_start: Point, cp_end: Point, end: Point) -> Self {
        let curve: Curve = Curve {
            start: start,
            control_point_start: cp_start,
            control_point_end: cp_end,
            end: end,
        };

        curve
    }

    /// Returns the point on a curve for a given x value. Returns none if there
    /// is no point for that x value, usually the case when the curve is vertical.
    pub fn return_point_from_x(&self, _x: f32) -> Option<Point> {
        unimplemented!()
    }

    pub fn return_point_from_y(&self, _y: f32) -> Option<Point> {
        unimplemented!()
    }

    /// Calculates the slope of a curve, returns none the curve is vertical
    pub fn slope(&self) -> Option<f32> {
        unimplemented!()
    }

    /// Returns the y intercept of a curve. Returns None if the curve is vertical and
    /// does not cross the y-axis.
    pub fn y_intercept(&self) -> Option<f32> {
        unimplemented!()
    }

    /// Returns an amount of random, non duplicate points on a curve
    pub fn random_points(&self, _amount: usize) -> Vec<Point> {
        unimplemented!()
    }

    pub fn equal(&self, other: Curve) -> bool {
        if *self == other {
            true
        } else if self.start == other.end && self.end == other.start {
            true
        } else {
            false
        }
    }

    /// Creates an svg path for a curve
    pub fn draw(&self) -> Path {
        let data = Data::new()
            .move_to((self.start.x, self.start.y))
            .cubic_curve_to((
                self.control_point_start.x,
                self.control_point_start.y,
                self.control_point_end.x,
                self.control_point_end.y,
                self.end.x,
                self.end.y,
            ));

        let path = path(data);
        path
    }
}

// impl Shape for Curve {
//     fn return_center(&self) -> Point {
//         unimplemented!()
//     }

//     fn contains(&self, _point: Point) -> bool {
//         unimplemented!()
//     }

//     /// Returns the point where two curves intersect, returns None, if they don't intersect.
//     fn intersection(&self, other: Curve, step: f32) -> Option<Point> {
//         unimplemented!()
//     }
// }
