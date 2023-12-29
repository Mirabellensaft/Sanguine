use rand::{thread_rng, Rng};
use svg::{node::element::Group, Node};

use sanguine_lib::resources::{
    border_coordinates::all::AllBorderCoordinates,
    composition::{Composition, Density},
    layout::{voronoi::VoronoiDiagram, Layout},
};

use crate::work::star_burst_lib;

use super::star_burst_lib::voronoi_comp::MyVoronoiDiagram;
const RADIUS_LOW: std::ops::RangeInclusive<i32> = 3_i32..=5_i32;
const RADIUS_MID: std::ops::RangeInclusive<i32> = 5_i32..=8_i32;
const RADIUS_HIGH: std::ops::RangeInclusive<i32> = 8_i32..=12_i32;
const RADIUS_FOCUS: std::ops::RangeInclusive<i32> = 12_i32..=20_i32;

pub fn form_group(work: VoronoiDiagram) -> Group {
    let mut graph = Group::new();
    let mut rng = thread_rng();

    let mut my_work = MyVoronoiDiagram(work);
    // Creates a baseline composition
    // work.add_center(CompositionCenter::Bottom);
    my_work.add_random_low(30);
    my_work.add_random_center(6);
    // work.connect_centers();
    my_work.add_random_low(10);

    let mut all_coords = AllBorderCoordinates::new_from_voronoi(&my_work.0, 6);
    all_coords.tesselate_voronoi(&my_work.0);
    // all_coords.slight_chaos();

    // Fills the gaps and edges in the baseline composition

    println!("cmp starts {} cells", my_work.0.cells.len());
    for cell in 0..my_work.0.cells.len() {
        if my_work.0.cells[cell].center.y > 1800.0 {
            let truth = rng.gen_bool(1.0 / 2.0);
            if truth {
                my_work.0.cells[cell].density = Density::High;
            } else {
                my_work.0.cells[cell].density = Density::Empty;
            }
        } else {
            let truth = rng.gen_bool(1.0 / 8.0);
            if truth {
                my_work.0.cells[cell].density = Density::Mid;
            } else {
                my_work.0.cells[cell].density = Density::Empty;
            }
        }
    }
    my_work.retro_composition();
    println!("comp starts");
    // Drawing of the Elements
    // println!("len cells {}", work.cells.len());
    for cell in 0..my_work.0.cells.len() {
        // println!("cell center {:?}", work.cells[cell].center);

        let mut radius = 0;

        match my_work.0.cells[cell].density {
            Density::Mid => radius = rng.gen_range(RADIUS_MID),
            Density::High => radius = rng.gen_range(RADIUS_HIGH),
            Density::Focus => radius = rng.gen_range(RADIUS_FOCUS),
            Density::Edge(_) => radius = rng.gen_range(RADIUS_LOW),
            Density::ThreeWay(_) => radius = rng.gen_range(RADIUS_MID),
            Density::Lopsided(_) => radius = rng.gen_range(RADIUS_MID),
            Density::Transition(_) => radius = rng.gen_range(RADIUS_LOW),
            _ => (),
        }
        graph = star_burst_lib::draw_voronoi::everything(
            &my_work.0.cells[cell],
            &all_coords.0[0][cell],
            radius,
            graph,
        );
        // match my_work.0.cells[cell].density {
        //     Density::Mid | Density::High => {
        //         let circle = Circle::new(my_work.0.cells[cell].center, radius as f32);
        //         graph.append(circle.draw());

        //         for side in &all_coords.0[0][cell].0 {
        //             lines::to_circle(&mut graph, &side, &circle, 0, side.0.len());
        //         }
        //     }
        //     _ => (),
        // }
    }

    for cell in my_work.0.get_points() {
        for line in &cell.border_lines {
            graph.append(line.debug_draw("red"));
        }
    }

    graph
}
