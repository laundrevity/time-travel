use backwards::SequenceValidator;
use std::collections::HashSet;

#[test]
fn test_valid_blinker_sequence() {
    let validator = SequenceValidator::new(3, 1);
    let mut sequence: Vec<HashSet<(i32, i32)>> = Vec::new();

    // Standard blinker sequence
    sequence.push([(-1, 0), (0, 0), (1, 0)].iter().cloned().collect()); // Initial state
    sequence.push([(0, -1), (0, 0), (0, 1)].iter().cloned().collect()); // Next state

    let status = validator.validate(sequence);
    assert!(status.is_valid);
    assert_eq!(status.exceptions, 0);
}

#[test]
fn test_invalid_blinker_sequence_with_exception() {
    let validator = SequenceValidator::new(3, 1);
    let mut sequence: Vec<HashSet<(i32, i32)>> = Vec::new();

    // Modified blinker sequence with an extra cell
    sequence.push([(-1, 0), (0, 0), (1, 0)].iter().cloned().collect()); // Initial state
    sequence.push([(0, -1), (0, 0), (0, 1), (1, 0)].iter().cloned().collect()); // Next state with extra cell

    let status = validator.validate(sequence);
    assert!(!status.is_valid);
    assert_eq!(status.exceptions, 1);
}