package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type Symbol struct {
	x int
	y int
}

type Part struct {
	number string
}

func (p *Part) Value() int {
	value, err := strconv.ParseInt(p.number, 10, 32)
	if err != nil {
		log.Fatalf("Error in conversion: %v", err)
	}
	return int(value)
}

type Schematic struct {
	coords  [][]interface{}
	symbols []*Symbol
}

func (s *Schematic) ParseLine(line string) {
	y := len(s.coords)
	s.coords = append(s.coords, make([]interface{}, len(line)))
	for x, r := range line {
		switch r {
		case '.':
			s.coords[y][x] = nil
		default:
			if _, err := strconv.Atoi(string(r)); err != nil {
				sym := &Symbol{x: x, y: y}
				s.coords[y][x] = sym
				s.symbols = append(s.symbols, sym)
				continue
			}
			if x > 0 {
				if p, ok := s.coords[y][x-1].(*Part); ok {
					p.number += string(r)
					s.coords[y][x] = p
					continue
				}
			}
			s.coords[y][x] = &Part{number: string(r)}
		}
	}
}

func (s *Schematic) ValidParts() []*Part {
	set := make(map[*Part]struct{})
	for _, sym := range s.symbols {
		for x := sym.x - 1; x <= sym.x+1; x++ {
			for y := sym.y - 1; y <= sym.y+1; y++ {
				if p, ok := s.coords[y][x].(*Part); ok {
					set[p] = struct{}{}
				}
			}
		}
	}
	pp := make([]*Part, len(set))
	i := 0
	for p := range set {
		pp[i] = p
		i++
	}
	return pp
}

func NewSchematic() Schematic {
	return Schematic{
		coords:  make([][]interface{}, 0),
		symbols: make([]*Symbol, 0),
	}
}

func load(filepath string) (*Schematic, error) {
	file, err := os.Open(filepath)
	if err != nil {
		return nil, err
	}
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			fmt.Println(err)
		}
	}(file)

	schematic := NewSchematic()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		schematic.ParseLine(line)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return &schematic, nil
}

func sumParts(parts []*Part) int {
	sum := 0
	for _, p := range parts {
		sum += p.Value()
	}
	return sum
}

func main() {
	schematic, err := load("./test_input.txt")
	if err != nil {
		log.Fatal(err)
	}

	parts := schematic.ValidParts()
	sum := sumParts(parts)
	if sum != 4361 {
		log.Fatalf("Should be 4361 (was %d)", sum)
	}

	schematic, err = load("./input.txt")
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part 1 Answer:")
	fmt.Println(sumParts(schematic.ValidParts()))
}
