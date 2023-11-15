use svg::node::element::Path;
use svg::node::{self, element::path::Data};
use svg::{Document, Node};

use rand::{thread_rng, Rng};

mod lib;
use lib::layout;
use lib::random_numbers;
use lib::shapes;

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
    
            let circle =shapes::circle(cx, cy, radius);
            graph.append(circle);
                   
            let coordinates = random_numbers::coordinates_on_border(&layout.field_container[row as usize][col as usize]);

            for i in 0..40 {
                let line = shapes::line(coordinates[i].0, coordinates[i].1, cx, cy);
                graph.append(line);
                
            }
                    
        }
    }

    graph

}








