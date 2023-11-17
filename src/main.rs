use svg::node;
use svg::{Document, Node};

use rand::{thread_rng, Rng};

mod lib;
use lib::layout;
use lib::random_numbers;
use lib::shapes;
use lib::math;

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

            let center = random_numbers::coordinate(&layout.field_container[row as usize][col as usize], radius);
        
    
            let circle =shapes::circle(center, radius);
            graph.append(circle);
                   
            let coordinates = random_numbers::coordinates_on_border(&layout.field_container[row as usize][col as usize]);

            for i in 0..40 {

                let line = math::Line::new(coordinates[i], center);
                
                let line = shapes::line(line);
                graph.append(line);
                
            }
                    
        }
    }

    graph

}








