# Student Guide to fleet-ternary-music

## What is ternary?

Three states: -1 (down/oppose), 0 (neutral/hold), +1 (up/assert).

## What does this have to do with music?

Each state maps to a musical interval:
- **+1** = major third up (4 semitones — sounds bright)
- **0** = unison (0 semitones — same note)
- **-1** = minor third down (4 semitones — sounds dark)

So a strategy vector like `[1, 0, -1, 1]` becomes:
```
Start at C4 (60)
+1 → E4 (64)  — bright
0  → E4 (64)  — sustain
-1 → C4 (60)  — dark, returns home
+1 → E4 (64)  — bright again
```

## Try it

```bash
python3 lib/student_bridge.py
```
Or directly:
```python
python3 -c "
from lib.bridge import vector_to_notes
print(vector_to_notes([1,0,-1,1,0,-1,1,1]))
"
```
