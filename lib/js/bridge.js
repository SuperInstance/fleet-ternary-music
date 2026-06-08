/**
 * Ternary ⇄ Music Bridge — JavaScript Implementation
 * 
 * JavaScript's dynamic type system reveals the mathematical structure
 * through function composition and array transformations.
 * The math is the same whether in typed or untyped languages.
 */

// The ternary operations as a plain object (composable, transformable)
const TERNARY = {
  1:  { name: 'assertion',  semitones: 4,  interval: 'major third up',   feel: 'bright' },
  0:  { name: 'sustain',    semitones: 0,  interval: 'unison',          feel: 'neutral' },
  '-1': { name: 'opposition', semitones: -4, interval: 'minor third down', feel: 'dark' },
};

/**
 * Convert ternary vector to MIDI pitches.
 * Pure function — no side effects, same input always produces same output.
 */
function vectorToNotes(vector, basePitch = 60) {
  return vector.reduce((notes, v) => {
    const semitones = (TERNARY[v] || TERNARY[0]).semitones;
    const last = notes[notes.length - 1];
    notes.push(Math.max(0, Math.min(127, last + semitones)));
    return notes;
  }, [basePitch]);
}

/**
 * Find symmetry groups (mirror and conservation pairs).
 * Reveals the palindrome structure in decision sequences.
 */
function analyzeSymmetry(vector) {
  const groups = [];
  for (let i = 0; i < Math.floor(vector.length / 2); i++) {
    const left = vector[i];
    const right = vector[vector.length - 1 - i];
    if (left === right) {
      groups.push({ type: 'mirror', leftIndex: i, rightIndex: vector.length - 1 - i, value: left });
    } else if (Number(left) + Number(right) === 0) {
      groups.push({ type: 'conservation', leftIndex: i, rightIndex: vector.length - 1 - i, leftValue: left, rightValue: right });
    }
  }
  return groups;
}

/**
 * The conservation law as a function: +1 + (-1) = 0
 */
function conserve(a, b) {
  return Number(a) + Number(b) === 0 ? 0 : Number(a) + Number(b);
}

/**
 * Generate the full theory analysis of a vector
 */
function analyze(vector) {
  const notes = vectorToNotes(vector);
  const sym = analyzeSymmetry(vector);
  const posCount = vector.filter(v => Number(v) === 1).length;
  const negCount = vector.filter(v => Number(v) === -1).length;
  const density = (posCount + negCount) / vector.length;
  const balance = (posCount - negCount) / vector.length;

  return {
    vector: vector.map(Number),
    notes,
    intervals: vector.map(v => TERNARY[v]?.interval || 'unknown'),
    symmetry: sym,
    stats: { length: vector.length, density, balance, posCount, negCount },
    summary: `Vector [${vector.join(',')}] → ${notes.length} notes, ${sym.length} symmetry groups`
  };
}

// Export for Node.js
if (typeof module !== 'undefined') module.exports = { vectorToNotes, analyzeSymmetry, conserve, analyze, TERNARY };

// CLI mode
if (typeof require !== 'undefined' && require.main === module) {
  const vec = [1, 0, -1, 1, 0, -1, 1, 1];
  console.log(JSON.stringify(analyze(vec), null, 2));
}
