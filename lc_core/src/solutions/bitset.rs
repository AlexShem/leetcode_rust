/// # 2166. Design Bitset
/// **Difficulty**: *Medium*
///
/// A **Bitset** is a data structure that compactly stores bits.
///
/// Implement the `Bitset` class:
///
/// - `Bitset(int size)` Initializes the Bitset with `size` bits, all of which are `0`.
/// - `void fix(int idx)` Updates the value of the bit at the index `idx` to `1`. If the value was already `1`, no change occurs.
/// - `void unfix(int idx)` Updates the value of the bit at the index `idx` to `0`. If the value was already `0`, no change occurs.
/// - `void flip()` Flips the values of each bit in the Bitset. In other words, all bits with value `0` will now have value `1` and vice versa.
/// - `boolean all()` Checks if the value of each bit in the Bitset is `1`. Returns `true` if it satisfies the condition, `false` otherwise.
/// - `boolean one()` Checks if there is at least one bit in the Bitset with value `1`. Returns `true` if it satisfies the condition, `false` otherwise.
/// - `int count()` Returns the total number of bits in the Bitset which have value `1`.
/// - `String toString()` Returns the current composition of the Bitset. Note that in the resultant string, the character at the `ith` index should coincide with the value at the `ith` bit of the Bitset.
pub struct Bitset {
    /// Each block contains 64 bits
    blocks: Vec<u64>,
    /// Total number of bits
    size: usize,
    /// Lazy flip flag
    flipped: bool,
    /// Current count of `1`-bits
    ones_count: usize,
}

impl Bitset {
    pub fn new(size: i32) -> Self {
        let size = size as usize;
        /*
        `size`==1 => num_blocks=1;
        `size`==63 => num_blocks=1;
        `size`==64 => num_blocks=2;
        */
        let num_blocks = (size + 63) / 64;
        Bitset { blocks: vec![0; num_blocks], size, flipped: false, ones_count: 0 }
    }

    /// Updates the value of the bit at the index `idx` to `1`. If the value was already `1`, no change occurs.
    pub fn fix(&mut self, idx: i32) {
        let idx = idx as usize;
        let block_idx = idx / 64;
        let bit_idx = idx % 64;
        let mask = 1u64 << bit_idx;

        let is_set: bool = if !self.flipped {
            self.blocks[block_idx] & mask != 0
        } else {
            self.blocks[block_idx] & mask == 0
        };

        if !is_set {
            match self.flipped {
                true => { self.blocks[block_idx] &= !mask }
                false => { self.blocks[block_idx] |= mask }
            }
            self.ones_count += 1;
        }
    }

    /// Updates the value of the bit at the index `idx` to `0`. If the value was already `0`, no change occurs.
    pub fn unfix(&mut self, idx: i32) {
        let idx = idx as usize;
        let block_idx = idx / 64;
        let bit_idx = idx % 64;
        let mask = 1u64 << bit_idx;

        let is_set: bool = if !self.flipped {
            self.blocks[block_idx] & mask != 0
        } else {
            self.blocks[block_idx] & mask == 0
        };

        if is_set {
            match self.flipped {
                true => { self.blocks[block_idx] |= mask }
                false => { self.blocks[block_idx] &= !mask }
            }
            self.ones_count -= 1;
        }
    }

    /// Flips the values of each bit in the Bitset. In other words, all bits with value `0` will now have value `1` and vice versa.
    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
        self.ones_count = self.size - self.ones_count;
    }

    /// Checks if the value of each bit in the Bitset is `1`. Returns `true` if it satisfies the condition, `false` otherwise.
    pub fn all(&self) -> bool {
        self.ones_count == self.size
    }

    /// Checks if there is at least one bit in the Bitset with value `1`. Returns `true` if it satisfies the condition, `false` otherwise.
    pub fn one(&self) -> bool {
        self.ones_count > 0
    }

    /// Returns the total number of bits in the Bitset which have value `1`.
    pub fn count(&self) -> i32 {
        self.ones_count as i32
    }

    /// Returns the current composition of the Bitset. Note that in the resultant string, the character at the `ith` index should coincide with the value at the `ith` bit of the Bitset.
    pub fn to_string(&self) -> String {
        let mut res = String::with_capacity(self.size);

        for idx in 0..self.size {
            let block_idx = idx / 64;
            let bit_idx = idx % 64;
            let mask = 1u64 << bit_idx;

            let bit: bool = if self.flipped {
                self.blocks[block_idx] & mask == 0
            } else {
                self.blocks[block_idx] & mask != 0
            };

            res.push(if bit { '1' } else { '0' });
        }
        res
    }
}
