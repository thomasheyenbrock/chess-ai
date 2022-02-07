use crate::{
    bitboard::Bitboard,
    piece::{Piece, PromotionPiece},
};

#[derive(Debug)]
pub enum Castle {
    Kingside,
    Queenside,
}

pub struct Move {
    pub player: bool,
    pub piece: Piece,
    pub from_square: Bitboard,
    pub to_square: Bitboard,
    pub en_passant_square: Bitboard,
    pub is_capturing_en_passant: bool,
    pub is_castling: Option<Castle>,
    pub is_promoting_to: Option<PromotionPiece>,
}
