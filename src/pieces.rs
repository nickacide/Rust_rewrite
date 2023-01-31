use std::str::FromStr;

use crate::bitboard::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pieces {
    pub king: BitBoard,
    pub queen: BitBoard,
    pub rook: BitBoard,
    pub bishop: BitBoard,
    pub knight: BitBoard,
    pub pawn: BitBoard,

    pub white: BitBoard,
    pub black: BitBoard,
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseFenError;
impl FromStr for Pieces {
    type Err = ParseFenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut king = BitBoard::EMPTY;
        let mut queen = BitBoard::EMPTY;
        let mut rook = BitBoard::EMPTY;
        let mut bishop = BitBoard::EMPTY;
        let mut knight = BitBoard::EMPTY;
        let mut pawn = BitBoard::EMPTY;

        let mut white = BitBoard::EMPTY;
        let mut black = BitBoard::EMPTY;

        let parsed: Vec<&str> = s.split("/").collect();
        let mut rank_count = 0;
        let mut offset = 0;
        for rank in parsed {
            assert!(rank_count <= 7);
            for sq in 0..rank.len() {
                let square = rank.as_bytes()[sq] as char;
                let index = (7 - rank_count) * 8 + sq + offset;
                match square {
                    'K' => {
                        king |= BitBoard::new(index);
                        white |= BitBoard::new(index);
                    }
                    'Q' => {
                        queen |= BitBoard::new(index);
                        white |= BitBoard::new(index);
                    }
                    'R' => {
                        rook |= BitBoard::new(index);
                        white |= BitBoard::new(index);
                    }
                    'B' => {
                        bishop |= BitBoard::new(index);
                        white |= BitBoard::new(index);
                    }
                    'N' => {
                        knight |= BitBoard::new(index);
                        white |= BitBoard::new(index);
                    }
                    'P' => {
                        pawn |= BitBoard::new(index);
                        white |= BitBoard::new(index);
                    }
                    'k' => {
                        king |= BitBoard::new(index);
                        black |= BitBoard::new(index);
                    }
                    'q' => {
                        queen |= BitBoard::new(index);
                        black |= BitBoard::new(index);
                    }
                    'r' => {
                        rook |= BitBoard::new(index);
                        black |= BitBoard::new(index);
                    }
                    'b' => {
                        bishop |= BitBoard::new(index);
                        black |= BitBoard::new(index);
                    }
                    'n' => {
                        knight |= BitBoard::new(index);
                        black |= BitBoard::new(index);
                    }
                    'p' => {
                        pawn |= BitBoard::new(index);
                        black |= BitBoard::new(index);
                    }
                    ' ' => break,
                    int => offset += int as usize - 49,
                }
            }
            rank_count += 1;
            offset = 0;
        }
        Ok(Pieces {
            king,
            queen,
            rook,
            bishop,
            knight,
            pawn,
            white,
            black,
        })
    }
}
