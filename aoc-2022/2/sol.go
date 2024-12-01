package solver

import (
	"fmt"
	"strings"
)

const Number string = "2"

func Solve(input []string) {
	overallAssumedScore := 0
	overallCorrectScore := 0
	for _, round := range input {
		overallAssumedScore += getAssumedRoundScore(round)
		overallCorrectScore += getCorrectRoundScore(round)
	}

	fmt.Printf("The total score according to the assumed strategy guide would be: %v\n", overallAssumedScore)
	fmt.Printf("The total score according to the correct strategy guide would be: %v", overallCorrectScore)
}

func getCorrectRoundScore(round string) int {
	opponentMove, suggestedOutcome := getMoves(round)
	opponentMoveScore := getMoveScore(opponentMove)

	switch suggestedOutcome {

	// lose
	case "X":
		suggestedMoveScore := (opponentMoveScore + 2) % 3
		return suggestedMoveScore + 1

	// draw
	case "Y":
		return opponentMoveScore + 4

	// win
	case "Z":
		suggestedMoveScore := (opponentMoveScore + 1) % 3
		return suggestedMoveScore + 7
	}

	panic("Unrecognized suggested outcome: '" + suggestedOutcome + "'")
}

func getAssumedRoundScore(round string) int {
	opponentMove, suggestedMove := getMoves(round)
	opponentMoveScore := getMoveScore(opponentMove)
	suggestedMoveScore := getMoveScore(suggestedMove)

	// tied
	if opponentMoveScore == suggestedMoveScore {
		return suggestedMoveScore + 4
	}

	// lost
	if opponentMoveScore == (suggestedMoveScore+1)%3 {
		return suggestedMoveScore + 1
	}

	// won
	return suggestedMoveScore + 7
}

func getMoveScore(move string) int {
	switch move {

	// rock
	case "A", "X":
		return 0

	// paper
	case "B", "Y":
		return 1

	// scissors
	case "C", "Z":
		return 2
	}

	panic("Unrecognized move: '" + move + "'")
}

func getMoves(round string) (opponent string, suggested string) {
	moves := strings.Split(round, " ")
	return moves[0], moves[1]
}
