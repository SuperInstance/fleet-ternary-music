# 🌐 SuperInstance Fleet — Complete Cross-Reference Matrix

> *Every repo's relationship to the ternary⇄music bridge*

## How to Read

Each repo below connects to the ternary→music mapping in a specific way.
The "Role" column explains the connection. The "Chain" column shows the data flow.

## Core Fleet (8 repos)

| Repo | Role in Chain | Connected to MIDI Fleet |
|------|---------------|----------------------|
| [fleet-bridge](https://github.com/SuperInstance/fleet-bridge) | I2I bottle transport for all MIDI tokens | All 20 MIDI repos speak I2I |
| [tminus-dispatcher](https://github.com/SuperInstance/tminus-dispatcher) | Timed dispatch of agent states = timed music events | jam-engine (tempo-synced beats) |
| [tminus-client](https://github.com/SuperInstance/tminus-client) | Receives dispatched events | musiclang (state→chord timing) |
| [composite-headspace](https://github.com/SuperInstance/composite-headspace) | Parallel reasoning (2 models) = fugue (2 voices) | fugue-engine (voice canons) |
| [symphony-runtime](https://github.com/SuperInstance/symphony-runtime) | Agent orchestration = composition conducting | music-theorist (analysis) |
| [i2i-bottle-agent](https://github.com/SuperInstance/i2i-bottle-agent) | Structured bottle protocol for agent comms | Every MIDI output is an I2I bottle |
| [constraint-tminus-bridge](https://github.com/SuperInstance/constraint-tminus-bridge) | Constraints on timed events = musical structure | fugue-engine (canon constraints) |
| [symphony-orchestrator](https://github.com/SuperInstance/symphony-orchestrator) | Multi-agent workflow = full orchestra | jam-engine (full band arrangement) |

## Ternary Math (15 repos)

| Repo | Role in Chain | Connected to MIDI Fleet |
|------|---------------|----------------------|
| [ternary-rhythm](https://github.com/SuperInstance/ternary-rhythm) | Rhythm pattern analysis (Rust) | tidalcycles (Python pattern rendering) |
| [ternary-matmul](https://github.com/SuperInstance/ternary-matmul) | Matrix multiplication on ternary | tokenizer (token sequence transformation) |
| [ternary-norm](https://github.com/SuperInstance/ternary-norm) | Normalization of states | music-theorist (normalized analysis) |
| [ternary-conv](https://github.com/SuperInstance/ternary-conv) | Convolution over state sequences | generator (convolution→completion) |
| [ternary-activation](https://github.com/SuperInstance/ternary-activation) | Activation functions for state transitions | markov (state transition probabilities) |
| [ternary-loss](https://github.com/SuperInstance/ternary-loss) | Loss functions = dissonance metrics | music-theorist (interval dissonance) |
| [ternary-quantize](https://github.com/SuperInstance/ternary-quantize) | Quantization of values | tokenizer (MIDI quantization to tokens) |
| [ternary-pool](https://github.com/SuperInstance/ternary-pool) | Pooling operations = pattern summarization | markov (pattern extraction from sequences) |
| [ternary-optimizer](https://github.com/SuperInstance/ternary-optimizer) | Optimization of states | voice-leader (optimal voice paths) |
| [ternary-em](https://github.com/SuperInstance/ternary-em) | Expectation-maximization | music-theorist (most likely key/scale) |
| [ternary-logistic](https://github.com/SuperInstance/ternary-logistic) | Logistic regression on states | markov (transition probabilities) |
| [ternary-regression](https://github.com/SuperInstance/ternary-regression) | Regression analysis = motion prediction | generator (predicting next note) |
| [ternary-svm](https://github.com/SuperInstance/ternary-svm) | Classification of state patterns | symmetry-analyzer (pattern classification) |
| [ternary-bite](https://github.com/SuperInstance/ternary-bite) | Bit-level ternary encoding | tokenizer (compact token encoding) |
| [ternary-checkpoint](https://github.com/SuperInstance/ternary-checkpoint) | State persistence = musical memory | music-theorist (historical analysis) |

## MIDI Fleet (20 repos)

| Repo | Role | Input → Output | Connected To |
|------|------|---------------|-------------|
| text2midi | Text→MIDI | text → MIDI file | All (foundation) |
| tidalcycles | Ternary→rhythm | vector → TidalCycles | ternary-rhythm, markov |
| musiclang | State→chords | states → progression | composer-analysis |
| generator | States→MIDI | sequence → MIDI file | markov, ternary-conv |
| tokenizer | MIDI↔tokens | MIDI ⟷ tokens | All (lingua franca) |
| markov | Statistical gen | seed → sequence | generator, text2midi |
| sheet-music | MIDI→score | MIDI → LilyPond | All (output) |
| visualizer | MIDI→SVG | MIDI → SVG | All (output) |
| jam-engine | Text→band | prompt → 3-track MIDI | All (full band) |
| player | MIDI→audio | MIDI → WAV | All (output) |
| theorist | MIDI→analysis | MIDI → JSON theory | All (analysis) |
| ternary-music | Math bridge | theory → reference | All (foundation) |
| symmetry-analyzer | Pattern detection | vector → symmetry groups | conservation-law-v2 |
| fugue-engine | Voice canons | states → counterpoint | composite-headspace |
| voice-leader | Voice leading | vector → optimal paths | ternary-optimizer |
| osc-server | Real-time | states → OSC | sonicpi, foxdot |

## Cross-Pollination Super-Chains

### Engineer's Toolchain
```
text → text2midi → MIDI → player → audio
                        → visualizer → SVG
                        → sheet-music → PDF
                        → theorist → analysis JSON
```

### Agent's Cognitive Pipeline
```
state → musiclang → chords
     → tidalcycles → rhythm
     → generator → melody
     → jam-engine → full band
```

### Mathematician's Research Chain
```
ternary-rhythm → pattern analysis
ternary-matmul → transform patterns
ternary-norm → normalize
ternary-conv → convolve sequences
fleet-ternary-music → map to intervals
fleet-music-theorist → analyze results
```

### Real-Time Performance Chain
```
composite-headspace → parallel reasoning
  → fugue-engine → voice canons
  → osc-server → Sonic Pi / FoxDot
  → live audio
```
