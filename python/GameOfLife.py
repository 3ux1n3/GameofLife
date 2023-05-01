import random
import time
import os


class Cell:
    def __init__(self):
        self.alive = False


def main():
    # Initialize the grid with random values.
    grid = [[Cell() for _ in range(10)] for _ in range(10)]
    for i in range(10):
        for j in range(10):
            grid[i][j].alive = random.random() < 0.5

    # Animate the game.
    while True:
        clear_console()
        render_grid(grid)
        next_grid = compute_next_grid(grid)
        grid = next_grid
        time.sleep(1)


def compute_next_grid(grid):
    next_grid = [[Cell() for _ in range(10)] for _ in range(10)]
    for i in range(10):
        for j in range(10):
            next_grid[i][j].alive = compute_cell_state(grid[i][j], get_neighbors(grid, i, j))
    return next_grid


def compute_cell_state(cell, neighbors):
    num_live_neighbors = sum(neighbor.alive for neighbor in neighbors)
    if cell.alive:
        # Cell is alive.
        if num_live_neighbors < 2 or num_live_neighbors > 3:
            return False  # Underpopulation or overpopulation.
        else:
            return True  # Survival.
    else:
        # Cell is dead.
        if num_live_neighbors == 3:
            return True  # Reproduction.
        else:
            return False  # No change.


def get_neighbors(grid, row, col):
    neighbors = []
    for i in range(max(row-1, 0), min(row+2, 10)):
        for j in range(max(col-1, 0), min(col+2, 10)):
            if i != row or j != col:
                neighbors.append(grid[i][j])
    return neighbors


def render_grid(grid):
    for row in grid:
        for cell in row:
            if cell.alive:
                print("\033[32m[#]\033[0m", end="")  # Green color for alive cells.
            else:
                print("[ ]", end="")
        print()


def clear_console():
    os.system('cls' if os.name == 'nt' else 'clear')


if __name__ == "__main__":
    main()
