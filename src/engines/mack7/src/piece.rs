#[derive(PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(PartialEq)]
pub enum CapturedPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    None,
}

#[derive(Debug)]
pub enum PromotionPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
}
