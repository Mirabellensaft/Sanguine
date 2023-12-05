
use sanguine_lib::resources::{layout, composition};
use svg::{node, Node};

pub fn form_group(layout: &layout::Grid) -> node::element::Group {
// pub fn form_group() {
    let mut graph = node::element::Group::new();

    let voi_diagram = composition::voronoi::VoronoiDiagram::new_uniform(layout, 100);

    for cell in voi_diagram.cells {
        for line in cell.0 {
            graph.append(line.draw()); 

        }

    }
    graph
}

