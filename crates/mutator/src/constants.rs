/// Interesting values for the mutator. Taken from:
/// - https://github.com/AFLplusplus/AFLplusplus/blob/stable/include/config.h#L359
pub const INTERESTING_8: [u8; 29] = [
    0,   // 00000000
    1,   // 00000001
    2,   // 00000010
    3,   // 00000011
    4,   // 00000100
    7,   // 00000111
    8,   // 00001000
    10,  // 00001010
    15,  // 00001111
    16,  // 00010000
    31,  // 00011111
    32,  // 00100000
    50,  // 00110010
    63,  // 00111111
    64,  // 01000000
    85,  // 01010101
    100, // 01100100
    127, // 01111111
    128, // 10000000
    140, // 10001100
    156, // 10011100
    170, // 10101010
    195, // 11000011
    209, // 11010001
    224, // 11100000
    240, // 11110000
    251, // 11111011
    254, // 11111110
    255, // 11111111
];
