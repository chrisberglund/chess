/*
    This uses an integer to represent all the data in a move with
    the following bits representing each part:
    0000 0000 0000 0000 0000 0111 1111 -> Origin square 0x7F
    0000 0000 0000 0011 1111 1000 0000 -> To square >> 7 0x3F
    0000 0000 0011 1100 0000 0000 0000 -> Captured >> 14 0xF
    0000 0000 0100 0000 0000 0000 0000 -> En passant 0x40000
    0000 0000 1000 0000 0000 0000 0000 -> Pawn start 0x80000
    0000 1111 0000 0000 0000 0000 0000 -> Promoted piece >> 20 0xF
    0001 0000 0000 0000 0000 0000 0000 -> Castle 0x1000000
 */

use crate::constants::squares::{FILE_SQUARES,RANK_SQUARES};
use crate::constants::pieces::EMPTY;

pub const MFLAG_EP:u32 = 0x40000; // En passant
pub const MFLAG_PS:u32 = 0x80000; // Pawn start
pub const MFLAG_CA:u32 = 0x1000000; // Castle
pub const MFLAG_CAP:u32 = 0x7C000; // Capture
pub const MFLAG_PROM:u32 = 0xF00000; // Promotion

#[derive(Debug, Copy, Clone)]
pub struct GameMove {
    pub move_int: u32,
    pub score: u8,
}

impl GameMove {
    #[inline(always)]
    pub fn new(from:u8, to:u8, cap:u8,
               prom:u8, flag:u32) -> GameMove {
        let move_int:u32 = (from as u32) | (to as u32) << 7 | (cap as u32) << 14 | (prom as u32) << 20 | flag;
        GameMove {
            move_int,
            score: 0
        }
    }

    /// Returns the starting square for the move
    #[inline(always)]
    pub fn origin(self) -> u8 { (self.move_int & 0x7F) as u8 }

    /// Returns the final square for the move
    #[inline(always)]
    pub fn destination(self) -> u8 { (self.move_int >> 7 & 0x7F) as u8 }

    /// Returns the piece type of any captured piece
    #[inline(always)]
    pub fn capture(self) -> u8 { (self.move_int >> 14 & 0xF) as u8 }

    /// Returns the piece type a pawn was promoted to (if applicable)
    #[inline(always)]
    pub fn promoted_piece(self) -> u8 { (self.move_int >> 20 & 0xF) as u8 }

    /// Returns whether or not this was the first move of the game for the pawn (if applicable)
    #[inline(always)]
    pub fn is_pawn_start(&self) -> bool { (self.move_int & MFLAG_PS) > 0}

    /// Returns whether or not the move was an en passant move
    #[inline(always)]
    pub fn is_en_passant(&self) -> bool { (self.move_int & MFLAG_EP) > 0}

    /// Returns whether or not this was a castling move
    #[inline(always)]
    pub fn is_castle_move(&self) -> bool { (self.move_int & MFLAG_CA) > 0}
}

/// Prints the board
impl std::fmt::Display for GameMove {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        const FILES:[&str; 8] = ["A", "B", "C", "D","E", "F", "G", "H"];
        const RANKS:[&str; 8] = ["1", "2", "3", "4","5", "6", "7", "8"];

        let mut output = String::from("");

        let start_file = FILES[FILE_SQUARES[self.origin() as usize] as usize];
        let start_rank = RANKS[RANK_SQUARES[self.origin() as usize] as usize];

        let end_file = FILES[FILE_SQUARES[self.destination() as usize] as usize];
        let end_rank = RANKS[RANK_SQUARES[self.destination() as usize] as usize];

        let capture = self.capture() != EMPTY;

        let mut output = String::from("");


        output.push_str(start_file);
        output.push_str(start_rank);
        output.push_str(if capture {"x"} else {" "});
        output.push_str(end_file);
        output.push_str(end_rank);
        write!(f, "{}", output)
    }
}

impl PartialEq for GameMove {
    fn eq(&self, other: &Self) -> bool {
        self.move_int == other.move_int
    }
}