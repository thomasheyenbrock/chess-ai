// TODO: calculate pinned pieces once based on the king position instead of redoing it for every piece

use crate::bitboard::{Bitboard, Direction};
use rayon::prelude::*;

enum Result {
    White,
    Black,
    Stalemate,
    DeadPosition,
    Repitition,
    FiftyMoveRule,
}

fn get_moves_in_direction(
    all_pieces: Bitboard,
    enemy_pieces: Bitboard,
    square: Bitboard,
    direction: Direction,
) -> Bitboard {
    let mut moves = Bitboard::new(0);
    let mut running = square.get_square_in_direction(direction);

    while !running.is_empty() {
        if (all_pieces & running).is_empty() {
            moves |= running;
            running = running.get_square_in_direction(direction);
        } else if !(enemy_pieces & running).is_empty() {
            moves |= running;
            running = Bitboard::new(0);
        } else {
            running = Bitboard::new(0);
        }
    }

    moves
}

fn get_rank_and_file_moves(
    all_pieces: Bitboard,
    enemy_pieces: Bitboard,
    square: Bitboard,
) -> Bitboard {
    get_moves_in_direction(all_pieces, enemy_pieces, square, Direction::Top)
        | get_moves_in_direction(all_pieces, enemy_pieces, square, Direction::Bottom)
        | get_moves_in_direction(all_pieces, enemy_pieces, square, Direction::Left)
        | get_moves_in_direction(all_pieces, enemy_pieces, square, Direction::Right)
}

fn get_diagonal_moves(all_pieces: Bitboard, enemy_pieces: Bitboard, square: Bitboard) -> Bitboard {
    get_moves_in_direction(all_pieces, enemy_pieces, square, Direction::TopLeft)
        | get_moves_in_direction(all_pieces, enemy_pieces, square, Direction::TopRight)
        | get_moves_in_direction(all_pieces, enemy_pieces, square, Direction::BottomLeft)
        | get_moves_in_direction(all_pieces, enemy_pieces, square, Direction::BottomRight)
}

#[derive(PartialEq)]
enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(PartialEq)]
enum CapturedPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    None,
}

enum PromotionPiece {
    Queen,
    Rook,
    Bishop,
    Knight,
}

enum Castle {
    Kingside,
    Queenside,
}

struct Move {
    player: bool,
    piece: Piece,
    from_square: Bitboard,
    to_square: Bitboard,
    en_passant_square: Bitboard,
    is_capturing_en_passant: bool,
    is_castling: Option<Castle>,
    is_promoting_to: Option<PromotionPiece>,
}

#[derive(Clone, Copy)]
struct Pieces {
    all: Bitboard,
    king: Bitboard,
    queen: Bitboard,
    rook: Bitboard,
    bishop: Bitboard,
    knight: Bitboard,
    pawn: Bitboard,
}

#[derive(Clone, Copy)]
struct Position {
    all: Bitboard,
    white: Pieces,
    black: Pieces,
}

