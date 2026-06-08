"""STUDENT-FRIENDLY VERSION of the ternary⇄music bridge.

If you're new to music theory or ternary systems, start here.
Then graduate to bridge.py for the full theory."""

# The Simple Idea:
# Ternary means three values: -1, 0, +1
# Music intervals map to these:
#   +1 = go UP a major third (4 semitones, sounds happy)
#    0 = stay on the same note
#   -1 = go DOWN a minor third (4 semitones, sounds sad)

# Try it yourself:
# python3 -c "
# note = 60  # middle C
# for step in [1, 0, -1, 1, 0, -1, 1, 1]:
#     if step == 1: note += 4   # up a major third
#     elif step == -1: note -= 4 # down a minor third
#     print(f'Step {step}: MIDI note {note}')
# "

# This is the same math the fleet uses to turn agent strategies into music.
