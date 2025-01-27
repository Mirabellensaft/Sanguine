use rand::{thread_rng, Rng};
use svg::node::element::Group;

use sanguine_lib::resources::{
    border_coordinates::all::AllBorderCoordinates,
    composition::{Composition, CompositionCenter, Density},
    layout::{grid::Grid, Layout},
};

use super::star_burst_lib::{self, grid_comp::MyGrid};

const RADIUS_MID: std::ops::RangeInclusive<i32> = 3_i32..=6_i32;
const RADIUS_HIGH: std::ops::RangeInclusive<i32> = 5_i32..=10_i32;
const RADIUS_FOCUS: std::ops::RangeInclusive<i32> = 10_i32..=20_i32;

pub fn form_group(work: Grid) -> Group {
    let mut graph = Group::new();
    let mut my_work = MyGrid(work);

    // Creates a baseline composition
    my_work.add_center(CompositionCenter::Bottom);
    my_work.add_random_low(30);
    my_work.add_random_center(6);
    my_work.connect_centers();
    my_work.add_random_low(10);

    let mut all_coords = AllBorderCoordinates::new(&my_work.0, 10);
    all_coords.tesselate();
    all_coords.slight_chaos();

    // Fills the gaps and edges in the baseline composition
    my_work.retro_composition();

    // Drawing of the Elements
    for row in 0..my_work.0.get_rows() {
        for col in 0..my_work.0.get_columns() {
            let mut rng = thread_rng();

            let mut radius = 0;

            match my_work.0.container[row][col].density {
                Density::Mid => radius = rng.gen_range(RADIUS_MID),
                Density::High => radius = rng.gen_range(RADIUS_HIGH),
                Density::Focus => radius = rng.gen_range(RADIUS_FOCUS),
                Density::Edge(_) => radius = rng.gen_range(RADIUS_MID),
                Density::ThreeWay(_) => radius = rng.gen_range(RADIUS_MID),
                _ => (),
            }
            let tile = &my_work.0.get_tiles()[row as usize][col as usize];
            println!(
                "tile: x{}, y{}, width{}, height{}",
                tile.x, tile.y, tile.width, tile.height
            );
            graph = star_burst_lib::draw::everything(
                my_work.0.container[row][col].density.clone(),
                &all_coords.0[row][col],
                tile,
                radius,
                graph,
            );
        }
    }
    graph
}
