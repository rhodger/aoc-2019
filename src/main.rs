mod puzzle03;

/// Handles command-line arguments
fn main() {
    let mut grid: puzzle03::Grid = puzzle03::Grid::grid();
    grid.execute("./content/input031.txt");
}