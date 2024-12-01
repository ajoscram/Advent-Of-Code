package solver

import (
	"fmt"
	"strconv"
	"strings"
)

const Number string = "4"

type assignment struct {
	lower int
	upper int
}

func Solve(input []string) {
	containedPairsCount := 0
	overlappedPairsCount := 0
	for _, assignmentPair := range input {
		assignments := getAssignments(assignmentPair)
		if contains(assignments[0], assignments[1]) || contains(assignments[1], assignments[0]) {
			containedPairsCount++
		}
		if overlaps(assignments[0], assignments[1]) || overlaps(assignments[1], assignments[0]) {
			overlappedPairsCount++
		}
	}
	fmt.Printf("The number of contained assignment pairs is: %v\n", containedPairsCount)
	fmt.Printf("The number of overlapped assignment pairs is: %v", overlappedPairsCount)
}

func getAssignments(assignmentPair string) []assignment {
	assignmentStrings := strings.Split(assignmentPair, ",")
	return []assignment{
		getAssignment(assignmentStrings[0]),
		getAssignment(assignmentStrings[1]),
	}
}

func getAssignment(assignmentStr string) assignment {
	assignmentBounds := strings.Split(assignmentStr, "-")
	lower, _ := strconv.Atoi(assignmentBounds[0])
	upper, _ := strconv.Atoi(assignmentBounds[1])
	return assignment{lower, upper}
}

func contains(first assignment, second assignment) bool {
	return first.lower <= second.lower && first.upper >= second.upper
}

func overlaps(first assignment, second assignment) bool {
	return (first.lower <= second.lower && first.upper >= second.lower) || contains(first, second)
}
