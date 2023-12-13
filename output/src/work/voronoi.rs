use sanguine_lib::resources::{
    composition::{Composition, Density},
    layout::{voronoi::VoronoiDiagram, Layout},
};
use svg::{node, Node};

pub fn form_group(work: &mut VoronoiDiagram) -> node::element::Group {
    // pub fn form_group() {
    let mut graph = node::element::Group::new();

    work.filled(Density::Mid);

    for cell in work.get_points() {
        for line in &cell.border_lines {
            graph.append(line.draw());
        }
    }
    graph
}
