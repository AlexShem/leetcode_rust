// Constraints:
//
// 1 <= size <= 105
// 0 <= idx <= size - 1
// At most 105 calls will be made in total to fix, unfix, flip, all, one, count, and toString.
// At least one call will be made to all, one, count, or toString.
// At most 5 calls will be made to toString.

use lc_core::solutions::bitset::Bitset;

#[test]
fn test_leetcode_example() {
    /*
    Bitset bs = new Bitset(5); // bitset = "00000".
bs.fix(3);     // the value at idx = 3 is updated to 1, so bitset = "00010".
bs.fix(1);     // the value at idx = 1 is updated to 1, so bitset = "01010".
bs.flip();     // the value of each bit is flipped, so bitset = "10101".
bs.all();      // return False, as not all values of the bitset are 1.
bs.unfix(0);   // the value at idx = 0 is updated to 0, so bitset = "00101".
bs.flip();     // the value of each bit is flipped, so bitset = "11010".
bs.one();      // return True, as there is at least 1 index with value 1.
bs.unfix(0);   // the value at idx = 0 is updated to 0, so bitset = "01010".
bs.count();    // return 2, as there are 2 bits with value 1.
bs.toString(); // return "01010", which is the composition of bitset.
     */
    let mut bitset = Bitset::new(5);
    bitset.fix(3);
    assert_eq!(bitset.to_string(), "00010");

    bitset.fix(1);     // the value at idx = 1 is updated to 1, so bitset = "01010".
    assert_eq!(bitset.to_string(), "01010");

    bitset.flip();     // the value of each bit is flipped, so bitset = "10101".
    assert_eq!(bitset.to_string(), "10101");

    assert!(!bitset.all());

    bitset.unfix(0);   // the value at idx = 0 is updated to 0, so bitset = "00101".
    assert_eq!(bitset.to_string(), "00101");

    bitset.flip();     // the value of each bit is flipped, so bitset = "11010".
    assert_eq!(bitset.to_string(), "11010");

    assert!(bitset.one());      // return True, as there is at least 1 index with value 1.

    bitset.unfix(0);   // the value at idx = 0 is updated to 0, so bitset = "01010".
    assert_eq!(bitset.to_string(), "01010");

    assert_eq!(bitset.count(), 2);    // return 2, as there are 2 bits with value 1.

    bitset.to_string(); // return "01010", which is the composition of bitset.
    assert_eq!(bitset.to_string(), "01010");
}

#[test]
fn test_new_bitset() {
    let bitset = Bitset::new(5);
    assert_eq!(bitset.to_string(), "00000");
    assert_eq!(bitset.count(), 0);
    assert!(!bitset.one());
    assert!(!bitset.all());
}

#[test]
fn test_fix_operations() {
    let mut bitset = Bitset::new(5);

    // Fix a single bit
    bitset.fix(2);
    assert_eq!(bitset.to_string(), "00100");
    assert_eq!(bitset.count(), 1);
    assert!(bitset.one());
    assert!(!bitset.all());

    // Fix another bit
    bitset.fix(0);
    assert_eq!(bitset.to_string(), "10100");
    assert_eq!(bitset.count(), 2);

    // Fix a bit that's already fixed
    bitset.fix(2);
    assert_eq!(bitset.to_string(), "10100");
    assert_eq!(bitset.count(), 2);

    // Fix all bits
    bitset.fix(1);
    bitset.fix(3);
    bitset.fix(4);
    assert_eq!(bitset.to_string(), "11111");
    assert_eq!(bitset.count(), 5);
    assert!(bitset.one());
    assert!(bitset.all());
}

#[test]
fn test_unfix_operations() {
    let mut bitset = Bitset::new(5);

    // Fix all bits
    bitset.fix(0);
    bitset.fix(1);
    bitset.fix(2);
    bitset.fix(3);
    bitset.fix(4);

    // Unfix a bit
    bitset.unfix(2);
    assert_eq!(bitset.to_string(), "11011");
    assert_eq!(bitset.count(), 4);
    assert!(bitset.one());
    assert!(!bitset.all());

    // Unfix another bit
    bitset.unfix(0);
    assert_eq!(bitset.to_string(), "01011");
    assert_eq!(bitset.count(), 3);

    // Unfix a bit that's already unfixed
    bitset.unfix(0);
    assert_eq!(bitset.to_string(), "01011");
    assert_eq!(bitset.count(), 3);

    // Unfix all bits
    bitset.unfix(1);
    bitset.unfix(3);
    bitset.unfix(4);
    assert_eq!(bitset.to_string(), "00000");
    assert_eq!(bitset.count(), 0);
    assert!(!bitset.one());
    assert!(!bitset.all());
}

#[test]
fn test_flip_operations() {
    let mut bitset = Bitset::new(5);

    // Initial state
    assert_eq!(bitset.to_string(), "00000");

    // Flip all bits
    bitset.flip();
    assert_eq!(bitset.to_string(), "11111");
    assert_eq!(bitset.count(), 5);
    assert!(bitset.one());
    assert!(bitset.all());

    // Fix and unfix some bits
    bitset.unfix(1);
    bitset.unfix(3);
    assert_eq!(bitset.to_string(), "10101");

    // Flip again
    bitset.flip();
    assert_eq!(bitset.to_string(), "01010");
    assert_eq!(bitset.count(), 2);
    assert!(bitset.one());
    assert!(!bitset.all());
}

#[test]
fn test_combined_operations() {
    let mut bitset = Bitset::new(10);

    // Test a sequence of operations
    bitset.fix(3);
    assert_eq!(bitset.to_string(), "0001000000");
    bitset.fix(1);
    assert_eq!(bitset.to_string(), "0101000000");
    bitset.flip();
    assert_eq!(bitset.to_string(), "1010111111");
    assert_eq!(bitset.count(), 8);

    bitset.unfix(7);
    assert_eq!(bitset.to_string(), "1010111011");
    bitset.unfix(1);
    assert_eq!(bitset.to_string(), "1010111011");
    assert_eq!(bitset.count(), 7);

    bitset.flip();
    assert_eq!(bitset.to_string(), "0101000100");
    assert_eq!(bitset.count(), 3);
    assert!(bitset.one());
    assert!(!bitset.all());

    // Fix all bits
    for i in 0..10 {
        bitset.fix(i);
    }
    assert_eq!(bitset.to_string(), "1111111111");
    assert_eq!(bitset.count(), 10);
    assert!(bitset.all());
}

#[test]
fn test_edge_cases() {
    // Test with size 1
    let mut bitset = Bitset::new(1);
    assert_eq!(bitset.to_string(), "0");

    bitset.fix(0);
    assert_eq!(bitset.to_string(), "1");
    assert!(bitset.all());

    bitset.flip();
    assert_eq!(bitset.to_string(), "0");
    assert!(!bitset.one());

    // Test with larger size
    let mut large_bitset = Bitset::new(100);
    large_bitset.fix(50);
    assert_eq!(large_bitset.count(), 1);
    assert!(large_bitset.one());

    // Only the 50th bit should be set to 1
    let s = large_bitset.to_string();
    assert_eq!(s.len(), 100);
    assert_eq!(s.chars().filter(|&c| c == '1').count(), 1);
}
