use std::time::SystemTime;

use crate::{bitboard::BitBoard, PieceType};

pub struct Move {
    from: usize,
    to: usize,
    promotion: Option<PieceType>,
}

pub const fn between_rays(from: BitBoard, to: BitBoard) -> BitBoard {
    const fn between_rays(from: BitBoard, to: BitBoard) -> BitBoard {
        let dr = to.rank() - from.rank();
        let df = to.file() - from.file();
        let orthogonal = dr == 0 || df == 0;
        let diagonal = dr.abs() == df.abs();
        if !(orthogonal || diagonal) {
            return BitBoard::EMPTY;
        }
        let dr = dr.signum();
        let df = df.signum();
        let mut square = from.offset(dr, df);
        let mut between = BitBoard::EMPTY;
        while square.0 != to.0 {
            // dbg!(square.0);
            between.0 |= square.0;
            square = square.offset(dr, df);
        }
        between
    }
    // panic!("e!");
    const TABLE: [[BitBoard; 64]; 64] = {
        let mut from = 0usize;
        let mut to = 0usize;
        let mut table: [[BitBoard; 64]; 64] = [[BitBoard::EMPTY; 64]; 64];
        while from < 64 {
            while to < 64 {
                table[from][to] = between_rays(BitBoard(1u64 << from), BitBoard(1u64 << to));
                // println!("{} {} {}", from, to, table[from][to].0);
                to += 1;
            }
            to = 0;
            from += 1;
        }
        table
    };
    TABLE[from.first()][to.first()]
}
#[test]
fn time_table() {
    let between = between_rays(BitBoard(1), BitBoard(1 << 63));
    let between = between_rays(BitBoard(1), BitBoard(1 << 63));
    let between = between_rays(BitBoard(1), BitBoard(1 << 63));
    let start = SystemTime::now();
    let between = between_rays(BitBoard(1), BitBoard(1 << 63));
    let since = start.elapsed().expect(".").as_nanos();
    print!("{}ns", since);
}