impl Position {
    fn make_move(self, m: &Move) -> (Position, CapturedPiece) {
        let mut next = self.clone();

        match m.is_castling {
            Some(Castle::Kingside) => {
                if m.player {
                    next.white.king = Bitboard::new(0x0000_0000_0000_0002);
                    next.white.rook ^= Bitboard::new(0x0000_0000_0000_0005);
                    next.white.all ^= Bitboard::new(0x0000_0000_0000_000F);
                    next.all ^= Bitboard::new(0x0000_0000_0000_000F);
                } else {
                    next.black.king = Bitboard::new(0x0200_0000_0000_0000);
                    next.black.rook ^= Bitboard::new(0x0500_0000_0000_0000);
                    next.black.all ^= Bitboard::new(0x0F00_0000_0000_0000);
                    next.all ^= Bitboard::new(0x0F00_0000_0000_0000);
                }
                return (next, CapturedPiece::None);
            }
            Some(Castle::Queenside) => {
                if m.player {
                    next.white.king = Bitboard::new(0x0000_0000_0000_0020);
                    next.white.rook ^= Bitboard::new(0x0000_0000_0000_0090);
                    next.white.all ^= Bitboard::new(0x0000_0000_0000_00B8);
                    next.all ^= Bitboard::new(0x0000_0000_0000_00B8);
                } else {
                    next.black.king = Bitboard::new(0x2000_0000_0000_0000);
                    next.black.rook ^= Bitboard::new(0x9000_0000_0000_0000);
                    next.black.all ^= Bitboard::new(0xB800_0000_0000_0000);
                    next.all ^= Bitboard::new(0xB800_0000_0000_0000);
                }
                return (next, CapturedPiece::None);
            }
            None => {}
        }

        let is_capturing = if !(m.to_square & next.white.pawn).is_empty() {
            CapturedPiece::Pawn
        } else if !(m.to_square & next.black.pawn).is_empty() {
            CapturedPiece::Pawn
        } else if !(m.to_square & next.white.knight).is_empty() {
            CapturedPiece::Knight
        } else if !(m.to_square & next.black.knight).is_empty() {
            CapturedPiece::Knight
        } else if !(m.to_square & next.white.bishop).is_empty() {
            CapturedPiece::Bishop
        } else if !(m.to_square & next.black.bishop).is_empty() {
            CapturedPiece::Bishop
        } else if !(m.to_square & next.white.rook).is_empty() {
            CapturedPiece::Rook
        } else if !(m.to_square & next.black.rook).is_empty() {
            CapturedPiece::Rook
        } else if !(m.to_square & next.white.queen).is_empty() {
            CapturedPiece::Queen
        } else if !(m.to_square & next.black.queen).is_empty() {
            CapturedPiece::Queen
        } else {
            CapturedPiece::None
        };

        match (m.player, &m.piece) {
            (true, Piece::King) => {
                next.white.king = (next.white.king ^ m.from_square) | m.to_square
            }
            (true, Piece::Queen) => {
                next.white.queen = (next.white.queen ^ m.from_square) | m.to_square
            }
            (true, Piece::Rook) => {
                next.white.rook = (next.white.rook ^ m.from_square) | m.to_square
            }
            (true, Piece::Bishop) => {
                next.white.bishop = (next.white.bishop ^ m.from_square) | m.to_square
            }
            (true, Piece::Knight) => {
                next.white.knight = (next.white.knight ^ m.from_square) | m.to_square
            }
            (true, Piece::Pawn) => {
                next.white.pawn = (next.white.pawn ^ m.from_square) | m.to_square
            }
            (false, Piece::King) => {
                next.black.king = (next.black.king ^ m.from_square) | m.to_square
            }
            (false, Piece::Queen) => {
                next.black.queen = (next.black.queen ^ m.from_square) | m.to_square
            }
            (false, Piece::Rook) => {
                next.black.rook = (next.black.rook ^ m.from_square) | m.to_square
            }
            (false, Piece::Bishop) => {
                next.black.bishop = (next.black.bishop ^ m.from_square) | m.to_square
            }
            (false, Piece::Knight) => {
                next.black.knight = (next.black.knight ^ m.from_square) | m.to_square
            }
            (false, Piece::Pawn) => {
                next.black.pawn = (next.black.pawn ^ m.from_square) | m.to_square
            }
        }

        if m.player {
            next.white.all = (next.white.all ^ m.from_square) | m.to_square;
        } else {
            next.black.all = (next.black.all ^ m.from_square) | m.to_square;
        }
        next.all = (next.all ^ m.from_square) | m.to_square;

        match (m.player, &is_capturing) {
            (true, CapturedPiece::Queen) => next.black.queen ^= m.to_square,
            (true, CapturedPiece::Rook) => next.black.rook ^= m.to_square,
            (true, CapturedPiece::Bishop) => next.black.bishop ^= m.to_square,
            (true, CapturedPiece::Knight) => next.black.knight ^= m.to_square,
            (true, CapturedPiece::Pawn) => next.black.pawn ^= m.to_square,
            (false, CapturedPiece::Queen) => next.white.queen ^= m.to_square,
            (false, CapturedPiece::Rook) => next.white.rook ^= m.to_square,
            (false, CapturedPiece::Bishop) => next.white.bishop ^= m.to_square,
            (false, CapturedPiece::Knight) => next.white.knight ^= m.to_square,
            (false, CapturedPiece::Pawn) => next.white.pawn ^= m.to_square,
            (_, CapturedPiece::None) => {}
        }

        match (m.player, &is_capturing) {
            (_, CapturedPiece::None) => {}
            (true, _) => next.black.all ^= m.to_square,
            (false, _) => next.white.all ^= m.to_square,
        }

        if m.is_capturing_en_passant {
            if m.player {
                let captured_square = m.to_square.get_bottom_square();
                next.black.pawn ^= captured_square;
                next.black.all ^= captured_square;
                next.all ^= captured_square;
            } else {
                let captured_square = m.to_square.get_top_square();
                next.white.pawn ^= captured_square;
                next.white.all ^= captured_square;
                next.all ^= captured_square;
            }
        }

        match &m.is_promoting_to {
            Some(promotion_piece) => {
                match (m.player, promotion_piece) {
                    (true, PromotionPiece::Queen) => next.white.queen |= m.to_square,
                    (true, PromotionPiece::Rook) => next.white.rook |= m.to_square,
                    (true, PromotionPiece::Bishop) => next.white.bishop |= m.to_square,
                    (true, PromotionPiece::Knight) => next.white.knight |= m.to_square,
                    (false, PromotionPiece::Queen) => next.black.queen |= m.to_square,
                    (false, PromotionPiece::Rook) => next.black.rook |= m.to_square,
                    (false, PromotionPiece::Bishop) => next.black.bishop |= m.to_square,
                    (false, PromotionPiece::Knight) => next.black.knight |= m.to_square,
                }
                if m.player {
                    next.white.pawn ^= m.to_square;
                } else {
                    next.black.pawn ^= m.to_square;
                }
            }
            None => {}
        }

        (next, is_capturing)
    }

    fn attackers_in_direction(
        self,
        square: Bitboard,
        pieces: Bitboard,
        direction: Direction,
    ) -> Bitboard {
        let mut attackers = Bitboard::new(0);
        let mut running = square.get_square_in_direction(direction);

        while !running.is_empty() {
            if !(pieces & running).is_empty() {
                attackers |= running;
                running = Bitboard::new(0);
            } else if (self.all & running).is_empty() {
                running = running.get_square_in_direction(direction);
            } else {
                running = Bitboard::new(0);
            }
        }

        attackers
    }

    fn attackers(self, player: bool, square: Bitboard) -> Bitboard {
        let forward_square = if player {
            square.get_bottom_square()
        } else {
            square.get_top_square()
        };

        let pieces = if player { self.white } else { self.black };

        let queen_and_rook = pieces.queen | pieces.rook;
        let queen_and_bishop = pieces.queen | pieces.bishop;

        let attackers = (square.king_moves() & pieces.king)
            | self.attackers_in_direction(square, queen_and_rook, Direction::Top)
            | self.attackers_in_direction(square, queen_and_rook, Direction::Bottom)
            | self.attackers_in_direction(square, queen_and_rook, Direction::Left)
            | self.attackers_in_direction(square, queen_and_rook, Direction::Right)
            | self.attackers_in_direction(square, queen_and_bishop, Direction::TopLeft)
            | self.attackers_in_direction(square, queen_and_bishop, Direction::TopRight)
            | self.attackers_in_direction(square, queen_and_bishop, Direction::BottomLeft)
            | self.attackers_in_direction(square, queen_and_bishop, Direction::BottomRight)
            | (square.knight_moves() & pieces.knight)
            | (forward_square.get_left_square() & pieces.pawn)
            | (forward_square.get_right_square() & pieces.pawn);

        attackers
    }

    fn attacked_squares_in_direction(
        self,
        square: Bitboard,
        all_pieces: Bitboard,
        direction: Direction,
    ) -> Bitboard {
        let mut attacked_squares = Bitboard::new(0);
        let mut running = square.get_square_in_direction(direction);

        while !running.is_empty() {
            attacked_squares |= running;
            running = if (all_pieces & running).is_empty() {
                running.get_square_in_direction(direction)
            } else {
                Bitboard::new(0)
            };
        }

        attacked_squares
    }

