use rand::{thread_rng, Rng};
use svg::node::element::Group;

use sanguine_lib::resources::{
    border_coordinates::AllBorderCoordinates,
    composition::grid::{CompositionCenter, CompositionOverlay, Density},
    layout,
};

use super::star_burst_lib;

const RADIUS_MID: std::ops::RangeInclusive<i32> = 3_i32..=6_i32;
const RADIUS_HIGH: std::ops::RangeInclusive<i32> = 5_i32..=10_i32;
const RADIUS_FOCUS: std::ops::RangeInclusive<i32> = 10_i32..=20_i32;

pub fn form_group(work: &layout::Work) -> Group {
    let mut graph = Group::new();

    // Creates a baseline composition
    let mut comp = CompositionOverlay::new_empty(layout);
    comp.add_center(CompositionCenter::Bottom, layout);
    comp.add_random_low(30, layout);
    comp.add_random_center(6, layout);
    comp.connect_centers();
    comp.add_random_low(10, layout);

    let mut all_coords = AllBorderCoordinates::new(layout, 10);
    all_coords.tesselate();
    all_coords.slight_chaos();

    // Fills the gaps and edges in the baseline composition
    comp.retro_composition(layout);

    // Drawing of the Elements
    for row in 0..layout.rows {
        for col in 0..layout.columns {
            let mut rng = thread_rng();

            let mut radius = 0;

            match comp.0[row][col] {
                Density::Mid => radius = rng.gen_range(RADIUS_MID),
                Density::High => radius = rng.gen_range(RADIUS_HIGH),
                Density::Focus => radius = rng.gen_range(RADIUS_FOCUS),
                Density::Edge(_) => radius = rng.gen_range(RADIUS_MID),
                Density::ThreeWay(_) => radius = rng.gen_range(RADIUS_MID),
                _ => (),
            }

            graph = star_burst_lib::draw::everything(
                comp.0[row][col],
                &all_coords.0[row][col],
                &layout.field_container[row as usize][col as usize],
                radius,
                graph,
            );
        }
    }
    graph
}
