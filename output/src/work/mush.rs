use rand::{seq::SliceRandom, thread_rng, Rng};

use sanguine_lib::resources::{
    border_coordinates::all::AllBorderCoordinates,
    composition::{Composition, CompositionCenter, Density},
    layout::{grid::Grid, Layout},
    shapes::{circle::Circle, curve::Curve, line::Line, point::Point},
};

use std::f64::consts::PI;
use svg::{node::element::Group, Node};

pub fn form_group(work: Grid) -> Group {
    let mut graph = Group::new();
    let mut rng = &mut rand::thread_rng();
    let center = Point::new(250.0, 250.0);
    let inner_radius = rng.gen_range(30.0..60.0);
    let outer_radius = rng.gen_range(80.0..230.0);
    let num_lines = 150;

    // Add inner and outer circles
    let inner_circle = Circle::new(center, inner_radius);
    // graph.append(inner_circle.draw());

    let outer_circle = Circle::new(center, outer_radius);
    // graph.append(outer_circle.draw());

    // Add wiggly lines between the circles
    for i in 0..num_lines {
        let mut rng = &mut rand::thread_rng();
        let angle = (i as f64 / num_lines as f64) * 2.0 * PI;

        // Start point on the inner circle
        let start = Point::new(
            center.x + (inner_radius * angle.cos() as f32),
            center.y + (inner_radius * angle.sin() as f32),
        );

        // End point on the outer circle
        let end = Point::new(
            center.x + (outer_radius * angle.cos() as f32),
            center.y + (outer_radius * angle.sin() as f32),
        );

        // Control points for wiggle effect (random variations)

        let cp_start = Point::new(
            start.x + rng.gen_range(-5.0..5.0),
            start.y + rng.gen_range(-5.0..5.0),
        );

        let cp_end = Point::new(
            end.x + rng.gen_range(-5.0..5.0),
            end.y + rng.gen_range(-5.0..5.0),
        );

        // Create a wiggly line path
        let wiggle = Curve::new(start, cp_start, cp_end, end);
        graph.append(wiggle.draw())
    }

    graph
}
