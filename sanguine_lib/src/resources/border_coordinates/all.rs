use crate::resources::layout::{grid::Grid, voronoi::VoronoiDiagram, Layout};

use super::{cell_border::CellBorderCoords, one_side::OneSide};

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
                    // is this clone ok???
                    work.get_fields()[row][col].clone(),
                    amount,
                ));
            }
            vec.push(inner);
        }
        AllBorderCoordinates(vec)
    }

    pub fn new_from_voronoi(work: &VoronoiDiagram, amount: usize) -> Self {
        let mut vec = Vec::new();

        for cell in &work.cells {
            let mut cell_v = CellBorderCoords::new_empty();
            cell_v.0.clear();
            for line in &cell.border_lines {
                let mut side = OneSide::new_random(*line, amount);
                side.sort_points_on_line();
                cell_v.0.push(side);
            }
            vec.push(cell_v);
        }
        let mut outer = Vec::new();
        outer.push(vec);
        AllBorderCoordinates(outer)
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

    pub fn tesselate_voronoi(&mut self, work: &VoronoiDiagram) {
        for cell_item in 0..work.cells.len() {
            for line_item in 0..work.cells[cell_item].border_lines.len() {
                let points = self.0[0][cell_item].0[line_item].clone();

                for other_cell_item in 0..work.cells.len() {
                    for other_line_item in 0..work.cells[other_cell_item].border_lines.len() {
                        let line = work.cells[cell_item].border_lines[line_item];
                        let other_line = work.cells[other_cell_item].border_lines[other_line_item];
                        if line.equal(other_line) {
                            self.0[0][other_cell_item].0[other_line_item] = points.clone();
                        }
                    }
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
