use svg::node::element::{Path, Circle};
use svg::node::{self, element::path::Data};
use svg::{Document, Node};

use rand::{thread_rng, Rng};

mod lib;
use lib::layout::{self, Field};
use lib::random_numbers;

fn main() {

    let work = layout::Format::new(300, 300, 5, 5, 5);
    let document = Document::new()
        .set("viewBox", (0, 0, work.width, work.height))
        .add(form_group(&work));

    svg::save("image.svg", &document).unwrap();
}

fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();

    for row in 0..layout.rows {
        for col in 0..layout.columns {

            let mut rng = thread_rng();
            let radius = rng.gen_range(0..=10);

            let cx = random_numbers::coordinate(&layout.field_container[row as usize][col as usize]).0;
            let cy= random_numbers::coordinate(&layout.field_container[row as usize][col as usize]).1;
    
            let circle = Circle::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("cx", cx)
                .set("cy", cy)
                .set("r", radius);
            graph.append(circle);
                   
            let coordinates = random_numbers::coordinates_on_border(&layout.field_container[row as usize][col as usize]);

            for i in 0..40 {
                let data = Data::new()
                    .move_to((coordinates[i].0, coordinates[i].1))
                    .line_to((cx, cy));

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 1)
                    .set("d", data);
                
                graph.append(path);
                
            }
                    
                }
            }
    graph
}



fn distorted_square(field: layout::Field) -> Path {
    let data = Data::new()
        .move_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .line_to((random_numbers::coordinate(&field).0, random_numbers::coordinate(&field).1))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);
    path
}





