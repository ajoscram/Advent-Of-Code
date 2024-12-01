package solver

import (
	"ajoscram/advent_of_code/common"
	"fmt"
	"sort"
	"strconv"
)

const Number string = "1"

func Solve(input []string) {

	var caloriesPerElf []int
	currentCalories := 0

	for _, line := range input {

		if line == "" {
			caloriesPerElf = append(caloriesPerElf, currentCalories)
			currentCalories = 0
			continue
		}

		calories, _ := strconv.Atoi(line)
		currentCalories += calories
	}

	sort.Ints(caloriesPerElf)

	mostCaloriesPerElf := caloriesPerElf[len(caloriesPerElf)-1]
	top3CaloriesPerElfAdded := common.Add(caloriesPerElf[len(caloriesPerElf)-3:]...)

	fmt.Printf("Most calories carried by an elf is: %v\n", mostCaloriesPerElf)
	fmt.Printf("Calories carried by the 3 elves with most calories: %v", top3CaloriesPerElfAdded)
}
