use std::str::FromStr;

use crate::moves::{between_rays, Move};
use crate::pin::PinMask;
use crate::{bitboard::BitBoard, color::Color, gamestate::GameState, pieces::Pieces, PieceType};
use crate::{BISHOP_LOOKUP, BLACK_PAWN_LOOKUP, KNIGHT_LOOKUP, ROOK_LOOKUP, WHITE_PAWN_LOOKUP};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub pieces: Pieces,
    pub side_to_move: Color,
    pub halfmoves: usize,
    pub fullmoves: usize,
    pub state: GameState,
}
pub struct ParseFenError;
impl FromStr for Board {
    type Err = ParseFenError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces = s.parse::<Pieces>().ok().unwrap();
        let state = s.parse::<GameState>().ok().unwrap();
        let parsed: Vec<&str> = s.split(" ").collect();
        let side_to_move = match parsed[1].to_lowercase().as_str() {
            "w" => Color::White,
            "b" => Color::Black,
            _ => {
                panic!("invalid color")
            }
        };
        let halfmoves = parsed[4].chars().next().unwrap() as usize - 48; // '0'
        let fullmoves = parsed[5].chars().next().unwrap() as usize - 48;

        Ok(Board {
            pieces,
            state,
            halfmoves,
            fullmoves,
            side_to_move,
        })
    }
}
impl Default for Board {
    fn default() -> Self {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
            .parse::<Board>()
            .ok()
            .unwrap()
    }
}
impl Board {
    pub const fn colors(&self, color: Color) -> BitBoard {
        match color {
            Color::White => self.pieces.white,
            Color::Black => self.pieces.black,
        }
    }
    pub const fn pieces(&self, piece_type: PieceType) -> BitBoard {
        match piece_type {
            PieceType::King => self.pieces.king,
            PieceType::Queen => self.pieces.queen,
            PieceType::Rook => self.pieces.rook,
            PieceType::Bishop => self.pieces.bishop,
            PieceType::Knight => self.pieces.knight,
            PieceType::Pawn => self.pieces.pawn,
        }
    }
    pub const fn occupied(&self) -> BitBoard {
        BitBoard(self.pieces.white.0 | self.pieces.black.0)
    }
    pub const fn pawn_attacks(&self, piece_index: usize, color: Color) -> BitBoard {
        match color {
            Color::White => WHITE_PAWN_LOOKUP[piece_index],
            Color::Black => BLACK_PAWN_LOOKUP[piece_index],
        }
    }
    pub const fn checkers_pinners(&self, color: Color) -> (BitBoard, BitBoard) {
        let our_king = self.colors(color).u64() & self.pieces(PieceType::King).u64();
        let their_pieces = match color {
            Color::White => self.colors(Color::Black).0,
            Color::Black => self.colors(Color::White).0,
        };

        let mut checkers = 0u64;
        let mut pinners = 0u64;
        let mut attackers = ROOK_LOOKUP[our_king.trailing_zeros() as usize].0
            & (self.pieces.rook.0 | self.pieces.queen.0)
            & their_pieces;
        attackers |= BISHOP_LOOKUP[our_king.trailing_zeros() as usize].0
            & (self.pieces.bishop.0 | self.pieces.queen.0)
            & their_pieces;
        // println!(
        //     "attackers {} {} {} {} {}",
        //     attackers,
        //     their_pieces,
        //     self.pieces.bishop.0,
        //     self.pieces.queen.0,
        //     BISHOP_LOOKUP[our_king.trailing_zeros() as usize].0
        // );
        while attackers > 0 {
            let blockers = between_rays(
                BitBoard(1 << attackers.trailing_zeros()),
                BitBoard(our_king),
            )
            .0 & self.occupied().0
                & !(our_king | 1u64 << attackers.trailing_zeros());
            match blockers.count_ones() {
                0 => checkers |= 1 << attackers.trailing_zeros(),
                1 => pinners |= 1 << attackers.trailing_zeros(),
                _ => {}
            }
            attackers &= attackers - 1;
        }
        checkers |= KNIGHT_LOOKUP[our_king.trailing_zeros() as usize].0 & their_pieces;
        checkers |= self
            .pawn_attacks(our_king.trailing_zeros() as usize, color)
            .0
            & self.pieces.pawn.0
            & their_pieces;
        (BitBoard(checkers), BitBoard(pinners))
    }
    pub const fn checkmask_pinmask(
        &self,
        color: Color,
        checkers: BitBoard,
        mut pinners: BitBoard,
    ) -> (BitBoard, PinMask) {
        let our_king = self.colors(color).0 & self.pieces(PieceType::King).0;
        let mut sliders = checkers.0 & !KNIGHT_LOOKUP[our_king.trailing_zeros() as usize].0;
        let mut checkmask = 0;
        // dbg!(sliders);
        while sliders > 0 {
            checkmask |= between_rays(
                BitBoard(1u64 << sliders.trailing_zeros() as usize),
                BitBoard(our_king),
            )
            .0;
            // dbg!(checkmask);
            sliders &= sliders - 1;
        }
        checkmask &= !our_king | KNIGHT_LOOKUP[our_king.trailing_zeros() as usize].0;

        let mut pinmask = PinMask {
            h: BitBoard::EMPTY,
            v: BitBoard::EMPTY,
            d1: BitBoard::EMPTY,
            d2: BitBoard::EMPTY,
        };
        let king_rank = (our_king.trailing_zeros() >> 3) as usize;
        let king_file = (our_king.trailing_zeros() & 7) as usize;
        while pinners.0 > 0 {
            let rank = pinners.first() >> 3;
            let file = pinners.first() & 7;
            let pin = (between_rays(BitBoard(our_king), BitBoard(1 << pinners.first())).0
                | 1u64 << pinners.first())
                & !self.colors(color).0;
            if king_rank == rank {
                pinmask.h = BitBoard(pin)
            } else if king_file == file {
                pinmask.v = BitBoard(pin)
            } else if king_rank - rank == king_file - file {
                pinmask.d1 = BitBoard(pin)
            } else if king_rank - rank == file - king_file {
                pinmask.d2 = BitBoard(pin)
            } else {
                panic!("invalid pin")
            }
            pinners.0 &= pinners.0 - 1;
        }
        (BitBoard(checkmask), pinmask)
    }
    pub const fn king_moves(&self, color: Color) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];
        let king_idx = (self.colors(color).0 & self.pieces(PieceType::King).0).trailing_zeros();
        // let mut moves_bb;

        todo!()
    }
}
