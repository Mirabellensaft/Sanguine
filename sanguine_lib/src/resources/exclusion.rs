use svg::node::element::path::{Command, Data};
use svg::parser::Event;

use super::shapes::Shape;
use crate::resources::shapes::{circle::Circle, ellipse::Ellipse, point::Point};

/// This module defines the Exclusion type. Currently it is a parser that can parse a limited
/// selection of shapes from SVG files it is given. In the future, these files will come from
/// a vectorized camera input from a camera directly above the plotter drawing area, so that
/// this framework has the ability to take into account what already is on the paper.
pub struct Exclusion(pub Vec<Box<dyn Shape>>);

impl Exclusion {
    pub fn make_exclusion(path: &str, content: &mut String) -> Option<Self> {
        let mut vec: Vec<Box<dyn Shape>> = Vec::new();

        for event in svg::open(path, content).unwrap() {
            // println!("{:?}\n", event);
            match event {
                Event::Tag(path, _, attributes) => {
                    match path {
                        "ellipse" => {
                            let cx: f32 = attributes
                                .get("cx")
                                .unwrap()
                                .parse()
                                .expect("Not a valid number");
                            let cy: f32 = attributes
                                .get("cy")
                                .unwrap()
                                .parse()
                                .expect("Not a valid number");
                            let rx: f32 = attributes
                                .get("rx")
                                .unwrap()
                                .parse()
                                .expect("Not a valid number");
                            let ry: f32 = attributes
                                .get("ry")
                                .unwrap()
                                .parse()
                                .expect("Not a valid number");

                            // println!("cy: {} cx: {}, rx: {}, ry: {}", cx, cy, rx, ry);

                            let ellipse = Ellipse::new(Point::new(cx, cy), rx, ry);
                            let b = Box::new(ellipse);
                            vec.push(b);
                        }
                        "circle" => {
                            let cx: f32 = attributes
                                .get("cx")
                                .unwrap()
                                .parse()
                                .expect("Not a valid number");
                            let cy: f32 = attributes
                                .get("cy")
                                .unwrap()
                                .parse()
                                .expect("Not a valid number");
                            let r: f32 = attributes
                                .get("rx")
                                .unwrap()
                                .parse()
                                .expect("Not a valid number");

                            // println!("cy: {} cx: {}, r: {}", cx, cy, r);

                            let circle = Circle::new(Point::new(cx, cy), r);
                            let b = Box::new(circle);
                            vec.push(b);
                        }
                        _ => {}
                    }

                    // println!("attributes {:?}\n", attributes);
                    if let Some(data) = attributes.get("d") {
                        // println!("data {:?}", data);
                        let data = Data::parse(data).unwrap();

                        for command in data.iter() {
                            match command {
                                &Command::Move(..) => { /* … */ }
                                &Command::Line(..) => { /* … */ }
                                &Command::EllipticalArc(..) => {
                                    // println!("EllipticalArc");
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Some(Exclusion(vec))
    }

    pub fn exclude(&self) {}
}
