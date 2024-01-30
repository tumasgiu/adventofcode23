package main

import (
	"bufio"
	"fmt"
	mapset "github.com/deckarep/golang-set/v2"
	"log"
	"os"
	"strconv"
)

type Symbol struct {
	x         int
	y         int
	maybeGear bool
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

type Gear struct {
	Ratio int
}

type Schematic struct {
	coords  [][]interface{}
	symbols []*Symbol

	validParts []*Part
	gears      []*Gear
}

func (s *Schematic) AddSymbol(x, y int, maybeGear bool) {
	sym := &Symbol{x: x, y: y, maybeGear: maybeGear}
	s.coords[y][x] = sym
	s.symbols = append(s.symbols, sym)
}

func (s *Schematic) ParseLine(line string) {
	y := len(s.coords)
	s.coords = append(s.coords, make([]interface{}, len(line)))
	for x, r := range line {
		switch r {
		case '.':
			s.coords[y][x] = nil
		case '*':
			s.AddSymbol(x, y, true)
		default:
			if _, err := strconv.Atoi(string(r)); err != nil {
				s.AddSymbol(x, y, false)
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

func (s *Schematic) process() {
	s.gears = make([]*Gear, 0)
	s.validParts = make([]*Part, 0)

	set := mapset.NewSet[*Part]()
	for _, sym := range s.symbols {
		symParts := mapset.NewSet[*Part]()
		for x := sym.x - 1; x <= sym.x+1; x++ {
			for y := sym.y - 1; y <= sym.y+1; y++ {
				if p, ok := s.coords[y][x].(*Part); ok {
					set.Add(p)
					symParts.Add(p)
				}
			}
		}
		if sym.maybeGear && symParts.Cardinality() == 2 {
			spp := symParts.ToSlice()
			s.gears = append(s.gears, &Gear{Ratio: spp[0].Value() * spp[1].Value()})
		}
	}
	s.validParts = append(s.validParts, set.ToSlice()...)
}

func (s *Schematic) ValidParts() []*Part {
	if s.validParts == nil {
		s.process()
	}
	return s.validParts
}

func (s *Schematic) Gears() []*Gear {
	if s.gears == nil {
		s.process()
	}
	return s.gears
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

func sumRatios(gears []*Gear) int {
	sum := 0
	for _, p := range gears {
		sum += p.Ratio
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

	gears := schematic.Gears()
	sum = sumRatios(gears)
	if sum != 467835 {
		log.Fatalf("Should be 467835 (was %d)", sum)
	}

	schematic, err = load("./input.txt")
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Part 1 Answer:")
	fmt.Println(sumParts(schematic.ValidParts()))

	fmt.Println("Part 2 Answer:")
	fmt.Println(sumRatios(schematic.Gears()))
}
