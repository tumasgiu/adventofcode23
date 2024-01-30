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

type Node struct {
	ID   string
	Next *Node
}

type LinkedList struct {
	Head *Node
}

func (l *LinkedList) Insert(id string) {
	newNode := &Node{
		ID: id,
	}

	if l.Head == nil {
		l.Head = newNode
	} else {
		currentNode := l.Head
		for currentNode.Next != nil {
			currentNode = currentNode.Next
		}
		currentNode.Next = newNode
	}
}

func solve(filepath string) (int, int) {
	stack := make(map[string]*LinkedList)
	total := 0
	file, err := os.Open(filepath)
	if err != nil {
		fmt.Println("Error: ", err)
		return -1, -1
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
		return -1, -1
	}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		points := 0
		match := re.FindStringSubmatch(line)
		if match != nil {
			id, winning, drawn := match[1], parseNumbers(match[2]), parseNumbers(match[3])
			winningCount := 0
			for _, n := range drawn {
				if slices.Index(winning, n) != -1 {
					if points == 0 {
						points = 1
					} else {
						points = points * 2
					}
					winningCount++
				}
			}
			//fmt.Printf("Card %s: %d points\n", id, points)
			total += points
			o := &LinkedList{}
			for i := 1; i <= winningCount; i++ {
				j, _ := strconv.Atoi(id)
				o.Insert(strconv.Itoa(i + j))
			}
			stack[id] = o
		} else {
			log.Fatal("Unmatched line")
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error: ", err)
	}

	cardsCount := 0
	for _, l := range stack {
		cardsCount += stackSize(stack, l)
	}

	return total, cardsCount
}

func stackSize(stack map[string]*LinkedList, start *LinkedList) int {
	cardsCount := 1
	list := start.Head

	for list != nil {
		id := list.ID
		o := stack[id]
		v := stackSize(stack, o)
		cardsCount += v
		list = list.Next
	}

	return cardsCount
}

func main() {
	total, cardsCount := solve("./test_input.txt")
	if total != 13 {
		log.Fatal("Should be 13, was", total)
	}
	if cardsCount != 30 {
		log.Fatal("Should be 30, was", cardsCount)
	}

	total, cardsCount = solve("./input.txt")
	fmt.Println("Part 1 Answer:", total)
	fmt.Println("Part 2 Answer:", cardsCount)
}
