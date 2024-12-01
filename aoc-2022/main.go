package main

import (
	solver "ajoscram/advent_of_code/5"
	"os"
	"path/filepath"
	"strings"
)

func main() {
	inputFilepath := filepath.Join(solver.Number, "in.txt")

	inputBytes, err := os.ReadFile(inputFilepath)
	if err != nil {
		panic(err)
	}

	input := strings.Split(string(inputBytes), "\r\n")

	solver.Solve(input)
}