    fn attacked_squares(self, player: bool) -> Bitboard {
        let all_pieces = self.all
            ^ if player {
                self.black.king
            } else {
                self.white.king
            };

        let mut attacked = if player {
            self.white.king.king_moves()
        } else {
            self.black.king.king_moves()
        };

        let queen_pieces = if player {
            self.white.queen
        } else {
            self.black.queen
        };
        for queen in queen_pieces.into_iter() {
            attacked = attacked
                | self.attacked_squares_in_direction(queen, all_pieces, Direction::Top)
                | self.attacked_squares_in_direction(queen, all_pieces, Direction::Bottom)
                | self.attacked_squares_in_direction(queen, all_pieces, Direction::Left)
                | self.attacked_squares_in_direction(queen, all_pieces, Direction::Right)
                | self.attacked_squares_in_direction(queen, all_pieces, Direction::TopLeft)
                | self.attacked_squares_in_direction(queen, all_pieces, Direction::TopRight)
                | self.attacked_squares_in_direction(queen, all_pieces, Direction::BottomLeft)
                | self.attacked_squares_in_direction(queen, all_pieces, Direction::BottomRight);
        }

        let rook_pieces = if player {
            self.white.rook
        } else {
            self.black.rook
        };
        for rook in rook_pieces.into_iter() {
            attacked = attacked
                | self.attacked_squares_in_direction(rook, all_pieces, Direction::Top)
                | self.attacked_squares_in_direction(rook, all_pieces, Direction::Bottom)
                | self.attacked_squares_in_direction(rook, all_pieces, Direction::Left)
                | self.attacked_squares_in_direction(rook, all_pieces, Direction::Right);
        }

        let bishop_pieces = if player {
            self.white.bishop
        } else {
            self.black.bishop
        };
        for bishop in bishop_pieces.into_iter() {
            attacked = attacked
                | self.attacked_squares_in_direction(bishop, all_pieces, Direction::TopLeft)
                | self.attacked_squares_in_direction(bishop, all_pieces, Direction::TopRight)
                | self.attacked_squares_in_direction(bishop, all_pieces, Direction::BottomLeft)
                | self.attacked_squares_in_direction(bishop, all_pieces, Direction::BottomRight);
        }

        let knight_pieces = if player {
            self.white.knight
        } else {
            self.black.knight
        };
        for knight in knight_pieces.into_iter() {
            attacked |= knight.knight_moves();
        }

        let pawn_pieces = if player {
            self.white.pawn
        } else {
            self.black.pawn
        };
        for pawn in pawn_pieces.into_iter() {
            let forward_square = if player {
                pawn.get_top_square()
            } else {
                pawn.get_bottom_square()
            };
            attacked |= forward_square.get_left_square() | forward_square.get_right_square();
        }

        attacked
    }

    fn is_check(self, player: bool) -> bool {
        let king = if player {
            self.white.king
        } else {
            self.black.king
        };
        !self.attackers(!player, king).is_empty()
    }

    fn pinned_movement_in_direction(
        self,
        square: Bitboard,
        king: Bitboard,
        attackers: Bitboard,
        direction: Direction,
    ) -> Option<Bitboard> {
        let opposite_direction = match direction {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::TopLeft => Direction::BottomRight,
            Direction::TopRight => Direction::BottomLeft,
            Direction::BottomLeft => Direction::TopRight,
            Direction::BottomRight => Direction::TopLeft,
        };
        let mut moves = Bitboard::new(0);
        let mut running = square.get_square_in_direction(direction);
        let mut found_king = false;
        let mut found_attacker = false;

        while !running.is_empty() {
            if !(king & running).is_empty() {
                found_king = true;
                running = Bitboard::new(0);
            } else if !(attackers & running).is_empty() {
                found_attacker = true;
                moves |= running;
                running = Bitboard::new(0);
            } else if (self.all & running).is_empty() {
                moves |= running;
                running = running.get_square_in_direction(direction);
            } else {
                // First piece is neither an attacker nor the king
                return None;
            }
        }

        if !(found_king || found_attacker) {
            // No piece at all found in this direction
            return None;
        }

        running = square.get_square_in_direction(opposite_direction);
        while !running.is_empty() {
            if !(king & running).is_empty() && found_attacker {
                return Some(moves);
            } else if !(attackers & running).is_empty() && found_king {
                return Some(moves | running);
            } else if (self.all & running).is_empty() {
                moves |= running;
                running = running.get_square_in_direction(opposite_direction);
            } else {
                // First piece is neither an attacker nor the king
                return None;
            }
        }

        // No piece at all found in this direction
        None
    }

    fn pinned_movement(
        self,
        square: Bitboard,
        king: Bitboard,
        enemy_queens_and_rooks: Bitboard,
        enemy_queens_and_bishops: Bitboard,
    ) -> Bitboard {
        match self.pinned_movement_in_direction(
            square,
            king,
            enemy_queens_and_rooks,
            Direction::Top,
        ) {
            Some(moves) => return moves,
            None => {}
        }

        match self.pinned_movement_in_direction(
            square,
            king,
            enemy_queens_and_rooks,
            Direction::Left,
        ) {
            Some(moves) => return moves,
            None => {}
        }

        match self.pinned_movement_in_direction(
            square,
            king,
            enemy_queens_and_bishops,
            Direction::TopLeft,
        ) {
            Some(moves) => return moves,
            None => {}
        }

        match self.pinned_movement_in_direction(
            square,
            king,
            enemy_queens_and_bishops,
            Direction::TopRight,
        ) {
            Some(moves) => return moves,
            None => {}
        }

        Bitboard::new(0xFFFF_FFFF_FFFF_FFFF)
    }

    fn get_push_squares_in_direction(
        self,
        square: Bitboard,
        attackers: Bitboard,
        direction: Direction,
    ) -> Bitboard {
        let mut moves = Bitboard::new(0);
        let mut running = square.get_square_in_direction(direction);

        while !running.is_empty() {
            if !(attackers & running).is_empty() {
                return moves;
            } else {
                moves |= running;
                running = running.get_square_in_direction(direction);
            }
        }

        Bitboard::new(0)
    }

