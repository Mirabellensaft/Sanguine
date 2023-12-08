use sanguine_lib::resources::{composition::voronoi::{VoronoiType, VoronoiDiagram}, layout};
use svg::{node, Node};

pub fn form_group(layout: &layout::Grid) -> node::element::Group {
    // pub fn form_group() {
    let mut graph = node::element::Group::new();

    let voi_diagram = VoronoiDiagram::new(layout, VoronoiType::Uniform(100));

    for cell in voi_diagram.cells {
        for line in cell.border_lines {
            graph.append(line.draw());
        }
    }
    graph
}
