use rand::Rng;
use std::f64::consts::PI;
use svg::node::element::path::Data;
use svg::node::element::Path;

use crate::resources::shapes::{path, point::Point};

pub struct Circloid {
    pub center: Point,
    pub base_radius: f32,
    /// 2.0..8.0
    pub amplitude: f32,
    /// 1.2..3.0
    pub frequency: f32,
    pub point_collection: Vec<Point>,
}

impl Circloid {
    pub fn new(center: Point, base_radius: f32, amplitude: f32, frequency: f32) -> Self {
        // Generate a random starting angle between 0 and 2*pi
        let mut rng = rand::thread_rng();
        let random_start_angle: f64 = rng.gen_range(0.0..2.0 * PI);
        let num_points = 1000;
        let mut point_collection: Vec<Point> = vec![];

        // 2/3 of the shape will be a sine wave
        let sine_fraction = 2.0 / 3.0;
        let sine_wave_points = (num_points as f64 * sine_fraction).round() as usize;

        for i in 0..=sine_wave_points {
            let t = sine_fraction * 2.0 * PI * (i as f64 / sine_wave_points as f64); // Angle for sine wave
            let angle = random_start_angle + t; // Apply the random starting angle

            // Gradually reduce amplitude as we approach the end of the sine wave
            let transition_factor = if i as f64 / sine_wave_points as f64 > 0.8 {
                // Reduce amplitude for a smooth transition (last 20% of sine)
                1.0 - ((i as f64 / sine_wave_points as f64) - 0.8) / 0.2
            } else {
                1.0
            };

            let modulated_amplitude = amplitude as f64 * transition_factor;

            // Modulate the radius using sine wave
            let modulated_radius =
                base_radius as f64 + modulated_amplitude * ((frequency * t as f32).sin()) as f64;

            // Parametric circle with modulated radius
            let x = center.x + (modulated_radius * angle.cos()) as f32;
            let y = center.y + (modulated_radius * angle.sin()) as f32;

            // Store the first point and move to it
            point_collection.push(Point { x, y });
        }

        // Arc that covers the remaining 1/3 of the shape
        let arc_radius = base_radius;
        let arc_start_angle = sine_fraction * 2.0 * PI;
        let arc_end_angle = 2.0 * PI;

        for i in 0..=(num_points - sine_wave_points) {
            let t = arc_start_angle
                + (i as f64 / (num_points - sine_wave_points) as f64)
                    * (arc_end_angle - arc_start_angle);
            let angle = random_start_angle + t; // Apply the same random starting angle to the arc

            // Simple circular arc for the remaining 1/3
            let x = center.x + arc_radius * angle.cos() as f32;
            let y = center.y + arc_radius * angle.sin() as f32;

            // Draw the line for the arc
            point_collection.push(Point { x, y });
            // data = data.line_to((x, y));
        }

        let circloid = Circloid {
            center: center,
            base_radius: base_radius,
            amplitude: amplitude, // 2.0..8.0
            frequency: frequency, //1.2..3.0
            point_collection: point_collection,
        };
        circloid
    }

    pub fn draw(&self) -> Path {
        let mut data = Data::new();
        let mut i = 0;

        for point in &self.point_collection {
            if i == 0 {
                data = data.move_to((point.x, point.y));
                i += 1;
            } else {
                data = data.line_to((point.x, point.y));
            }
        }

        // Close the path to complete the circle
        data = data.close();

        let path = path(data);
        path
    }
}
