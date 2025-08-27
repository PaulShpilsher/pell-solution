//! Unit tests for the error module

use pell991::PellError;

#[test]
fn test_error_display() {
    assert_eq!(format!("{}", PellError::InvalidD(0)), "D must be > 1, got 0");
    assert_eq!(format!("{}", PellError::PerfectSquare(4)), "D must be non-square, got 4 which is 2²");
    assert_eq!(format!("{}", PellError::InvalidK(0)), "k must be > 0, got 0");
}

#[test]
fn test_error_debug() {
    let error1 = PellError::InvalidD(5);
    let error2 = PellError::PerfectSquare(9);
    let error3 = PellError::InvalidK(0);
    
    // Test that Debug is implemented
    assert!(format!("{:?}", error1).contains("InvalidD"));
    assert!(format!("{:?}", error2).contains("PerfectSquare"));
    assert!(format!("{:?}", error3).contains("InvalidK"));
}

#[test]
fn test_error_equality() {
    // Test PartialEq implementation
    assert_eq!(PellError::InvalidD(0), PellError::InvalidD(0));
    assert_eq!(PellError::PerfectSquare(4), PellError::PerfectSquare(4));
    assert_eq!(PellError::InvalidK(0), PellError::InvalidK(0));
    
    // Test inequality
    assert_ne!(PellError::InvalidD(0), PellError::InvalidD(1));
    assert_ne!(PellError::PerfectSquare(4), PellError::PerfectSquare(9));
    assert_ne!(PellError::InvalidK(0), PellError::InvalidK(1));
    
    // Test different error types
    assert_ne!(PellError::InvalidD(0), PellError::PerfectSquare(4));
    assert_ne!(PellError::PerfectSquare(4), PellError::InvalidK(0));
    assert_ne!(PellError::InvalidK(0), PellError::InvalidD(0));
}

#[test]
fn test_error_clone() {
    let error = PellError::InvalidD(42);
    let cloned = error.clone();
    assert_eq!(error, cloned);
}

#[test]
fn test_error_as_std_error() {
    use std::error::Error;
    
    let error = PellError::InvalidD(0);
    let _: &dyn Error = &error; // Should compile if Error trait is implemented
    
    // Test that source() returns None (no underlying cause)
    assert!(error.source().is_none());
}

#[test]
fn test_perfect_square_error_formatting() {
    // Test various perfect squares to ensure correct square root display
    let test_cases = [
        (4, "2"),
        (9, "3"),
        (16, "4"),
        (25, "5"),
        (100, "10"),
        (10000, "100"),
    ];
    
    for (square, expected_root) in test_cases {
        let error = PellError::PerfectSquare(square);
        let error_msg = format!("{}", error);
        assert!(error_msg.contains(expected_root), 
                "Error message '{}' should contain root '{}'", error_msg, expected_root);
        assert!(error_msg.contains(&format!("{}²", expected_root)),
                "Error message '{}' should contain '{}²'", error_msg, expected_root);
    }
}