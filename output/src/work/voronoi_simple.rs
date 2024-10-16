use sanguine_lib::resources::layout::{voronoi::VoronoiDiagram, Layout};
use svg::{node, Node};

pub fn form_group(work: &VoronoiDiagram) -> node::element::Group {
    // pub fn form_group() {
    let mut graph = node::element::Group::new();

    for cell in work.get_cells() {
        for line in &cell.border_lines {
            graph.append(line.draw());
        }
    }
    graph
}
