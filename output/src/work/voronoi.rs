use sanguine_lib::resources::layout;
use svg::{node, Node};

pub fn form_group(work: &layout::Work) -> node::element::Group {
    // pub fn form_group() {
    let mut graph = node::element::Group::new();
    
    for cell in work.0.get_points() {
        for line in &cell.border_lines {
            graph.append(line.draw());
        }
    }
    graph
}