    fn is_dead(self) -> bool {
        let white_queens = self.white.queen.count_ones();
        if white_queens > 0 {
            return false;
        }

        let black_queens = self.black.queen.count_ones();
        if black_queens > 0 {
            return false;
        }

        let white_rooks = self.white.rook.count_ones();
        if white_rooks > 0 {
            return false;
        }

        let black_rooks = self.black.rook.count_ones();
        if black_rooks > 0 {
            return false;
        }

        let white_pawns = self.white.pawn.count_ones();
        if white_pawns > 0 {
            return false;
        }

        let black_pawns = self.black.pawn.count_ones();
        if black_pawns > 0 {
            return false;
        }

        let white_bishops = self.white.bishop.count_ones();
        if white_bishops > 1 {
            return false;
        }

        let black_bishops = self.black.bishop.count_ones();
        if black_bishops > 1 {
            return false;
        }

        let white_knights = self.white.knight.count_ones();
        if white_knights > 1 {
            return false;
        }

        let black_knights = self.black.knight.count_ones();
        if black_knights > 1 {
            return false;
        }

        let number_of_white_pieces =
            white_queens + white_rooks + white_bishops + white_knights + white_pawns;
        let number_of_black_pieces =
            black_queens + black_rooks + black_bishops + black_knights + black_pawns;

        // king against king
        if number_of_white_pieces + number_of_black_pieces == 0 {
            return true;
        }

        // king against king and bishop
        if number_of_white_pieces == 0 && number_of_black_pieces == 1 && black_bishops == 1 {
            return true;
        }
        if number_of_black_pieces == 0 && number_of_white_pieces == 1 && white_bishops == 1 {
            return true;
        }

        // king against king and knight
        if number_of_white_pieces == 0 && number_of_black_pieces == 1 && black_knights == 1 {
            return true;
        }
        if number_of_black_pieces == 0 && number_of_white_pieces == 1 && white_knights == 1 {
            return true;
        }

        // king and bishop against king and bishop, with both bishops on squares of the same color
        if number_of_white_pieces == 1
            && number_of_black_pieces == 1
            && white_bishops == 1
            && black_bishops == 1
        {
            let is_white_bishop_on_white_square =
                (self.white.bishop & Bitboard::new(0xAA55_AA55_AA55_AA55)).is_empty();
            let is_black_bishop_on_white_square =
                (self.black.bishop & Bitboard::new(0xAA55_AA55_AA55_AA55)).is_empty();
            return is_white_bishop_on_white_square == is_black_bishop_on_white_square;
        }

        false
    }
}

struct PossibleCastles {
    white_kingside: bool,
    white_queenside: bool,
    black_kingside: bool,
    black_queenside: bool,
}

pub struct Game {
    position: Position,
    player: bool,
    // last_move: Move,
    possible_castles: PossibleCastles,
    en_passant_square: Bitboard,
    position_counts: Vec<String>,
    move_counter: i32,
    fifty_move_counter: i32,
}

impl Game {
    fn id(&self) -> String {
        format!(
            "{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}-{}",
            self.position.white.king.str(),
            self.position.white.queen.str(),
            self.position.white.rook.str(),
            self.position.white.bishop.str(),
            self.position.white.knight.str(),
            self.position.white.pawn.str(),
            self.position.black.king.str(),
            self.position.black.queen.str(),
            self.position.black.rook.str(),
            self.position.black.bishop.str(),
            self.position.black.knight.str(),
            self.position.black.pawn.str(),
            self.player.to_string(),
            if self.possible_castles.white_kingside {
                "K"
            } else {
                ""
            },
            if self.possible_castles.white_queenside {
                "Q"
            } else {
                ""
            },
            if self.possible_castles.black_kingside {
                "k"
            } else {
                ""
            },
            if self.possible_castles.black_queenside {
                "q"
            } else {
                ""
            },
            self.en_passant_square.str()
        )
    }

    fn make_move(&self, m: &Move) -> Game {
        let (new_position, is_capturing) = self.position.make_move(&m);

        let possible_castles = PossibleCastles {
            white_kingside: self.possible_castles.white_kingside
                && !(self.player && m.piece == Piece::King)
                && !(self.player
                    && m.piece == Piece::Rook
                    && m.from_square == Bitboard::new(0x0000_0000_0000_0001))
                && !(!self.player
                    && is_capturing == CapturedPiece::Rook
                    && m.to_square == Bitboard::new(0x0000_0000_0000_0001)),
            white_queenside: self.possible_castles.white_queenside
                && !(self.player && m.piece == Piece::King)
                && !(self.player
                    && m.piece == Piece::Rook
                    && m.from_square == Bitboard::new(0x0000_0000_0000_0080))
                && !(!self.player
                    && is_capturing == CapturedPiece::Rook
                    && m.to_square == Bitboard::new(0x0000_0000_0000_0080)),
            black_kingside: self.possible_castles.black_kingside
                && !(!self.player && m.piece == Piece::King)
                && !(!self.player
                    && m.piece == Piece::Rook
                    && m.from_square == Bitboard::new(0x0100_0000_0000_0000))
                && !(self.player
                    && is_capturing == CapturedPiece::Rook
                    && m.to_square == Bitboard::new(0x0100_0000_0000_0000)),
            black_queenside: self.possible_castles.black_queenside
                && !(!self.player && m.piece == Piece::King)
                && !(!self.player
                    && m.piece == Piece::Rook
                    && m.from_square == Bitboard::new(0x8000_0000_0000_0000))
                && !(self.player
                    && is_capturing == CapturedPiece::Rook
                    && m.to_square == Bitboard::new(0x8000_0000_0000_0000)),
        };

        let player = !self.player;
        let move_counter = self.move_counter + (if self.player { 0 } else { 1 });
        let en_passant_square = m.en_passant_square;
        let fifty_move_counter = if m.piece == Piece::Pawn
            || is_capturing != CapturedPiece::None
            || m.is_capturing_en_passant
        {
            0
        } else {
            self.fifty_move_counter + 1
        };

        let mut position_counts: Vec<String>;
        if !(is_capturing != CapturedPiece::None
            || m.is_promoting_to.is_some()
            || m.is_castling.is_some())
        {
            position_counts = vec![];
        } else {
            position_counts = self.position_counts.clone();
            position_counts.push(self.id());
        }

        Game {
            position: new_position,
            player,
            // last_move: m,
            possible_castles,
            en_passant_square,
            position_counts,
            move_counter,
            fifty_move_counter,
        }
    }

