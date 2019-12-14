use std::env;
use clap::*;

mod puzzle01;
mod puzzle02;
mod puzzle03;

/// Handles command-line arguments
fn main() {
    let grid: puzzle03::Grid = puzzle03::Grid::Grid();
}