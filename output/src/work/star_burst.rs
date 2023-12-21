use rand::{thread_rng, Rng};
use svg::node::element::Group;

use sanguine_lib::resources::{
    border_coordinates::all::AllBorderCoordinates,
    composition::{Composition, CompositionCenter, Density},
    layout::{grid::Grid, Layout},
};

use super::star_burst_lib;

const RADIUS_MID: std::ops::RangeInclusive<i32> = 3_i32..=6_i32;
const RADIUS_HIGH: std::ops::RangeInclusive<i32> = 5_i32..=10_i32;
const RADIUS_FOCUS: std::ops::RangeInclusive<i32> = 10_i32..=20_i32;

pub fn form_group(work: &mut Grid) -> Group {
    let mut graph = Group::new();

    // Creates a baseline composition
    work.add_center(CompositionCenter::Bottom);
    work.add_random_low(30);
    work.add_random_center(6);
    work.connect_centers();
    work.add_random_low(10);

    let mut all_coords = AllBorderCoordinates::new(work, 10);
    all_coords.tesselate();
    all_coords.slight_chaos();

    // Fills the gaps and edges in the baseline composition
    work.retro_composition();

    // Drawing of the Elements
    for row in 0..work.get_rows() {
        for col in 0..work.get_columns() {
            let mut rng = thread_rng();

            let mut radius = 0;

            match work.container[row][col].density {
                Density::Mid => radius = rng.gen_range(RADIUS_MID),
                Density::High => radius = rng.gen_range(RADIUS_HIGH),
                Density::Focus => radius = rng.gen_range(RADIUS_FOCUS),
                Density::Edge(_) => radius = rng.gen_range(RADIUS_MID),
                Density::ThreeWay(_) => radius = rng.gen_range(RADIUS_MID),
                _ => (),
            }
            let field = &work.get_fields()[row as usize][col as usize];
            println!(
                "field: x{}, y{}, width{}, height{}",
                field.x, field.y, field.column_width, field.row_height
            );
            graph = star_burst_lib::draw::everything(
                work.container[row][col].density,
                &all_coords.0[row][col],
                field,
                radius,
                graph,
            );
        }
    }
    graph
}
