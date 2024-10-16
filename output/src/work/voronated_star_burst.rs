use rand::{thread_rng, Rng};
use svg::{node::element::Group, Node};

use sanguine_lib::resources::{
    border_coordinates::all::AllBorderCoordinates,
    composition::Density,
    layout::{voronoi::VoronoiDiagram, Layout},
    shapes::circle::Circle,
};

use super::star_burst_lib::lines;

const RADIUS_MID: std::ops::RangeInclusive<i32> = 3_i32..=6_i32;
const RADIUS_HIGH: std::ops::RangeInclusive<i32> = 5_i32..=10_i32;
const RADIUS_FOCUS: std::ops::RangeInclusive<i32> = 10_i32..=20_i32;

pub fn form_group(work: &mut VoronoiDiagram) -> Group {
    let mut graph = Group::new();
    let mut rng = thread_rng();

    // Creates a baseline composition
    // work.add_center(CompositionCenter::Bottom);
    // work.add_random_low(30);
    // work.add_random_center(6);
    // work.connect_centers();
    // work.add_random_low(10);

    let mut all_coords = AllBorderCoordinates::new_from_voronoi(work, 6);
    all_coords.tesselate_voronoi(&work);
    // all_coords.slight_chaos();

    // Fills the gaps and edges in the baseline composition
    // work.retro_composition();
    println!("cmp starts");
    for cell in 0..work.cells.len() {
        if work.cells[cell].center.y > 1800.0 {
            let truth = rng.gen_bool(1.0 / 3.0);
            if truth {
                work.cells[cell].density = Density::Mid;
            } else {
                work.cells[cell].density = Density::High;
            }
        } else {
            let truth = rng.gen_bool(1.0 / 8.0);
            if truth {
                work.cells[cell].density = Density::Empty;
            } else {
                work.cells[cell].density = Density::Mid;
            }
        }
    }
    println!("drawing starts");
    // Drawing of the Elements
    // println!("len cells {}", work.cells.len());
    for cell in 0..work.cells.len() {
        // println!("cell center {:?}", work.cells[cell].center);

        let mut radius = 0;

        match work.cells[cell].density {
            Density::Mid => radius = rng.gen_range(RADIUS_MID),
            Density::High => radius = rng.gen_range(RADIUS_HIGH),
            Density::Focus => radius = rng.gen_range(RADIUS_FOCUS),
            Density::Edge(_) => radius = rng.gen_range(RADIUS_MID),
            Density::ThreeWay(_) => radius = rng.gen_range(RADIUS_MID),
            _ => (),
        }

        match work.cells[cell].density {
            Density::Mid | Density::High => {
                let circle = Circle::new(work.cells[cell].center, radius as f32);
                graph.append(circle.draw());

                for side in &all_coords.0[0][cell].0 {
                    lines::to_circle(&mut graph, &side, &circle, 0, side.0.len());
                }
            }
            _ => (),
        }
    }
    for cell in work.get_cells() {
        for line in &cell.border_lines {
            graph.append(line.draw());
        }
    }

    graph
}
