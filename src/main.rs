use std::{thread, time};

mod cell;
mod cell_grid;

fn main() {
    let mut grid = cell_grid::CellGrid::new(8);
    grid.toggle(0, 3);
    grid.toggle(5, 4);
    grid.toggle(3, 3);
    grid.toggle(5, 6);
    grid.toggle(7, 3);
    grid.toggle(7, 7);
    grid.toggle(4, 7);
    grid.toggle(3, 0);
    grid.toggle(3, 1);
    grid.toggle(3, 2);

    loop {
        println!("{:?}", grid);
        grid.update();
        thread::sleep(time::Duration::from_secs(1));
    }
}
