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

## 📚 Further Reading

### Neo-Riemannian Theory
The mathematical basis for our ternary↔interval mappings. 
Neo-Riemannian theory defines transformations between consonant triads using
minimal voice-leading steps — exactly the same group structure as our -1/0/+1 system.

**Key paper:** Lewin, D. (1987). *Generalized Musical Intervals and Transformations.* Yale University Press.

### Symmetry Groups in Music
Our `analyze_symmetry()` function detects palindromes and conservation pairs
in agent state sequences. This mirrors the P (parallel), L (leading-tone exchange),
and R (relative) transforms of Neo-Riemannian theory.

**Key paper:** Tymoczko, D. (2011). *A Geometry of Music.* Oxford University Press.

### Conservation Laws
The principle that +1 + (-1) = 0 in our system mirrors conservation of musical
material — the foundation of counterpoint and fugue writing.

**Key paper:** Huron, D. (2006). *Sweet Anticipation: Music and the Psychology of Expectation.* MIT Press.

### Agent-Based Music Generation
Our fleet's ensign pattern mirrors multi-agent music systems.

**Key paper:** Eigenfeldt, A., & Pasquier, P. (2013). "Evolving Structures for Electronic Dance Music."
*Proceedings of the International Conference on Computational Creativity.*