    fn legal_pawn_attack_moves(&self, from_square: Bitboard, to_square: Bitboard) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];

        let promotion_squares = if self.player {
            Bitboard::new(0xFF00_0000_0000_0000)
        } else {
            Bitboard::new(0x0000_0000_0000_00FF)
        };
        if !(to_square & promotion_squares).is_empty() {
            result.push(Move {
                player: self.player,
                piece: Piece::Pawn,
                from_square,
                to_square,
                en_passant_square: Bitboard::new(0),
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: Some(PromotionPiece::Queen),
            });
            result.push(Move {
                player: self.player,
                piece: Piece::Pawn,
                from_square,
                to_square,
                en_passant_square: Bitboard::new(0),
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: Some(PromotionPiece::Rook),
            });
            result.push(Move {
                player: self.player,
                piece: Piece::Pawn,
                from_square,
                to_square,
                en_passant_square: Bitboard::new(0),
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: Some(PromotionPiece::Bishop),
            });
            result.push(Move {
                player: self.player,
                piece: Piece::Pawn,
                from_square,
                to_square,
                en_passant_square: Bitboard::new(0),
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: Some(PromotionPiece::Knight),
            });
        } else {
            result.push(Move {
                player: self.player,
                piece: Piece::Pawn,
                from_square,
                to_square,
                en_passant_square: Bitboard::new(0),
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: None,
            });
        };

        result
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];

        let friendly_pieces = if self.player {
            self.position.white.all
        } else {
            self.position.black.all
        };
        let enemy_pieces = if self.player {
            self.position.black.all
        } else {
            self.position.white.all
        };
        let empty_squares = Bitboard::new(0xFFFF_FFFF_FFFF_FFFF) ^ self.position.all;
        let attacked_squares = self.position.attacked_squares(!self.player);

        let king = if self.player {
            self.position.white.king
        } else {
            self.position.black.king
        };
        let mut king_moves =
            king.king_moves() & (Bitboard::new(0xFFFF_FFFF_FFFF_FFFF) ^ attacked_squares);
        king_moves = king_moves ^ (king_moves & friendly_pieces);
        for to_square in king_moves.into_iter() {
            result.push(Move {
                player: self.player,
                piece: Piece::King,
                from_square: king,
                to_square,
                en_passant_square: Bitboard::new(0),
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: None,
            })
        }

        let attackers = self.position.attackers(!self.player, king);

        let number_of_attackers = attackers.count_ones();
        if number_of_attackers > 1 {
            // Multiple pieces are giving check, so the king has to move
            return result;
        }

        let mut capture_mask = Bitboard::new(0xFFFF_FFFF_FFFF_FFFF);
        let mut push_mask = Bitboard::new(0xFFFF_FFFF_FFFF_FFFF);
        if number_of_attackers == 1 {
            capture_mask = attackers;

            let knight = if self.player {
                self.position.black.knight
            } else {
                self.position.white.knight
            };
            let pawn = if self.player {
                self.position.black.pawn
            } else {
                self.position.white.pawn
            };
            if (!(attackers & knight).is_empty()) || (!(attackers & pawn).is_empty()) {
                // checked by knight or pawn, this can't be blocked
                push_mask = Bitboard::new(0);
            } else {
                push_mask =
                    self.position
                        .get_push_squares_in_direction(king, attackers, Direction::Top)
                        | self.position.get_push_squares_in_direction(
                            king,
                            attackers,
                            Direction::Bottom,
                        )
                        | self.position.get_push_squares_in_direction(
                            king,
                            attackers,
                            Direction::Left,
                        )
                        | self.position.get_push_squares_in_direction(
                            king,
                            attackers,
                            Direction::Right,
                        )
                        | self.position.get_push_squares_in_direction(
                            king,
                            attackers,
                            Direction::TopLeft,
                        )
                        | self.position.get_push_squares_in_direction(
                            king,
                            attackers,
                            Direction::TopRight,
                        )
                        | self.position.get_push_squares_in_direction(
                            king,
                            attackers,
                            Direction::BottomLeft,
                        )
                        | self.position.get_push_squares_in_direction(
                            king,
                            attackers,
                            Direction::BottomRight,
                        )
            }
        }

        let capture_or_push_mask = capture_mask | push_mask;

        let enemy_queens = if self.player {
            self.position.black.queen
        } else {
            self.position.white.queen
        };
        let enemy_queens_and_rooks = enemy_queens
            | (if self.player {
                self.position.black.rook
            } else {
                self.position.white.rook
            });
        let enemy_queens_and_bishops = enemy_queens
            | (if self.player {
                self.position.black.bishop
            } else {
                self.position.white.bishop
            });

        let queen = if self.player {
            self.position.white.queen
        } else {
            self.position.black.queen
        };
        for from_square in queen.into_iter() {
            let moveable_squares = capture_or_push_mask
                & (get_rank_and_file_moves(self.position.all, enemy_pieces, from_square)
                    | get_diagonal_moves(self.position.all, enemy_pieces, from_square))
                & self.position.pinned_movement(
                    from_square,
                    king,
                    enemy_queens_and_rooks,
                    enemy_queens_and_bishops,
                );
            for to_square in moveable_squares.into_iter() {
                result.push(Move {
                    player: self.player,
                    piece: Piece::Queen,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::new(0),
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }
        }

        let rook = if self.player {
            self.position.white.rook
        } else {
            self.position.black.rook
        };
        for from_square in rook.into_iter() {
            let moveable_squares = capture_or_push_mask
                & get_rank_and_file_moves(self.position.all, enemy_pieces, from_square)
                & self.position.pinned_movement(
                    from_square,
                    king,
                    enemy_queens_and_rooks,
                    enemy_queens_and_bishops,
                );
            for to_square in moveable_squares.into_iter() {
                result.push(Move {
                    player: self.player,
                    piece: Piece::Rook,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::new(0),
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }
        }

        let bishop = if self.player {
            self.position.white.bishop
        } else {
            self.position.black.bishop
        };
        for from_square in bishop.into_iter() {
            let moveable_squares = capture_or_push_mask
                & get_diagonal_moves(self.position.all, enemy_pieces, from_square)
                & self.position.pinned_movement(
                    from_square,
                    king,
                    enemy_queens_and_rooks,
                    enemy_queens_and_bishops,
                );
            for to_square in moveable_squares.into_iter() {
                result.push(Move {
                    player: self.player,
                    piece: Piece::Bishop,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::new(0),
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }
        }

        let knight = if self.player {
            self.position.white.knight
        } else {
            self.position.black.knight
        };
        for from_square in knight.into_iter() {
            let knight_moves = from_square.knight_moves();
            let moveable_squares = capture_or_push_mask
                & knight_moves
                & (knight_moves ^ friendly_pieces)
                & self.position.pinned_movement(
                    from_square,
                    king,
                    enemy_queens_and_rooks,
                    enemy_queens_and_bishops,
                );
            for to_square in moveable_squares.into_iter() {
                result.push(Move {
                    player: self.player,
                    piece: Piece::Knight,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::new(0),
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }
        }

        let pawn = if self.player {
            self.position.white.pawn
        } else {
            self.position.black.pawn
        };
        for from_square in pawn.into_iter() {
            let mut to_square: Bitboard;

            let pinned_movement = self.position.pinned_movement(
                from_square,
                king,
                enemy_queens_and_rooks,
                enemy_queens_and_bishops,
            );

            let forward_square = if self.player {
                from_square.get_top_square()
            } else {
                from_square.get_bottom_square()
            };

            // Pawn single moves
            to_square = forward_square & empty_squares & pinned_movement & push_mask;
            if !to_square.is_empty() {
                let promotion_squares = if self.player {
                    Bitboard::new(0xFF00_0000_0000_0000)
                } else {
                    Bitboard::new(0x0000_0000_0000_00FF)
                };
                if !(to_square & promotion_squares).is_empty() {
                    result.push(Move {
                        player: self.player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::new(0),
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: Some(PromotionPiece::Queen),
                    });
                    result.push(Move {
                        player: self.player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::new(0),
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: Some(PromotionPiece::Rook),
                    });
                    result.push(Move {
                        player: self.player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::new(0),
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: Some(PromotionPiece::Bishop),
                    });
                    result.push(Move {
                        player: self.player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::new(0),
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: Some(PromotionPiece::Knight),
                    });
                } else {
                    result.push(Move {
                        player: self.player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::new(0),
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: None,
                    })
                }
            }

            // Pawn attacks
            let to_square_left =
                forward_square.get_left_square() & enemy_pieces & pinned_movement & capture_mask;
            if !to_square_left.is_empty() {
                result.extend(self.legal_pawn_attack_moves(from_square, to_square_left));
            }
            let to_square_right =
                forward_square.get_right_square() & enemy_pieces & pinned_movement & capture_mask;
            if !to_square_right.is_empty() {
                result.extend(self.legal_pawn_attack_moves(from_square, to_square_right));
            }

            // Pawn double moves
            let double_forward_square = if self.player {
                (forward_square & Bitboard::new(0x0000_0000_00FF_0000)).get_top_square()
            } else {
                (forward_square & Bitboard::new(0x0000_FF00_0000_0000)).get_bottom_square()
            };
            to_square = double_forward_square
                & empty_squares
                & (if self.player {
                    empty_squares.get_top_square()
                } else {
                    empty_squares.get_bottom_square()
                })
                & pinned_movement
                & push_mask;
            if !to_square.is_empty() {
                result.push(Move {
                    player: self.player,
                    piece: Piece::Pawn,
                    from_square,
                    to_square,
                    en_passant_square: (if self.player {
                        to_square.get_bottom_square()
                    } else {
                        to_square.get_top_square()
                    }),
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }

            let forward_square = if self.player {
                from_square.get_top_square()
            } else {
                from_square.get_bottom_square()
            };
            let en_passant_captures =
                forward_square.get_left_square() | forward_square.get_right_square();
            to_square = en_passant_captures
                & self.en_passant_square
                & pinned_movement
                & (if self.player {
                    capture_mask.get_top_square()
                } else {
                    capture_mask.get_bottom_square()
                });
            if !to_square.is_empty() {
                let m = Move {
                    player: self.player,
                    piece: Piece::Pawn,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::new(0),
                    is_capturing_en_passant: true,
                    is_castling: None,
                    is_promoting_to: None,
                };
                let position = self.position.make_move(&m).0;
                if !position.is_check(self.player) {
                    result.push(m);
                }
            }
        }

        let kingside_castle = if self.player {
            self.possible_castles.white_kingside
        } else {
            self.possible_castles.black_kingside
        };
        let kingside_squares_between = self.position.all
            & (if self.player {
                Bitboard::new(0x0000_0000_0000_0006)
            } else {
                Bitboard::new(0x0600_0000_0000_0000)
            });
        let kingside_attacks = attacked_squares
            & (if self.player {
                Bitboard::new(0x0000_0000_0000_000E)
            } else {
                Bitboard::new(0x0E00_0000_0000_0000)
            });
        let can_castle_kingside =
            kingside_castle && kingside_squares_between.is_empty() && kingside_attacks.is_empty();

        if can_castle_kingside {
            result.push(Move {
                player: self.player,
                piece: Piece::King,
                from_square: if self.player {
                    Bitboard::new(0x0000_0000_0000_0008)
                } else {
                    Bitboard::new(0x0800_0000_0000_0000)
                },
                to_square: if self.player {
                    Bitboard::new(0x0000_0000_0000_0002)
                } else {
                    Bitboard::new(0x0200_0000_0000_0000)
                },
                en_passant_square: Bitboard::new(0),
                is_capturing_en_passant: false,
                is_castling: Some(Castle::Kingside),
                is_promoting_to: None,
            })
        }

        let queenside_castle = if self.player {
            self.possible_castles.white_queenside
        } else {
            self.possible_castles.black_queenside
        };
        let queenside_squares_between = self.position.all
            & (if self.player {
                Bitboard::new(0x0000_0000_0000_0070)
            } else {
                Bitboard::new(0x7000_0000_0000_0000)
            });
        let queenside_attacks = attacked_squares
            & (if self.player {
                Bitboard::new(0x0000_0000_0000_0038)
            } else {
                Bitboard::new(0x3800_0000_0000_0000)
            });
        let can_castle_queenside = queenside_castle
            && queenside_squares_between.is_empty()
            && queenside_attacks.is_empty();

        if can_castle_queenside {
            result.push(Move {
                player: self.player,
                piece: Piece::King,
                from_square: if self.player {
                    Bitboard::new(0x0000_0000_0000_0008)
                } else {
                    Bitboard::new(0x0800_0000_0000_0000)
                },
                to_square: if self.player {
                    Bitboard::new(0x0000_0000_0000_0020)
                } else {
                    Bitboard::new(0x2000_0000_0000_0000)
                },
                en_passant_square: Bitboard::new(0),
                is_capturing_en_passant: false,
                is_castling: Some(Castle::Queenside),
                is_promoting_to: None,
            })
        }

        result
    }

    pub fn count_legal_moves(&self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        self.legal_moves()
            .par_iter()
            .map(|m| self.make_move(m).count_legal_moves(depth - 1))
            .sum()
    }

    fn result(&mut self, legal_moves: u64) -> Option<Result> {
        if legal_moves == 0 {
            if self.position.is_check(self.player) {
                return if self.player {
                    Some(Result::Black)
                } else {
                    Some(Result::White)
                };
            }
            return Some(Result::Stalemate);
        }

        if self.fifty_move_counter >= 100 {
            return Some(Result::FiftyMoveRule);
        }

        self.position_counts.sort();
        let mut count = 1;
        let mut prev = "";
        for id in self.position_counts.iter() {
            if prev == id {
                count += 1;
            } else {
                count = 1;
                prev = id;
            }
            if count >= 3 {
                return Some(Result::Repitition);
            }
        }

        if self.position.is_dead() {
            return Some(Result::DeadPosition);
        }

        return None;
    }
}

pub fn game_from_fen(fen: &str) -> Game {
    let fen_parts: Vec<&str> = fen.split(" ").collect();
    let mut position = Position {
        all: Bitboard::new(0),
        white: Pieces {
            all: Bitboard::new(0),
            king: Bitboard::new(0),
            queen: Bitboard::new(0),
            rook: Bitboard::new(0),
            bishop: Bitboard::new(0),
            knight: Bitboard::new(0),
            pawn: Bitboard::new(0),
        },
        black: Pieces {
            all: Bitboard::new(0),
            king: Bitboard::new(0),
            queen: Bitboard::new(0),
            rook: Bitboard::new(0),
            bishop: Bitboard::new(0),
            knight: Bitboard::new(0),
            pawn: Bitboard::new(0),
        },
    };

    for (rank_index, rank) in fen_parts[0].split("/").into_iter().enumerate() {
        let mut file_index = 0;
        for piece in rank.chars() {
            match piece.to_digit(10) {
                Some(digit) => file_index += digit as usize,
                None => {
                    let square =
                        Bitboard::new(1 << (63 - rank_index as u32 * 8 - file_index as u32));
                    match piece {
                        'K' => position.white.king |= square,
                        'Q' => position.white.queen |= square,
                        'R' => position.white.rook |= square,
                        'B' => position.white.bishop |= square,
                        'N' => position.white.knight |= square,
                        'P' => position.white.pawn |= square,
                        'k' => position.black.king |= square,
                        'q' => position.black.queen |= square,
                        'r' => position.black.rook |= square,
                        'b' => position.black.bishop |= square,
                        'n' => position.black.knight |= square,
                        'p' => position.black.pawn |= square,
                        _ => panic!("bad fen"),
                    }
                    match piece {
                        'K' | 'Q' | 'R' | 'B' | 'N' | 'P' => position.white.all |= square,
                        'k' | 'q' | 'r' | 'b' | 'n' | 'p' => position.black.all |= square,
                        _ => panic!("bad fen"),
                    }
                    position.all |= square;
                    file_index += 1;
                }
            }
        }
    }

    let en_passant_square = match fen_parts[3] {
        "a8" => Bitboard::new(0x8000_0000_0000_0000),
        "b8" => Bitboard::new(0x4000_0000_0000_0000),
        "c8" => Bitboard::new(0x2000_0000_0000_0000),
        "d8" => Bitboard::new(0x1000_0000_0000_0000),
        "e8" => Bitboard::new(0x0800_0000_0000_0000),
        "f8" => Bitboard::new(0x0400_0000_0000_0000),
        "g8" => Bitboard::new(0x0200_0000_0000_0000),
        "h8" => Bitboard::new(0x0100_0000_0000_0000),
        "a7" => Bitboard::new(0x0080_0000_0000_0000),
        "b7" => Bitboard::new(0x0040_0000_0000_0000),
        "c7" => Bitboard::new(0x0020_0000_0000_0000),
        "d7" => Bitboard::new(0x0010_0000_0000_0000),
        "e7" => Bitboard::new(0x0008_0000_0000_0000),
        "f7" => Bitboard::new(0x0004_0000_0000_0000),
        "g7" => Bitboard::new(0x0002_0000_0000_0000),
        "h7" => Bitboard::new(0x0001_0000_0000_0000),
        "a6" => Bitboard::new(0x0000_8000_0000_0000),
        "b6" => Bitboard::new(0x0000_4000_0000_0000),
        "c6" => Bitboard::new(0x0000_2000_0000_0000),
        "d6" => Bitboard::new(0x0000_1000_0000_0000),
        "e6" => Bitboard::new(0x0000_0800_0000_0000),
        "f6" => Bitboard::new(0x0000_0400_0000_0000),
        "g6" => Bitboard::new(0x0000_0200_0000_0000),
        "h6" => Bitboard::new(0x0000_0100_0000_0000),
        "a5" => Bitboard::new(0x0000_0080_0000_0000),
        "b5" => Bitboard::new(0x0000_0040_0000_0000),
        "c5" => Bitboard::new(0x0000_0020_0000_0000),
        "d5" => Bitboard::new(0x0000_0010_0000_0000),
        "e5" => Bitboard::new(0x0000_0008_0000_0000),
        "f5" => Bitboard::new(0x0000_0004_0000_0000),
        "g5" => Bitboard::new(0x0000_0002_0000_0000),
        "h5" => Bitboard::new(0x0000_0001_0000_0000),
        "a4" => Bitboard::new(0x0000_0000_8000_0000),
        "b4" => Bitboard::new(0x0000_0000_4000_0000),
        "c4" => Bitboard::new(0x0000_0000_2000_0000),
        "d4" => Bitboard::new(0x0000_0000_1000_0000),
        "e4" => Bitboard::new(0x0000_0000_0800_0000),
        "f4" => Bitboard::new(0x0000_0000_0400_0000),
        "g4" => Bitboard::new(0x0000_0000_0200_0000),
        "h4" => Bitboard::new(0x0000_0000_0100_0000),
        "a3" => Bitboard::new(0x0000_0000_0080_0000),
        "b3" => Bitboard::new(0x0000_0000_0040_0000),
        "c3" => Bitboard::new(0x0000_0000_0020_0000),
        "d3" => Bitboard::new(0x0000_0000_0010_0000),
        "e3" => Bitboard::new(0x0000_0000_0008_0000),
        "f3" => Bitboard::new(0x0000_0000_0004_0000),
        "g3" => Bitboard::new(0x0000_0000_0002_0000),
        "h3" => Bitboard::new(0x0000_0000_0001_0000),
        "a2" => Bitboard::new(0x0000_0000_0000_8000),
        "b2" => Bitboard::new(0x0000_0000_0000_4000),
        "c2" => Bitboard::new(0x0000_0000_0000_2000),
        "d2" => Bitboard::new(0x0000_0000_0000_1000),
        "e2" => Bitboard::new(0x0000_0000_0000_0800),
        "f2" => Bitboard::new(0x0000_0000_0000_0400),
        "g2" => Bitboard::new(0x0000_0000_0000_0200),
        "h2" => Bitboard::new(0x0000_0000_0000_0100),
        "a1" => Bitboard::new(0x0000_0000_0000_0080),
        "b1" => Bitboard::new(0x0000_0000_0000_0040),
        "c1" => Bitboard::new(0x0000_0000_0000_0020),
        "d1" => Bitboard::new(0x0000_0000_0000_0010),
        "e1" => Bitboard::new(0x0000_0000_0000_0008),
        "f1" => Bitboard::new(0x0000_0000_0000_0004),
        "g1" => Bitboard::new(0x0000_0000_0000_0002),
        "h1" => Bitboard::new(0x0000_0000_0000_0001),
        _ => Bitboard::new(0x0000_0000_0000_0000),
    };

    return Game {
        position,
        player: fen_parts[1] == "w",
        // last_move: Move {
        //     player: false,
        //     piece: Piece::King,
        //     from_square: Bitboard::new(0),
        //     to_square: Bitboard::new(0),
        //     en_passant_square: Bitboard::new(0),
        //     is_capturing_en_passant: false,
        //     is_castling: None,
        //     is_promoting_to: None,
        // },
        possible_castles: PossibleCastles {
            white_kingside: fen_parts[2].contains("K"),
            white_queenside: fen_parts[2].contains("Q"),
            black_kingside: fen_parts[2].contains("k"),
            black_queenside: fen_parts[2].contains("q"),
        },
        en_passant_square,
        position_counts: vec![],
        move_counter: fen_parts[5]
            .chars()
            .next()
            .unwrap_or('0')
            .to_digit(10)
            .unwrap_or(0) as i32,
        fifty_move_counter: fen_parts[4]
            .chars()
            .next()
            .unwrap_or('0')
            .to_digit(10)
            .unwrap_or(0) as i32,
    };
}

// proc equals*(game1: Game, game2: Game): bool =
//     return game1.player == game2.player and
//         game1.fifty_move_counter == game2.fifty_move_counter and
//         game1.move_counter == game2.move_counter and
//         game1.en_passant_square == game2.en_passant_square and
//         game1.possible_castles == game2.possible_castles and
//         game1.position == game2.position

#[cfg(test)]
mod lexer {
    use super::*;

    #[test]
    fn test_position_1() {
        let cases = [(1, 20), (2, 400), (3, 8902), (4, 197281), (5, 4865609)];
        for (depth, moves) in cases {
            let game = game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            assert_eq!(game.count_legal_moves(depth,), moves);
        }
    }

    #[test]
    fn test_position_2() {
        let cases = [(1, 48), (2, 2039), (3, 97862), (4, 4085603)];
        for (depth, moves) in cases {
            let game = game_from_fen(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            );
            assert_eq!(game.count_legal_moves(depth,), moves)
        }
    }

    #[test]
    fn test_position_3() {
        let cases = [(1, 14), (2, 191), (3, 2812), (4, 43238), (5, 674624)];
        for (depth, moves) in cases {
            let game = game_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
            assert_eq!(game.count_legal_moves(depth,), moves)
        }
    }

    #[test]
    fn test_position_4() {
        let cases = [(1, 6), (2, 264), (3, 9467), (4, 422333)];
        for (depth, moves) in cases {
            let game =
                game_from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
            assert_eq!(game.count_legal_moves(depth,), moves)
        }
    }

    #[test]
    fn test_position_5() {
        let cases = [(1, 44), (2, 1486), (3, 62379), (4, 2103487)];
        for (depth, moves) in cases {
            let game = game_from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
            assert_eq!(game.count_legal_moves(depth,), moves)
        }
    }

    #[test]
    fn test_position_6() {
        let cases = [(1, 46), (2, 2079), (3, 89890), (4, 3894594)];
        for (depth, moves) in cases {
            let game = game_from_fen(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            );
            assert_eq!(game.count_legal_moves(depth,), moves)
        }
    }
}
