use rand::prelude::*;


use crate::resources::layout;

use super::{Density, Composition};


impl Composition for layout::voronoi::VoronoiDiagram {
    fn filled(&mut self, density_var: Density) {
        
        for item in 0..self.cells.len() {
            self.cells[item].density = density_var;
        }       
    }

    fn add_random_center(&mut self, amount: usize) {
        let mut rng = thread_rng();

        for _i in 0..amount {
            let cell_index = rng.gen_range(0..self.cells.len());
            
            // high density around the focus can't be set yet, because 
            // there's no way yet to find the neighbors

            // for row in vertical - 1..=vertical + 1 {
            //     for col in horizontal - 1..=horizontal + 1 {
            //         self.field_container[row][col].density = Density::High;
            //         println!("mid");
            //     }
            // }

            self.cells[cell_index].density = Density::Focus;
        }
    }

    fn add_random_low(&mut self, amount: usize) {
        let mut rng = thread_rng();

        for _i in 0..amount {
            let cell_index = rng.gen_range(0..self.cells.len());

        self.cells[cell_index].density = Density::Low;
        }
    }

    fn connect_centers(&mut self) {
        unimplemented!()
    }

    fn add_center(&mut self, center: super::CompositionCenter) {
        unimplemented!()
    }

    fn retro_composition(&mut self) {
        unimplemented!()
    }

    fn direction_of_contact(&mut self, row: usize, col: usize) -> Vec<bool> {
        unimplemented!()
    }
}


   

// helpers

// fn point_to_point(voi_point: (f64, f64)) -> Point {
//     Point::new(voi_point.0 as f32, voi_point.1 as f32)
// }
