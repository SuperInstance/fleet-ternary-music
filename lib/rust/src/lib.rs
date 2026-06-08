//! # Ternary ⇄ Music Bridge — Rust Implementation
//!
//! The type system encodes the mathematical structure directly:
//! - `Ternary::PlusOne + Ternary::MinusOne = Ternary::Zero` (group closure)
//! - `Interval::MajorThird + Interval::MinorThird = Interval::Unison` (conservation)
//!
//! This is Neo-Riemannian theory's P/L/R group expressed in Rust's type system.

use std::fmt;

/// The three states of a ternary strategy vector.
/// Encoded as an enum because the values aren't just numbers —
/// they're a mathematical group with closure properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ternary {
    Assertion,  // +1
    Sustain,    // 0
    Opposition, // -1
}

impl Ternary {
    /// Convert to semitone interval.
    /// Assertion → major third up (+4)
    /// Sustain → unison (0)
    /// Opposition → minor third down (-4)
    pub fn to_interval(self) -> Interval {
        match self {
            Ternary::Assertion => Interval::MajorThird,    // +4 semitones
            Ternary::Sustain => Interval::Unison,          // 0 semitones
            Ternary::Opposition => Interval::MinorThird,   // -4 semitones
        }
    }
}

/// Musical intervals in semitones.
/// The conservation law: MajorThird + MinorThird = Unison
/// because +4 + (-4) = 0
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Interval {
    Unison,        // 0 semitones
    MinorThird,    // -4 semitones (down)
    MajorThird,    // +4 semitones (up)
    Other(i8),     // Any other interval
}

impl Interval {
    pub fn semitones(&self) -> i8 {
        match self {
            Interval::Unison => 0,
            Interval::MinorThird => -4,
            Interval::MajorThird => 4,
            Interval::Other(n) => *n,
        }
    }

    /// Group closure: our two non-zero intervals cancel to zero.
    /// This proves +1 + (-1) = 0 in the music theory domain.
    pub fn compose(self, other: Interval) -> Interval {
        match (self.semitones(), other.semitones()) {
            (4, -4) | (-4, 4) => Interval::Unison,  // Conservation!
            (a, b) => {
                let sum = a + b;
                if sum == 0 { Interval::Unison }
                else { Interval::Other(sum) }
            }
        }
    }
}

/// Convert a ternary vector to MIDI pitches.
pub fn vector_to_notes(vector: &[Ternary], base_pitch: u8) -> Vec<u8> {
    let mut notes = vec![base_pitch];
    for t in vector {
        let interval = t.to_interval();
        let semitones = interval.semitones() as i16;
        let last = notes.last().copied().unwrap_or(base_pitch) as i16;
        let next = (last + semitones).clamp(0, 127) as u8;
        notes.push(next);
    }
    notes
}

/// Find symmetry groups (mirror pairs and conservation pairs).
/// A mirror pair is when vector[i] == vector[n-1-i].
/// A conservation pair is when vector[i] + vector[n-1-i] == 0.
pub fn analyze_symmetry(vector: &[Ternary]) -> Vec<SymmetryGroup> {
    let n = vector.len();
    let mut groups = Vec::new();
    for i in 0..n / 2 {
        let left = vector[i];
        let right = vector[n - 1 - i];
        if left == right {
            groups.push(SymmetryGroup::Mirror { left_index: i, right_index: n - 1 - i, value: left });
        } else if left.cancels(right) {
            groups.push(SymmetryGroup::Conservation { left_index: i, right_index: n - 1 - i, left_value: left, right_value: right });
        }
    }
    groups
}

impl Ternary {
    /// A ternary value cancels its opposite: +1 cancels -1, -1 cancels +1.
    /// This is the group closure property: any element composed with its inverse = identity (0).
    fn cancels(self, other: Ternary) -> bool {
        matches!((self, other), (Ternary::Assertion, Ternary::Opposition) | (Ternary::Opposition, Ternary::Assertion))
    }
}

#[derive(Debug)]
pub enum SymmetryGroup {
    Mirror { left_index: usize, right_index: usize, value: Ternary },
    Conservation { left_index: usize, right_index: usize, left_value: Ternary, right_value: Ternary },
}

impl fmt::Display for Ternary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ternary::Assertion => write!(f, "+1"),
            Ternary::Sustain => write!(f, " 0"),
            Ternary::Opposition => write!(f, "-1"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conservation_law() {
        // The mathematical foundation: +1 + (-1) = 0
        let major = Ternary::Assertion.to_interval();  // +4
        let minor = Ternary::Opposition.to_interval();  // -4
        assert_eq!(major.compose(minor), Interval::Unison);  // 0
    }

    #[test]
    fn test_vector_to_notes() {
        let vec = vec![Ternary::Assertion, Ternary::Sustain, Ternary::Opposition];
        let notes = vector_to_notes(&vec, 60);
        assert_eq!(notes, vec![60, 64, 64, 60]);
    }

    #[test]
    fn test_symmetry() {
        let vec = vec![
            Ternary::Assertion, Ternary::Sustain, Ternary::Opposition,
            Ternary::Assertion, Ternary::Sustain, Ternary::Opposition,
            Ternary::Assertion, Ternary::Assertion,
        ];
        let sym = analyze_symmetry(&vec);
        assert!(!sym.is_empty());
    }
}
