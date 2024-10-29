use lsystem::{LSystem, MapRules};
use sanguine_lib::resources::layout::grid::Grid;
use sanguine_lib::resources::lsystems::{State, SystemState};
use sanguine_lib::resources::shapes::point::Point;

use std::f32::consts::PI;
use svg::node::element::Group;

pub fn form_group(work: Grid) -> Group {
    let graph = Group::new();

    // Set rules for the L-System
    let mut rules = MapRules::new();
    rules.set_str('F', "FF");
    rules.set_str('X', "F-[[X]+X]+F[+FX]-X");
    let axiom = "X".chars().collect();
    let lsystem = LSystem::new(rules, axiom);
    let iteration = 7;

    // Set starting properties
    let start = Point::new(600.0, 1200.0);
    let length = 20.0;
    let length_increment = 20.0;
    let turning_angle = (PI / 2.0) / 3.0;
    let turning_angle_increment = 20.0;
    let current_direction = 3.0 * (PI / 2.0);
    let reverse = false;

    let state = State::new(
        start,
        length,
        length_increment,
        turning_angle,
        turning_angle_increment,
        current_direction,
        reverse,
    );

    let system_state = SystemState::new(state);
    system_state.form_system(graph, iteration, lsystem)
}
