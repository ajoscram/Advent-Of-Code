package solver

import (
	"ajoscram/advent_of_code/common"
	"fmt"
	"strings"
	"unicode"
)

const Number string = "3"
const lowerOffset int = 'a' - 1
const upperOffset int = 'A' - 27

func Solve(input []string) {
	compartmentPrioritiesSum := 0
	teamBadgePrioritySum := 0
	for index, rucksack := range input {
		firstCompartment, secondCompartment := splitRucksack(rucksack)
		repeatedItem := getRepeatedRune(firstCompartment, secondCompartment)
		compartmentPrioritiesSum += getPriority(repeatedItem)

		if index%3 == 0 {
			repeatedItem := getRepeatedRune(rucksack, input[index+1], input[index+2])
			teamBadgePrioritySum += getPriority(repeatedItem)
		}
	}
	fmt.Printf("The sum of compartment priorities is equal to: %v\n", compartmentPrioritiesSum)
	fmt.Printf("The sum of team badge priorities is equal to: %v", teamBadgePrioritySum)
}

func splitRucksack(rucksack string) (firstCompartment string, secondCompartment string) {
	split_index := len(rucksack) / 2
	firstCompartment = rucksack[:split_index]
	secondCompartment = rucksack[split_index:]
	return
}

func getRepeatedRune(first string, values ...string) rune {

	if len(values) == 0 {
		switch len(first) {
		case 0:
			panic("No repeats found")
		case 1:
			return []rune(first)[0]
		default:
			panic("Too many repeats found: " + first)
		}
	}

	repeatMap := map[rune]bool{}
	for _, char := range values[0] {
		if strings.ContainsRune(first, char) {
			repeatMap[char] = true
		}
	}

	repeats := common.Keys(repeatMap)
	return getRepeatedRune(string(repeats), values[1:]...)
}

func getPriority(item rune) int {
	if unicode.IsLower(item) {
		return int(item) - lowerOffset
	}

	return int(item) - upperOffset
}
