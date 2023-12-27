use rand::prelude::*;

use sanguine_lib::resources::{
    composition::CompositionCenter,
    layout::voronoi::{Cell, VoronoiDiagram},
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
        println!("retro");
        let clone_d = self.0.clone();
        for cell in &mut self.0.cells {
            match cell.density {
                Density::Empty => {
                    let neighbors = cell.find_neighbors(&clone_d);
                    let number = neighbors.len();
                    let filled_neighbors = direction_of_contact(cell, neighbors);
                    println!("number {}, empty: {:?}", number, filled_neighbors.len());
                    // missing case: both are the same
                    match (number, filled_neighbors.len()) {
                        (_, 0) => cell.set_density(Density::Empty),
                        (_, 1) => cell.set_density(Density::Edge(Lines(filled_neighbors))),
                        (_, 2) => {
                            cell.set_density(Density::Transition(Lines(filled_neighbors)));
                            println!("transition")
                        }

                        (_, 3) => cell.set_density(Density::ThreeWay(Lines(filled_neighbors))),

                        //or corner
                        (5, 4) | (6, 4..=5) | (7, 4..=6) | (8, 4..=7) | (9, 4..=8) => {
                            cell.set_density(Density::Lopsided(Lines(filled_neighbors)))
                        }

                        _ => cell.set_density(Density::Low),
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

fn direction_of_contact(cell: &Cell, neighbors: Vec<(Point, Density, Line)>) -> Vec<(Line, usize)> {
    let mut touch_line = Vec::new();

    for neighbor in neighbors {
        match neighbor.1 {
            Density::Empty
            | Density::Lopsided(_)
            | Density::Edge(_)
            | Density::Transition(_)
            | Density::Corner(_) => {}

            _ => touch_line.push((neighbor.2, 0)),
        }
    }

    for border in 0..cell.border_lines.len() {
        for mut line in touch_line.iter_mut() {
            if cell.border_lines[border].equal(line.0) {
                line.1 = border;
            }
        }
    }
    // println!("touch line: {:?}", touch_line);
    touch_line
}

// helpers

// fn point_to_point(voi_point: (f64, f64)) -> Point {
//     Point::new(voi_point.0 as f32, voi_point.1 as f32)
// }
