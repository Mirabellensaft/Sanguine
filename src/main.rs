use svg::node::element::Path;
use svg::node::{self, element::path::Data};
use svg::{Document, Node};

use rand::{thread_rng, Rng};

mod lib;
use lib::layout;

fn main() {

    let work = layout::Format::new(300, 300, 5, 3, 3);
    let document = Document::new()
        .set("viewBox", (0, 0, work.width, work.height))
        .add(form_group(&work))
        .add(form_group(&work));

    svg::save("image.svg", &document).unwrap();
}

fn form_group(layout: &layout::Format) -> node::element::Group {
    let mut graph = node::element::Group::new();
    for row in 0..layout.rows {
        for col in 0..layout.columns {
            let path = random_square(layout.field_container[row as usize][col as usize]);
            graph.append(path)
        }
    }
    graph
}

fn return_random(field: &layout::Field) -> (i32, i32) {
    let mut rng = thread_rng();

    let horizontal: std::ops::Range<i32> = std::ops::Range {
        start: field.x,
        end: field.x + field.column_width,
    };

    let vertical: std::ops::Range<i32> = std::ops::Range {
        start: field.y,
        end: field.y + field.row_height,
    };
    
    let randoms = (rng.gen_range(horizontal), rng.gen_range(vertical));
    randoms
}

fn random_square(field: layout::Field) -> Path {
    let data = Data::new()
        .move_to((return_random(&field).0, return_random(&field).1))
        .line_to((return_random(&field).0, return_random(&field).1))
        .line_to((return_random(&field).0, return_random(&field).1))
        .line_to((return_random(&field).0, return_random(&field).1))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);
    path
}
