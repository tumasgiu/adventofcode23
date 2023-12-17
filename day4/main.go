package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"slices"
	"strconv"
	"strings"
)

func parseNumbers(numbers string) []int {
	strs := strings.Split(numbers, " ")
	res := make([]int, 0)
	for _, str := range strs {
		if str == "" {
			continue
		}
		num, err := strconv.Atoi(str)
		if err != nil {
			log.Fatal("This should not happen(TM)")
		}
		res = append(res, num)
	}

	return res
}

func solve(filepath string) int {
	total := 0
	file, err := os.Open(filepath)
	if err != nil {
		fmt.Println("Error: ", err)
		return -1
	}

	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			log.Fatal(err)
		}
	}(file)

	re, err := regexp.Compile(`Card\s+(?P<id>\d+):\s+(?P<winning>.+?)\s+\|\s+(?P<drawn>.+)`)
	if err != nil {
		fmt.Println("Error compiling regex: ", err)
		return -1
	}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		points := 0
		match := re.FindStringSubmatch(line)
		if match != nil {
			id, winning, drawn := match[1], parseNumbers(match[2]), parseNumbers(match[3])
			for _, n := range drawn {
				if slices.Index(winning, n) != -1 {
					if points == 0 {
						points = 1
					} else {
						points = points * 2
					}
				}
			}
			fmt.Printf("Card %s: %d points\n", id, points)
			total += points
		} else {
			log.Fatal("Unmatched line")
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error: ", err)
	}

	return total
}

func main() {
	total := solve("./test_input.txt")
	if total != 13 {
		log.Fatal("Should be 13, was", total)
	}

	total = solve("./input.txt")
	fmt.Println("Part 1 Answer:", total)
}
