use rand::prelude::*;

use sanguine_lib::resources::{
    composition::CompositionCenter,
    layout::voronoi::VoronoiDiagram,
    shapes::{line::Line, point::Point},
};

use sanguine_lib::resources::composition::{Composition, Density, Direction::Lines};

#[derive(Debug, Clone)]
pub struct MyVoronoiDiagram(pub VoronoiDiagram);
impl Composition for MyVoronoiDiagram {
    fn filled(&mut self, density_var: &Density) {
        for item in 0..self.0.cells.len() {
            self.0.cells[item].set_density(density_var.clone());
        }
    }

    fn add_random_center(&mut self, amount: usize) {
        let mut rng = thread_rng();

        for _i in 0..amount {
            let cell_index = rng.gen_range(0..self.0.cells.len());

            // high density around the focus can't be set yet, because
            // there's no way yet to find the neighbors

            // for row in vertical - 1..=vertical + 1 {
            //     for col in horizontal - 1..=horizontal + 1 {
            //         self.0.field_container[row][col].density = Density::High;
            //         println!("mid");
            //     }
            // }

            self.0.cells[cell_index].density = Density::Focus;
        }
    }

    fn add_random_low(&mut self, amount: usize) {
        let mut rng = thread_rng();

        for _i in 0..amount {
            let cell_index = rng.gen_range(0..self.0.cells.len());

            self.0.cells[cell_index].density = Density::Low;
        }
    }

    fn connect_centers(&mut self) {
        unimplemented!()
    }

    fn add_center(&mut self, center: CompositionCenter) {
        unimplemented!()
    }

    fn retro_composition(&mut self) {
        let clone_d = self.0.clone();
        for cell in &mut self.0.cells {
            match cell.density {
                Density::Empty => {
                    let neighbors = cell.find_neighbors(&clone_d);
                    let number = neighbors.len();
                    let contactless = direction_of_contact(neighbors);
                    match contactless.len() {
                        0 => cell.set_density(Density::Low),
                        1 => cell.set_density(Density::ThreeWay(Lines(contactless))),
                        2 => cell.set_density(Density::Transition(Lines(contactless))),
                        3 => cell.set_density(Density::Corner(Lines(contactless))),
                        4 => cell.set_density(Density::Edge(Lines(contactless))),
                        5 => cell.set_density(Density::Edge(Lines(contactless))),
                        6 => cell.density = Density::Empty,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    fn direction_of_contact(&mut self, row: usize, col: usize) -> Vec<bool> {
        unimplemented!()
    }
}

fn direction_of_contact(neighbors: Vec<(Point, Density, Line)>) -> Vec<Line> {
    let mut touch_line = Vec::new();
    for neighbor in neighbors {
        match neighbor.1 {
            Density::Empty => touch_line.push(neighbor.2),
            _ => {}
        }
    }
    touch_line
}

// helpers

// fn point_to_point(voi_point: (f64, f64)) -> Point {
//     Point::new(voi_point.0 as f32, voi_point.1 as f32)
// }
