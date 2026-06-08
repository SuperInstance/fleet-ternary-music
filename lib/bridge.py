"""
Ternary ⇄ Music Theory Bridge
================================
The core mathematical mapping between ternary {-1, 0, +1} strategy vectors
and musical intervals, scales, and harmonic structures.

For the complete theory, see THEORY.md
"""

# Ternary value → musical interval (semitones)
TERNARY_TO_INTERVAL = {
    -1: -4,   # opposition → minor third down (dissonant)
     0:  0,   # sustain → unison (no change)
    +1:  4,   # assertion → major third up (consonant)
}

# Extended intervals for multi-step analysis
TERNARY_TO_EXTENDED = {
    -1: -7,   # perfect fifth down
     0:  0,    # unison
    +1:  7,    # perfect fifth up
}

# Conservation pairs (opposites in music theory)
# +1 and -1 are musical mirrors — they cancel to 0 like dissonance resolves to consonance
CONSERVATION_PAIRS = [
    (0, 0, 0),      # C → C (unchanged)
    (1, -1, 0),      # assertion + opposition = sustain
    (-1, 1, 0),      # opposition + assertion = sustain
    (1, 1, -2),      # double assertion = double major third up
    (-1, -1, 2),     # double opposition = double minor third down
]

def vector_to_notes(ternary_vector, base_pitch=60):
    """Convert a ternary strategy vector to MIDI pitches.
    
    Each +1 moves up a major third from the last note.
    Each -1 moves down a minor third from the last note.
    Each 0 repeats the last note.
    
    This creates a musically coherent sequence from any strategy vector.
    """
    notes = [base_pitch]
    for v in ternary_vector:
        interval = TERNARY_TO_INTERVAL.get(v, 0)
        last = notes[-1]
        next_note = max(0, min(127, last + interval))
        notes.append(next_note)
    return notes

def analyze_symmetry(ternary_vector):
    """Find symmetry groups in a strategy vector.
    
    Returns groups of indices where the vector is palindrome,
    or where conservation pairs cancel.
    """
    length = len(ternary_vector)
    groups = []
    
    # Check for palindromic symmetry
    for i in range(length // 2):
        if ternary_vector[i] == ternary_vector[length - 1 - i]:
            groups.append({
                "type": "mirror",
                "left_index": i,
                "right_index": length - 1 - i,
                "value": ternary_vector[i]
            })
        elif ternary_vector[i] + ternary_vector[length - 1 - i] == 0:
            groups.append({
                "type": "conservation_pair",
                "left_index": i,
                "right_index": length - 1 - i,
                "values": [ternary_vector[i], ternary_vector[length - 1 - i]]
            })
    
    return groups

def interval_name(semitones):
    """Convert semitone distance to musical interval name."""
    names = {
        -12: "octave down", -11: "major seventh down",
        -10: "minor seventh down", -9: "major sixth down",
        -8: "minor sixth down", -7: "perfect fifth down",
        -6: "tritone down", -5: "perfect fourth down",
        -4: "major third down", -3: "minor third down",
        -2: "major second down", -1: "minor second down",
        0: "unison",
        1: "minor second up", 2: "major second up",
        3: "minor third up", 4: "major third up",
        5: "perfect fourth up", 6: "tritone up",
        7: "perfect fifth up", 8: "minor sixth up",
        9: "major sixth up", 10: "minor seventh up",
        11: "major seventh up", 12: "octave up"
    }
    return names.get(semitones, f"{semitones} semitones")

if __name__ == "__main__":
    import json
    
    # Demo: Agent state vector → music
    vectors = [
        [1, 0, -1, 1, 0, -1, 1, 1],
        [1, 1, 1, -1, -1, -1, 1, 1],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [-1, 1, -1, 1, -1, 1, -1, 1],
    ]
    
    for v in vectors:
        notes = vector_to_notes(v)
        sym = analyze_symmetry(v)
        print(f"Vector: {v}")
        print(f"  Notes (MIDI): {notes}")
        print(f"  Intervals: {[TERNARY_TO_INTERVAL.get(x, 0) for x in v]}")
        print(f"  Interval names: {[interval_name(TERNARY_TO_INTERVAL.get(x, 0)) for x in v]}")
        print(f"  Symmetry groups: {len(sym)}")
        for g in sym:
            print(f"    {g['type']}: index {g['left_index']} ↔ {g['right_index']}")
        print()
