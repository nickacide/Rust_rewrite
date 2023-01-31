// use std::collections::HashMap;

use crate::bitboard::BitBoard;

const fn to_12x10(index: isize) -> isize {
    index + 21 + 2 * (index / 8)
}
const fn to_8x8(index: isize) -> isize {
    (index - 21) - 2 * ((index - 21) / 10)
}
const fn verify_index(index: isize) -> bool {
    if index < 21 || index > 119 {
        return false;
    }
    (index - 21) % 10 < 8
}

const fn king_lookup(piece_index: usize) -> BitBoard {
    let moves = [-11, -10, -9, -1, 1, 9, 10, 11];
    let mut result = 0u64;
    let mut m = 0;
    while m < moves.len() {
        let new = to_12x10(piece_index as isize) + moves[m];
        if verify_index(new) && to_8x8(new) < 64 {
            result |= 1 << to_8x8(new);
        }
        m += 1;
    }
    BitBoard(result)
}
const fn queen_lookup(piece_index: usize) -> BitBoard {
    let moves = [-11, -10, -9, -1, 1, 9, 10, 11];
    let mut result = 0u64;
    let mut m = 0;
    let mut multiplier = 1;
    while m < moves.len() {
        while multiplier < 8 {
            let new = to_12x10(piece_index as isize) + moves[m] * multiplier;
            if verify_index(new) && to_8x8(new) < 64 {
                result |= 1 << to_8x8(new);
            } else {
                // m += 1;
                break;
            };
            multiplier += 1;
        }
        multiplier = 0;
        m += 1;
    }
    BitBoard(result)
}
const fn rook_lookup(piece_index: usize) -> BitBoard {
    let moves = [-10, -1, 1, 10];
    let mut result = 0u64;
    let mut m = 0;
    let mut multiplier = 1;
    while m < moves.len() {
        while multiplier < 8 {
            let new = to_12x10(piece_index as isize) + moves[m] * multiplier;
            if verify_index(new) && to_8x8(new) < 64 {
                result |= 1 << to_8x8(new);
            } else {
                // m += 1;
                break;
            };
            multiplier += 1;
        }
        multiplier = 0;
        m += 1;
    }
    BitBoard(result)
}
const fn bishop_lookup(piece_index: usize) -> BitBoard {
    let moves = [-11, -9, 9, 11];
    let mut result = 0u64;
    let mut m = 0;
    let mut multiplier = 1;
    while m < moves.len() {
        while multiplier < 8 {
            let new = to_12x10(piece_index as isize) + moves[m] * multiplier;
            if verify_index(new) && to_8x8(new) < 64 {
                result |= 1 << to_8x8(new);
            } else {
                // m += 1;
                break;
            };
            multiplier += 1;
        }
        multiplier = 0;
        m += 1;
    }
    BitBoard(result)
}
const fn knight_lookup(piece_index: usize) -> BitBoard {
    let moves = [-21, -19, -12, -8, 8, 12, 19, 21];
    let mut result = 0u64;
    let mut m = 0;
    while m < moves.len() {
        let new = to_12x10(piece_index as isize) + moves[m];
        if verify_index(new) && to_8x8(new) < 64 {
            result |= 1 << to_8x8(new);
        }
        m += 1;
    }
    BitBoard(result)
}
const fn white_pawn_lookup(piece_index: usize) -> BitBoard {
    let moves = [11, 9];
    let mut result = 0u64;
    let mut m = 0;
    while m < moves.len() {
        let new = to_12x10(piece_index as isize) + moves[m];
        if verify_index(new) && to_8x8(new) < 64 {
            result |= 1 << to_8x8(new);
        }
        m += 1;
    }
    BitBoard(result)
}
const fn black_pawn_lookup(piece_index: usize) -> BitBoard {
    let moves = [-11, -9];
    let mut result = 0u64;
    let mut m = 0;
    while m < moves.len() {
        let new = to_12x10(piece_index as isize) + moves[m];
        if verify_index(new) && to_8x8(new) < 64 {
            result |= 1 << to_8x8(new);
        }
        m += 1;
    }
    BitBoard(result)
}

// const fn generate_key(key: u64) -> u64 {
//     let mut result = key;
//     let sq1 = key.trailing_zeros() as i32;
//     let rank1 = sq1 / 8;
//     let file1 = sq1 % 8;
//     let sq2 = (key & key - 1).trailing_zeros() as i32;
//     let rank2 = sq2 / 8;
//     let file2 = sq2 % 8;

//     if rank1 - rank2 == file2 - file1 {
//         let mut i = file2;
//         while i < file1 {
//             result |= 1 << (sq1 + (i - file2) * 7);
//             i += 1;
//         }
//     } else if rank1 - rank2 == file1 - file2 {
//         let mut i = file1;
//         while i < file2 {
//             result |= 1 << (sq1 + (i - file1) * 9);
//             i += 1;
//         }
//     } else if rank1 == rank2 {
//         let mut i = file1;
//         while i < file2 {
//             result |= 1 << (sq1 + i - file1);
//             i += 1;
//         }
//     } else if file1 == file2 {
//         let mut i = rank1;
//         while i < rank2 {
//             result |= 1 << (sq1 + (i - rank1) * 8);
//             i += 1;
//         }
//     } else {
//         // panic!("Invalid direction")
//     }
//     result
// }

// pub fn generate_slide_lookup() -> HashMap<BitBoard, BitBoard> {
//     let mut slide_lookup: HashMap<BitBoard, BitBoard> = HashMap::new();
//     let mut i = 0;
//     while i < 64 {
//         let mut slide_squares = queen_lookup(i).u64();
//         while slide_squares > 0 {
//             let key = 1u64 << i | 1u64 << slide_squares.trailing_zeros();
//             slide_lookup.insert(BitBoard(key), BitBoard(generate_key(key)));
//             slide_squares &= slide_squares - 1;
//         }
//         i += 1;
//     }
//     slide_lookup
// }
pub const fn generate_king_lookup() -> [BitBoard; 64] {
    let mut lookup_table: [BitBoard; 64] = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = king_lookup(i);
        i += 1;
    }
    lookup_table
}
pub const fn generate_queen_lookup() -> [BitBoard; 64] {
    let mut lookup_table: [BitBoard; 64] = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = queen_lookup(i);
        i += 1;
    }
    lookup_table
}
pub const fn generate_rook_lookup() -> [BitBoard; 64] {
    let mut lookup_table: [BitBoard; 64] = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = rook_lookup(i);
        i += 1;
    }
    lookup_table
}
pub const fn generate_bishop_lookup() -> [BitBoard; 64] {
    let mut lookup_table: [BitBoard; 64] = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = bishop_lookup(i);
        i += 1;
    }
    lookup_table
}
pub const fn generate_knight_lookup() -> [BitBoard; 64] {
    let mut lookup_table: [BitBoard; 64] = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = knight_lookup(i);
        i += 1;
    }
    lookup_table
}
pub const fn generate_white_pawn_lookup() -> [BitBoard; 64] {
    let mut lookup_table: [BitBoard; 64] = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = white_pawn_lookup(i);
        i += 1;
    }
    lookup_table
}
pub const fn generate_black_pawn_lookup() -> [BitBoard; 64] {
    let mut lookup_table: [BitBoard; 64] = [BitBoard(0); 64];
    let mut i = 0;
    while i < 64 {
        lookup_table[i] = black_pawn_lookup(i);
        i += 1;
    }
    lookup_table
}
