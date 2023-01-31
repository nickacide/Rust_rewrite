use crate::bitboard::BitBoard;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PinMask {
    pub h: BitBoard,
    pub v: BitBoard,
    pub d1: BitBoard,
    pub d2: BitBoard    
}