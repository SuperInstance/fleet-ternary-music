package main

import (
	"fmt"
	"math"
	"os"
	"text/tabwriter"
)

// Ternary represents a -1, 0, or +1 strategy value.
// In Go, we use constants with iota to encode the group structure.
const (
	NegOne = -1 + iota  // -1 (opposition)
	Zero                // 0 (sustain)
	One                 // +1 (assertion)
)

// Interval represents a musical interval in semitones.
type Interval struct {
	Semitones int8
	Name      string
}

// The three core intervals — the group's generator set
var (
	MajorThird = Interval{4, "major third up"}    // +1 assertion
	Unison     = Interval{0, "unison"}             // 0 sustain
	MinorThird = Interval{-4, "minor third down"} // -1 opposition
)

// ternaryToInterval maps ternary values to musical intervals.
// This is the core mathematical mapping.
func ternaryToInterval(t int) Interval {
	switch t {
	case One:
		return MajorThird
	case Zero:
		return Unison
	case NegOne:
		return MinorThird
	}
	return Unison
}

// ComposeIntervals applies the group operation.
// Conservation law: MajorThird + MinorThird = Unison
func ComposeIntervals(a, b Interval) Interval {
	if a.Semitones+b.Semitones == 0 {
		return Unison // Conservation!
	}
	return Interval{a.Semitones + b.Semitones, "composite"}
}

// VectorToNotes converts a ternary vector to MIDI pitches.
func VectorToNotes(vector []int, basePitch uint8) []uint8 {
	notes := make([]uint8, 0, len(vector)+1)
	notes = append(notes, basePitch)
	for _, v := range vector {
		interval := ternaryToInterval(v)
		last := notes[len(notes)-1]
		next := int(last) + int(interval.Semitones)
		if next < 0 {
			next = 0
		} else if next > 127 {
			next = 127
		}
		notes = append(notes, uint8(next))
	}
	return notes
}

// SymmetryGroup describes a symmetry relationship in a vector
type SymmetryGroup struct {
	Type       string
	LeftIndex  int
	RightIndex int
}

// AnalyzeSymmetry finds mirror pairs and conservation pairs.
func AnalyzeSymmetry(vector []int) []SymmetryGroup {
	var groups []SymmetryGroup
	n := len(vector)
	for i := 0; i < n/2; i++ {
		left := vector[i]
		right := vector[n-1-i]
		if left == right {
			groups = append(groups, SymmetryGroup{"mirror", i, n - 1 - i})
		} else if left+right == 0 {
			groups = append(groups, SymmetryGroup{"conservation", i, n - 1 - i})
		}
	}
	return groups
}

// Stats returns density and balance of a ternary vector
type Stats struct {
	Length  int
	Density float64
	Balance float64
}

func Analyze(stats *Stats, vector []int) {
	pos, neg := 0, 0
	for _, v := range vector {
		if v == One {
			pos++
		} else if v == NegOne {
			neg++
		}
	}
	stats.Length = len(vector)
	stats.Density = math.Round(float64(pos+neg)/float64(len(vector))*100) / 100
	stats.Balance = math.Round(float64(pos-neg)/float64(len(vector))*100) / 100
}

func main() {
	vector := []int{1, 0, -1, 1, 0, -1, 1, 1}
	notes := VectorToNotes(vector, 60)
	sym := AnalyzeSymmetry(vector)
	var stats Stats
	Analyze(&stats, vector)

	w := tabwriter.NewWriter(os.Stdout, 0, 0, 2, ' ', 0)
	fmt.Fprintf(w, "Vector:\t%v\n", vector)
	fmt.Fprintf(w, "Notes:\t%v\n", notes)
	fmt.Fprintf(w, "Length:\t%d\n", stats.Length)
	fmt.Fprintf(w, "Density:\t%.2f\n", stats.Density)
	fmt.Fprintf(w, "Balance:\t%.2f\n", stats.Balance)
	fmt.Fprintf(w, "Symmetry groups:\t%d\n", len(sym))
	for _, s := range sym {
		fmt.Fprintf(w, "  %s:\tindex %d ⇄ index %d\n", s.Type, s.LeftIndex, s.RightIndex)
	}

	// Demonstrate conservation law in Go
	composed := ComposeIntervals(MajorThird, MinorThird)
	fmt.Fprintf(w, "\nConservation:\t%s + %s = %s (%d semitones)\n",
		MajorThird.Name, MinorThird.Name, composed.Name, composed.Semitones)
	w.Flush()
}
