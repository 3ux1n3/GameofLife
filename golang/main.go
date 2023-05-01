package main

import (
	"fmt"
	"math/rand"
	"time"
)

type Cell struct {
	alive bool
}

func main() {
	// Initialize the grid with random values.
	r := rand.New(rand.NewSource(time.Now().UnixNano()))
	grid := make([][]Cell, 30)
	for i := range grid {
		grid[i] = make([]Cell, 30)
		for j := range grid[i] {
			grid[i][j].alive = r.Intn(2) == 1
		}
	}

	// Animate the game.
	for {
		fmt.Print("\033[2J") // Clear the console.
		fmt.Print("\033[H")  // Move the cursor to the top-left corner.
		renderGrid(grid)
		nextGrid := computeNextGrid(grid)
		grid = nextGrid
		time.Sleep(1000 * time.Millisecond) // Delay between frames.
	}
}

func computeNextGrid(grid [][]Cell) [][]Cell {
	nextGrid := make([][]Cell, len(grid))
	for i := range nextGrid {
		nextGrid[i] = make([]Cell, len(grid[i]))
		for j := range nextGrid[i] {
			nextGrid[i][j].alive = computeCellState(grid[i][j], getNeighbors(grid, i, j))
		}
	}
	return nextGrid
}

func computeCellState(cell Cell, neighbors []Cell) bool {
	numLiveNeighbors := 0
	for _, neighbor := range neighbors {
		if neighbor.alive {
			numLiveNeighbors++
		}
	}
	if cell.alive {
		// Cell is alive.
		if numLiveNeighbors < 2 || numLiveNeighbors > 3 {
			return false // Underpopulation or overpopulation.
		} else {
			return true // Survival.
		}
	} else {
		// Cell is dead.
		if numLiveNeighbors == 3 {
			return true // Reproduction.
		} else {
			return false // No change.
		}
	}
}

func getNeighbors(grid [][]Cell, row, col int) []Cell {
	neighbors := []Cell{}
	for i := row - 1; i <= row+1; i++ {
		for j := col - 1; j <= col+1; j++ {
			if i >= 0 && i < len(grid) && j >= 0 && j < len(grid[i]) && !(i == row && j == col) {
				neighbors = append(neighbors, grid[i][j])
			}
		}
	}
	return neighbors
}

func renderGrid(grid [][]Cell) {
	for _, row := range grid {
		for _, cell := range row {
			if cell.alive {
				fmt.Printf("\033[32m[%s]\033[0m", "#")
			} else {
				fmt.Print("   ")
			}
		}
		fmt.Println()
	}
}
