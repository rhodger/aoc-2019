mod puzzle03;

/// Handles command-line arguments
fn main() {
    let mut grid1: puzzle03::Grid = puzzle03::Grid::grid();
    let mut grid2: puzzle03::Grid = puzzle03::Grid::grid();

    puzzle03::Grid::puzzle1(&mut grid1, &mut grid2, "./content/input031.txt");
}