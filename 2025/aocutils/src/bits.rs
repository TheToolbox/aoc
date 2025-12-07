/// Get the value of a specific bit (0 or 1)
///
/// # Example
/// ```
/// use AOCUtils::bits::get_bit;
/// assert_eq!(get_bit(0b1010, 1), 1);
/// assert_eq!(get_bit(0b1010, 2), 0);
/// ```
pub fn get_bit(num: u64, position: u32) -> u64 {
    (num >> position) & 1
}

/// Set a specific bit to 1
///
/// # Example
/// ```
/// use AOCUtils::bits::set_bit;
/// assert_eq!(set_bit(0b1010, 2), 0b1110);
/// ```
pub fn set_bit(num: u64, position: u32) -> u64 {
    num | (1 << position)
}

/// Clear a specific bit (set to 0)
///
/// # Example
/// ```
/// use AOCUtils::bits::clear_bit;
/// assert_eq!(clear_bit(0b1010, 1), 0b1000);
/// ```
pub fn clear_bit(num: u64, position: u32) -> u64 {
    num & !(1 << position)
}

/// Toggle a specific bit
///
/// # Example
/// ```
/// use AOCUtils::bits::toggle_bit;
/// assert_eq!(toggle_bit(0b1010, 1), 0b1000);
/// assert_eq!(toggle_bit(0b1010, 2), 0b1110);
/// ```
pub fn toggle_bit(num: u64, position: u32) -> u64 {
    num ^ (1 << position)
}

/// Count the number of set bits (1s) in a number
///
/// # Example
/// ```
/// use AOCUtils::bits::count_ones;
/// assert_eq!(count_ones(0b1010), 2);
/// assert_eq!(count_ones(0b1111), 4);
/// ```
pub fn count_ones(num: u64) -> u32 {
    num.count_ones()
}

/// Count the number of clear bits (0s) in the lower n bits
///
/// # Example
/// ```
/// use AOCUtils::bits::count_zeros;
/// assert_eq!(count_zeros(0b1010, 4), 2);
/// ```
pub fn count_zeros(num: u64, bits: u32) -> u32 {
    bits - count_ones(num & ((1 << bits) - 1))
}

/// Check if a specific bit is set
///
/// # Example
/// ```
/// use AOCUtils::bits::is_bit_set;
/// assert_eq!(is_bit_set(0b1010, 1), true);
/// assert_eq!(is_bit_set(0b1010, 2), false);
/// ```
pub fn is_bit_set(num: u64, position: u32) -> bool {
    get_bit(num, position) == 1
}

/// Extract a range of bits from a number
///
/// # Example
/// ```
/// use AOCUtils::bits::extract_bits;
/// // Extract bits 1-3 from 0b1010 (which is 0b101)
/// assert_eq!(extract_bits(0b1010, 1, 3), 0b101);
/// ```
pub fn extract_bits(num: u64, start: u32, end: u32) -> u64 {
    let mask = (1 << (end - start + 1)) - 1;
    (num >> start) & mask
}

/// Reverse the bits in a number (for n-bit numbers)
///
/// # Example
/// ```
/// use AOCUtils::bits::reverse_bits;
/// assert_eq!(reverse_bits(0b1010, 4), 0b0101);
/// ```
pub fn reverse_bits(num: u64, bits: u32) -> u64 {
    let mut result = 0u64;
    let mut n = num;
    for i in 0..bits {
        result |= (n & 1) << (bits - 1 - i);
        n >>= 1;
    }
    result
}
