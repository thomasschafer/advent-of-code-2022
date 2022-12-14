package day04

import (
	"strconv"
	"strings"

	"github.com/thomasschafer/advent_of_code_2022/utils"
)

func assignmentContainsAnother(assignment1, assignment2 string) bool {
	assignment1Split := strings.Split(assignment1, "-")
	min1 := utils.Expect(strconv.Atoi(assignment1Split[0]))
	max1 := utils.Expect(strconv.Atoi(assignment1Split[1]))
	assignment2Split := strings.Split(assignment2, "-")
	min2 := utils.Expect(strconv.Atoi(assignment2Split[0]))
	max2 := utils.Expect(strconv.Atoi(assignment2Split[1]))
	return (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
}

func Part1(filePath string) int {
	rows := utils.RowsFromFile(filePath)
	result := 0
	for _, row := range rows {
		rowSplit := strings.Split(row, ",")
		assignment1 := rowSplit[0]
		assignment2 := rowSplit[1]
		if assignmentContainsAnother(assignment1, assignment2) {
			result += 1
		}
	}
	return result
}
