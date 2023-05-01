use rand::prelude::*;
use std::{thread, time};

#[derive(Clone, Copy)]
struct Cell {
    alive: bool,
}

fn main() {
    // Initialize the grid with random values.
    let mut rng = thread_rng();
    let mut grid = [[Cell { alive: false }; 20]; 20];
    for i in 0..20 {
        for j in 0..20 {
            grid[i][j].alive = rng.gen_bool(0.5);
        }
    }

    // Animate the game.
    loop {
        clear_console();
        //println!("Generation {}:", generation);
        render_grid(&grid);
        let next_grid = compute_next_grid(&grid);
        grid = next_grid;
        thread::sleep(time::Duration::from_millis(500));
    }
}

fn compute_next_grid(grid: &[[Cell; 20]; 20]) -> [[Cell; 20]; 20] {
    let mut next_grid = [[Cell { alive: false }; 20]; 20];
    for i in 0..20 {
        for j in 0..20 {
            next_grid[i][j].alive = compute_cell_state(grid[i][j], get_neighbors(grid, i, j));
        }
    }
    next_grid
}

fn compute_cell_state(cell: Cell, neighbors: [Cell; 8]) -> bool {
    let num_live_neighbors = neighbors.iter().filter(|&neighbor| neighbor.alive).count();
    if cell.alive {
        // Cell is alive.
        if num_live_neighbors < 2 || num_live_neighbors > 3 {
            false // Underpopulation or overpopulation.
        } else {
            true // Survival.
        }
    } else {
        // Cell is dead.
        if num_live_neighbors == 3 {
            true // Reproduction.
        } else {
            false // No change.
        }
    }
}

fn get_neighbors(grid: &[[Cell; 20]; 20], row: usize, col: usize) -> [Cell; 8] {
    let mut neighbors = [Cell { alive: false }; 8];
    let mut k = 0;
    for i in row.saturating_sub(1)..=(row + 1).min(9) {
        for j in col.saturating_sub(1)..=(col + 1).min(9) {
            if i != row || j != col {
                neighbors[k] = grid[i][j];
                k += 1;
            }
        }
    }
    neighbors
}

fn render_grid(grid: &[[Cell; 20]; 20]) {
    for row in grid.iter() {
        for cell in row.iter() {
            if cell.alive {
                print!("\x1B[32m[#]\x1B[0m"); // Green color for alive cells.
            } else {
                print!("[ ]");
            }
        }
        println!();
    }
}

fn clear_console() {
    print!("\x1B[2J"); // Clear the console.
    print!("\x1B[H");  // Move the cursor to the top-left corner.
}
