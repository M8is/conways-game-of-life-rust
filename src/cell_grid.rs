use crate::cell;

#[derive(PartialEq)]
pub struct CellGrid {
    cells: Vec<cell::Cell>,
    dim: usize,
}

impl std::fmt::Debug for CellGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for chunk in self.cells.chunks(self.dim) {
            write!(f, "\n{:?}", chunk)?;
        }
        write!(f, "\n")
    }
}

impl CellGrid {
    /// Creates a new grid of size dim x dim.
    pub fn new(dim: usize) -> CellGrid {
        let mut cells: Vec<cell::Cell> = Vec::with_capacity(dim * dim);
        for _ in 0..(dim * dim) {
            cells.push(cell::Cell { alive: false });
        }
        CellGrid { cells, dim }
    }

    /// Forces the cell at coordinates (x,y) to change its state.
    pub fn toggle(&mut self, x: usize, y: usize) {
        let i = x + self.dim * y;
        self.cells[i].alive = !self.cells[i].alive;
    }

    /// Updates all cell states in the grid, according to the cell's update rules.
    pub fn update(&mut self) {
        let mut alive_neighbors: Vec<u8> = Vec::with_capacity(self.cells.len());
        for i in 0..self.cells.len() {
            alive_neighbors.push(self.get_alive_neighbors(i));
        }

        for (i, cell) in self.cells.iter_mut().enumerate() {
            cell.update(alive_neighbors[i]);
        }
    }

    /// Gets the number of alive neighbors
    fn get_alive_neighbors(&self, i: usize) -> u8 {
        let mut n = 0;
        let x = i % self.dim;
        let y = i / self.dim;
        if x > 0 {
            // Check left
            if self.cells[i - 1].alive {
                n = n + 1;
            }

            // Check above left
            if y > 0 {
                if self.cells[i - self.dim - 1].alive {
                    n = n + 1;
                }
            }

            // Check below left
            if y < self.dim - 1 {
                if self.cells[i + self.dim - 1].alive {
                    n = n + 1;
                }
            }
        };

        if x < self.dim - 1 {
            // Check right
            if self.cells[i + 1].alive {
                n = n + 1;
            }

            // Check above right
            if y > 0 {
                if self.cells[i - self.dim + 1].alive {
                    n = n + 1;
                }
            }

            // Check below right
            if y < self.dim - 1 {
                if self.cells[i + self.dim + 1].alive {
                    n = n + 1;
                }
            }
        };

        // Check above
        if y > 0 {
            if self.cells[i - self.dim].alive {
                n = n + 1;
            }
        };

        // Check below
        if y < self.dim - 1 {
            if self.cells[i + self.dim].alive {
                n = n + 1;
            }
        };

        n
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
        assert!(grid.cells[4 + 2 * 8].alive);
        grid.toggle(4, 2);
        assert!(!grid.cells[4 + 2 * 8].alive);
    }

    #[test]
    fn counts_alive_neighbors() {
        // Grid setup (X = dead, O = alive):
        // O X O
        // X O O
        // O X X
        let mut grid = CellGrid::new(3);
        grid.toggle(0, 0);
        grid.toggle(2, 0);
        grid.toggle(1, 1);
        grid.toggle(2, 1);
        grid.toggle(0, 2);

        let actual = grid.get_alive_neighbors(4);
        assert_eq!(actual, 4);
    }

    #[test]
    fn counts_alive_neighbors_on_edge() {
        // Grid setup (X = dead, O = alive):
        // O X O
        // X O O
        // O X X
        let mut grid = CellGrid::new(3);
        grid.toggle(0, 0);
        grid.toggle(2, 0);
        grid.toggle(1, 1);
        grid.toggle(2, 1);
        grid.toggle(0, 2);

        let actual = grid.get_alive_neighbors(2);
        assert_eq!(actual, 2);
    }

    #[test]
    fn updates_cells() {
        // Grid before update (X = dead, O = alive):
        // O X O X
        // X O O X
        // O X X X
        // X X O X
        let mut grid = CellGrid::new(4);
        grid.toggle(0, 0);
        grid.toggle(2, 0);
        grid.toggle(1, 1);
        grid.toggle(2, 1);
        grid.toggle(0, 2);
        grid.toggle(2, 3);

        grid.update();

        // Expected grid after update (X = dead, O = alive):
        // X X O X
        // O X O X
        // X X O X
        // X X X X
        let mut expected = CellGrid::new(4);
        expected.toggle(2, 0);
        expected.toggle(0, 1);
        expected.toggle(2, 1);
        expected.toggle(2, 2);
        assert_eq!(grid, expected);
    }
}
