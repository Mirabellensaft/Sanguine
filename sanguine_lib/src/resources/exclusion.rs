use svg::node::element::path::{Command, Data};
use svg::parser::Event;

use crate::resources::shapes::ellipse::{Ellipse, self};
use crate::resources::shapes::point::Point;

pub struct Exclusion(pub Vec<Ellipse>);

impl Exclusion {
    pub fn make_exclusion(path: &str, content: &mut String) -> Option<Self> {

        let mut vec = Vec::new();
    
        for event in svg::open(path, content).unwrap() {
            println!("{:?}\n", event);
            match event {
                Event::Tag(path, _, attributes) => {
    
                    // println!("attributes {:?}\n", attributes);
                    // if let Some(data) = attributes.get("d") {
                    //     println!("data {:?}", data);
    
    
                    match path {
                        "ellipse" => {
                            let cx: f32 = attributes.get("cx").unwrap().parse().expect("Not a valid number");
                            let cy: f32 = attributes.get("cy").unwrap().parse().expect("Not a valid number");
                            let rx: f32 = attributes.get("rx").unwrap().parse().expect("Not a valid number");
                            let ry: f32 = attributes.get("ry").unwrap().parse().expect("Not a valid number");

                            println!("cy: {} cx: {}, rx: {}, ry: {}", cx, cy, rx, ry);
    
                            let ellipse = Ellipse::new(
                                Point::new(cx, cy), rx, ry
                            );

                            vec.push(ellipse);
    
                            
                        
                        },
                        _ => {},
                    }
                    // let data = Data::parse(data).unwrap();
                    
                    // for command in data.iter() {
                    //     match command {
                    //         &Command::Move(..) => {
                    //              /* … */ 
                    //             },
                    //         &Command::Line(..) => {
                    //              /* … */ 
                    //             },
                    //         &Command::EllipticalArc(..) => {
                    //             println!("ElliptivalArc");
                    //         }
                    //         _ => {}
                    //     }
                    // }
    
                    }
                    
                
                _ => {}
            }
            
        }
        Some(Exclusion(vec))
    }
    
    pub fn exclude(&self) {

    }

}
