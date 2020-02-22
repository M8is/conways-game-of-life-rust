/// Represents a single cell in Conway's Game of Life.
pub struct Cell {
    pub alive: bool,
}

impl Cell {
    /// Updates the cells state according to the rules of Conways Game of Life.
    /// The rules are as follows:
    /// 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    /// 2. Any live cell with two or three live neighbours lives on to the next generation.
    /// 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
    /// 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    pub fn update(&mut self, n: u8) {
        self.alive = n == 3 || (self.alive && n == 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! rule_tests {
        ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (alive, neighbors, expected) = $value;
                    let mut cell = Cell { alive };

                    cell.update(neighbors);

                    assert_eq!(cell.alive, expected);
                })*
        }
    }

    rule_tests! {
        stays_dead_on_0: (false, 0, false),
        stays_dead_on_1: (false, 1, false),
        stays_dead_on_2: (false, 2, false),
        becomes_alive_on_3: (false, 3, true),
        stays_dead_on_4: (false, 4, false),
        stays_dead_on_5: (false, 5, false),
        stays_dead_on_6: (false, 6, false),
        stays_dead_on_7: (false, 7, false),
        stays_dead_on_8: (false, 8, false),
        dies_on_0: (true, 0, false),
        dies_on_1: (true, 1, false),
        stays_alive_on_2: (true, 2, true),
        stays_alive_on_3: (true, 2, true),
        dies_on_4: (true, 4, false),
        dies_on_5: (true, 5, false),
        dies_on_6: (true, 6, false),
        dies_on_7: (true, 7, false),
        dies_on_8: (true, 8, false),
    }
}
