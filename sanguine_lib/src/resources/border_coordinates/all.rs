use crate::resources::layout::{Layout,grid::Grid}; 

use super::cell_border::CellBorderCoords;

/// This module contains a bunch of functions that create random coordinates on field borders.
///
/// It's currently limited to fields with 4 sides.

#[derive(Clone, Debug, PartialEq)]
pub struct AllBorderCoordinates(pub Vec<Vec<CellBorderCoords>>);

impl AllBorderCoordinates {
    /// Returns a Vector of edge points for an entire work of art.
    pub fn new(work: &Grid, amount: usize) -> Self {
        let mut vec = Vec::new();

        for row in 0..work.get_rows() {
            let mut inner = Vec::new();
            for col in 0..work.get_columns() {
                inner.push(CellBorderCoords::new(
                    work.get_fields()[row][col],
                    amount,
                ));
            }
            vec.push(inner);
        }
        AllBorderCoordinates(vec)
    }

    /// Tesselation, makes sure that points on the edges of a field are corresponding with the neighboring edge
    /// 0 = top
    /// 1 = left
    /// 2 = bottom
    /// 3 = right
    pub fn tesselate(&mut self) {
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                if row == 0 && col != 0 {
                    self.0[row][col].0.remove(1);
                    let copy_side = self.0[row][col - 1].0[3].copy_side();
                    self.0[row][col].0.insert(1, copy_side);
                } else if row != 0 && col == 0 {
                    self.0[row][col].0.remove(0);
                    let copy_side = self.0[row - 1][col].0[2].copy_side();
                    self.0[row][col].0.insert(0, copy_side);
                } else if row != 0 && col != 0 {
                    self.0[row][col].0.remove(1);
                    let copy_side = self.0[row][col - 1].0[3].copy_side();
                    self.0[row][col].0.insert(1, copy_side);

                    self.0[row][col].0.remove(0);
                    let copy_side = self.0[row - 1][col].0[2].copy_side();
                    self.0[row][col].0.insert(0, copy_side);
                }
            }
        }
    }

    pub fn slight_chaos(&mut self) {
        for row in 0..self.0.len() {
            for col in 0..self.0[row].len() {
                for side in 0..self.0[row][col].0.len() {
                    self.0[row][col].0[side].chaos_sort();
                }
            }
        }
    }
}
