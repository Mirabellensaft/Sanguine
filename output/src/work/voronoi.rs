use sanguine_lib::resources::{
    composition::{Composition, Density},
    layout::{voronoi::VoronoiDiagram, Layout},
};
use svg::{node, Node};

use super::star_burst_lib::voronoi_comp::MyVoronoiDiagram;

pub fn form_group(work: VoronoiDiagram) -> node::element::Group {
    // pub fn form_group() {
    let mut graph = node::element::Group::new();
    let mut my_work = MyVoronoiDiagram(work);
    my_work.filled(&Density::Mid);

    for cell in my_work.0.get_points() {
        for line in &cell.border_lines {
            graph.append(line.draw());
        }
    }
    graph
}
