use rand::Rng;
use sanguine_lib::resources::layout::{voronoi::VoronoiDiagram, Layout};
use sanguine_lib::resources::shapes::circloid::Circloid;
use svg::{node, Node};

pub fn form_group(work: &VoronoiDiagram) -> node::element::Group {
    // pub fn form_group() {
    let mut graph = node::element::Group::new();
    let mut rng = rand::thread_rng();

    for cell in work.get_cells() {
        for i in 0..5 {
            let circloid = Circloid::new(
                cell.center,
                rng.gen_range(25.0..50.0),
                rng.gen_range(2.0..8.0),
                rng.gen_range(1.2..3.0),
            );

            graph.append(circloid.draw());
        }
    }
    graph
}
