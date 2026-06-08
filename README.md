# 🧮 fleet-ternary-music

> *The mathematical bridge between ternary {-1,0,+1} and music theory*

Every fleet repo uses these core mappings internally. This repo documents
and implements the complete theory.

## Quick Start

```python
from lib.bridge import vector_to_notes, analyze_symmetry, interval_name

# Convert any strategy to notes
v = [1, 0, -1, 1, 0, -1, 1, 1]
notes = vector_to_notes(v)  # [60, 64, 64, 60, 64, 64, 60, 63, 67]

# Find symmetry
sym = analyze_symmetry(v)
```

## For Beginners

See `lib/student_bridge.py` for a gentle introduction.
Start there if the theory seems dense.

## For Engineers

See `THEORY.md` and `lib/bridge.py` for the complete framework.
This is the mathematical heart of the SuperInstance MIDI fleet.
