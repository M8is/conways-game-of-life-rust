use crate::cell;

pub struct CellGrid {
    cells: Vec<cell::Cell>,
    dim: usize,
}

impl CellGrid {
    pub fn new(dim: usize) -> CellGrid {
        let mut cells: Vec<cell::Cell> = Vec::with_capacity(dim * dim);
        for _ in 0..(dim * dim) {
            cells.push(cell::Cell { alive: false });
        }
        CellGrid { cells, dim }
    }

    /// Forces the cell at coordinates (x,y) to change its state.
    pub fn toggle(&mut self, x: usize, y: usize) {
        let i = self.dim * x + y;
        self.cells[i].alive = !self.cells[i].alive;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_grid() {
        let n = 8;
        let grid = CellGrid::new(n);

        assert_eq!(grid.dim, n);
        assert_eq!(grid.cells.len(), n * n);
        for cell in grid.cells {
            assert!(!cell.alive);
        }
    }

    #[test]
    fn toggles_cell() {
        let mut grid = CellGrid::new(8);
        grid.toggle(4, 2);
        assert!(grid.cells[4 * 8 + 2].alive);
        grid.toggle(4, 2);
        assert!(!grid.cells[4 * 8 + 2].alive);
    }
}
