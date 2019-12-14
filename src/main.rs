mod puzzle03;

/// Handles command-line arguments
fn main() {
    let grid: puzzle03::Grid = puzzle03::Grid::grid();
    println!("First, {}, {}", grid.get_pos()[0], grid.get_pos()[1]);
}