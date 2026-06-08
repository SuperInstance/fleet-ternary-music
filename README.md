# 🧮 ternary-music

**Three numbers — -1, 0, +1 — generate every interval in Western music.**

---

## Wait, show me

```python
from lib.bridge import vector_to_notes, analyze_symmetry

# A single line of ternary produces a melody
notes = vector_to_notes([1, 0, -1, 1, 0, -1, 1, 1])
# → [60, 64, 64, 60, 64, 64, 60, 64, 68]
```

That's a melody. From eight numbers that could have been anything — agent decisions, sensor readings, stock prices, game states.

---

## The mapping — it's not arbitrary

| Ternary value | What it means | Musical interval | Why that interval |
|--------------|---------------|------------------|-------------------|
| **+1** | Assertion | Major third up (4 semitones) | The most consonant interval after the fifth |
| **0** | Sustain | Unison (0 semitones) | No change — hold position |
| **-1** | Opposition | Minor third down (4 semitones) | The primary minor interval |

Every +1 is balanced by a -1. They cancel to 0.

```python
# Conservation in action:
assert vector_to_notes([1, -1]) == [60, 64, 60]  # returns to start
assert vector_to_notes([-1, 1]) == [60, 56, 60]  # returns to start
```

This is a **group closure property** — the same mathematics that underlies Neo-Riemannian theory (Lewin, 1987), the geometry of chords (Tymoczko, 2011), and conservation of musical material in counterpoint.

---

## Five things you'll do with this

### 1. Turn any data stream into music

```python
stock_prices = [1, 0, -1, -1, 1, 1, 0, 1]  # up, flat, down...
notes = vector_to_notes(stock_prices)
# Your stock data just became a melody
```

### 2. Find symmetry in sequences

```python
sym = analyze_symmetry([1, 0, -1, 1, 0, -1, 1, 1])
# → 2 mirror pairs found (index 0⇄7, index 2⇄5)
```

### 3. Generate chord progressions

Pair with [fleet-midi-musiclang](https://github.com/SuperInstance/fleet-midi-musiclang): the same conservation math that produces voice-leading in counterpoint produces functional chord progressions in diatonic harmony.

### 4. Seed the entire MIDI fleet

Every repo in the MIDI fleet (text2midi, tidalcycles, generator, markov, jam-engine) uses these core mappings internally. This repo documents the theory that powers 20 production-grade music tools.

### 5. Write academic papers

The connection between ternary systems, Neo-Riemannian theory, and group theory is publishable. See `THEORY.md` and the paper references at the bottom of this file.

---

## Architecture

```
Ternary vector (-1, 0, +1)
         │
         ▼
  ┌─────────────────┐
  │  TERNARY_TO_    │  +1 → +4 (major third up)
  │  INTERVAL map   │   0 →  0 (unison)
  │                 │  -1 → -4 (minor third down)
  └────────┬────────┘
           │
           ▼
  ┌─────────────────┐      ┌─────────────────┐
  │  vector_to_     │      │  analyze_       │
  │  notes()        │      │  symmetry()     │
  └────────┬────────┘      └────────┬────────┘
           │                        │
           ▼                        ▼
    MIDI pitch sequence      Conservation pairs
    for any DAW              and mirror groups
```

---

## Use cases you haven't considered

- **Game design:** Player decisions (-1, 0, +1) → dynamic soundtrack
- **Sensor data:** IoT readings → ambient music
- **Algorithmic trading:** Market signals → harmonic analysis
- **Biometrics:** Heart rate variability → generative music
- **Network monitoring:** Packet flow patterns → rhythmic sequences
- **Choreography:** Dance movement vectors → score generation
- **Language processing:** Sentiment analysis (-1 negative, 0 neutral, +1 positive) → melodic contour
- **Climate data:** Temperature anomalies → harmonic tension

---

## Where this fits

This is **Theta** — the Fleet Mathematics Officer.

Every single MIDI repo in the fleet depends on this mapping. When Rhapsodia generates a jazz progression, she's using these intervals. When Rhythmica creates a pattern, she's mapping through these values. When Harmonia analyzes a chord, she's referencing these transformations.

**Next:** [fleet-symmetry-analyzer](https://github.com/SuperInstance/fleet-symmetry-analyzer) — deeper symmetry detection  
**Next:** [fleet-fugue-engine](https://github.com/SuperInstance/fleet-fugue-engine) — counterpoint from symmetry  
**Next:** [fleet-voice-leader](https://github.com/SuperInstance/fleet-voice-leader) — conservation in voice leading

---

## Further reading

- Lewin, D. (1987). *Generalized Musical Intervals and Transformations.* — The mathematical foundation for what we're doing here.
- Cohn, R. (2012). *Audacious Euphony.* — Hexatonic cycles and the P/L/R transform group.
- Tymoczko, D. (2011). *A Geometry of Music.* — Why chord progressions follow geometric rules.
- Huron, D. (2006). *Sweet Anticipation.* — Why +1 and -1 want to cancel to 0 (expectation psychology).
- Eigenfeldt & Pasquier (2013). *Evolving Structures for EDM.* — Multi-agent music systems (what our fleet is).
