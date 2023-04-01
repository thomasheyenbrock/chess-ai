// TODO: calculate pinned pieces once based on the king position instead of redoing it for every piece

use rayon::prelude::*;
use std::cmp::Ordering;

use crate::{
    bitboard::Bitboard,
    chess_move::{Castle, Move, MoveIndex},
    direction::Direction,
    piece::{CapturedPiece, Piece, PromotionPiece},
    position::{Pieces, Position},
};

pub enum GameResult {
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
    let mut moves = Bitboard::EMPTY;
    let mut running = square.get_square_in_direction(direction);

    while !running.is_empty() {
        if (all_pieces & running).is_empty() {
            moves |= running;
            running = running.get_square_in_direction(direction);
        } else if !(enemy_pieces & running).is_empty() {
            moves |= running;
            running = Bitboard::EMPTY;
        } else {
            running = Bitboard::EMPTY;
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

#[derive(Copy, Clone, Debug)]
pub struct PossibleCastles {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}

#[derive(Clone, Copy, Debug)]
struct PositionWithMeta {
    white_king: Bitboard,
    white_queen: Bitboard,
    white_rook: Bitboard,
    white_bishop: Bitboard,
    white_knight: Bitboard,
    white_pawn: Bitboard,
    black_king: Bitboard,
    black_queen: Bitboard,
    black_rook: Bitboard,
    black_bishop: Bitboard,
    black_knight: Bitboard,
    black_pawn: Bitboard,
    player: bool,
    castle_white_kingside: bool,
    castle_white_queenside: bool,
    castle_black_kingside: bool,
    castle_black_queenside: bool,
    en_passant_square: Bitboard,
}

impl Ord for PositionWithMeta {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut o: Ordering;

        o = self.player.cmp(&other.player);
        if o != Ordering::Equal {
            return o;
        }

        o = self.white_king.cmp(&other.white_king);
        if o != Ordering::Equal {
            return o;
        }

        o = self.black_king.cmp(&other.black_king);
        if o != Ordering::Equal {
            return o;
        }

        o = self.white_queen.cmp(&other.white_queen);
        if o != Ordering::Equal {
            return o;
        }

        o = self.black_queen.cmp(&other.black_queen);
        if o != Ordering::Equal {
            return o;
        }

        o = self.white_rook.cmp(&other.white_rook);
        if o != Ordering::Equal {
            return o;
        }

        o = self.black_rook.cmp(&other.black_rook);
        if o != Ordering::Equal {
            return o;
        }

        o = self.white_bishop.cmp(&other.white_bishop);
        if o != Ordering::Equal {
            return o;
        }

        o = self.black_bishop.cmp(&other.black_bishop);
        if o != Ordering::Equal {
            return o;
        }

        o = self.white_knight.cmp(&other.white_knight);
        if o != Ordering::Equal {
            return o;
        }

        o = self.black_knight.cmp(&other.black_knight);
        if o != Ordering::Equal {
            return o;
        }

        o = self.white_pawn.cmp(&other.white_pawn);
        if o != Ordering::Equal {
            return o;
        }

        o = self.black_pawn.cmp(&other.black_pawn);
        if o != Ordering::Equal {
            return o;
        }

        o = self.castle_white_kingside.cmp(&other.castle_white_kingside);
        if o != Ordering::Equal {
            return o;
        }

        o = self
            .castle_white_queenside
            .cmp(&other.castle_white_queenside);
        if o != Ordering::Equal {
            return o;
        }

        o = self.castle_black_kingside.cmp(&other.castle_black_kingside);
        if o != Ordering::Equal {
            return o;
        }

        o = self
            .castle_black_queenside
            .cmp(&other.castle_black_queenside);
        if o != Ordering::Equal {
            return o;
        }

        o = self.en_passant_square.cmp(&other.en_passant_square);
        if o != Ordering::Equal {
            return o;
        }

        Ordering::Equal
    }
}

impl PartialOrd for PositionWithMeta {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PositionWithMeta {
    fn eq(&self, other: &Self) -> bool {
        self.white_king == other.white_king
            && self.white_queen == other.white_queen
            && self.white_rook == other.white_rook
            && self.white_bishop == other.white_bishop
            && self.white_knight == other.white_knight
            && self.white_pawn == other.white_king
            && self.black_king == other.black_king
            && self.black_queen == other.black_queen
            && self.black_rook == other.black_rook
            && self.black_bishop == other.black_bishop
            && self.black_knight == other.black_knight
            && self.black_pawn == other.black_king
            && self.player == other.player
            && self.castle_white_kingside == other.castle_white_kingside
            && self.castle_white_queenside == other.castle_white_queenside
            && self.castle_black_kingside == other.castle_black_kingside
            && self.castle_black_queenside == other.castle_black_queenside
            && self.en_passant_square == other.en_passant_square
    }
}

impl Eq for PositionWithMeta {}

impl PositionWithMeta {
    fn empty() -> PositionWithMeta {
        PositionWithMeta {
            white_king: Bitboard::EMPTY,
            white_queen: Bitboard::EMPTY,
            white_rook: Bitboard::EMPTY,
            white_bishop: Bitboard::EMPTY,
            white_knight: Bitboard::EMPTY,
            white_pawn: Bitboard::EMPTY,
            black_king: Bitboard::EMPTY,
            black_queen: Bitboard::EMPTY,
            black_rook: Bitboard::EMPTY,
            black_bishop: Bitboard::EMPTY,
            black_knight: Bitboard::EMPTY,
            black_pawn: Bitboard::EMPTY,
            player: true,
            castle_white_kingside: true,
            castle_white_queenside: true,
            castle_black_kingside: true,
            castle_black_queenside: true,
            en_passant_square: Bitboard::EMPTY,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub position: Position,
    pub player: bool,
    pub last_move: Option<MoveIndex>,
    pub possible_castles: PossibleCastles,
    pub en_passant_square: Bitboard,
    previous_positions: Vec<PositionWithMeta>,
    move_counter: i32,
    fifty_move_counter: i32,
}

impl Game {
    pub fn from_fen(fen: &str) -> Game {
        let fen_parts: Vec<&str> = fen.split(" ").collect();
        let mut position = Position {
            all: Bitboard::EMPTY,
            white: Pieces {
                all: Bitboard::EMPTY,
                king: Bitboard::EMPTY,
                queen: Bitboard::EMPTY,
                rook: Bitboard::EMPTY,
                bishop: Bitboard::EMPTY,
                knight: Bitboard::EMPTY,
                pawn: Bitboard::EMPTY,
            },
            black: Pieces {
                all: Bitboard::EMPTY,
                king: Bitboard::EMPTY,
                queen: Bitboard::EMPTY,
                rook: Bitboard::EMPTY,
                bishop: Bitboard::EMPTY,
                knight: Bitboard::EMPTY,
                pawn: Bitboard::EMPTY,
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
            _ => Bitboard::EMPTY,
        };

        return Game {
            position,
            player: fen_parts[1] == "w",
            last_move: None,
            possible_castles: PossibleCastles {
                white_kingside: fen_parts[2].contains("K"),
                white_queenside: fen_parts[2].contains("Q"),
                black_kingside: fen_parts[2].contains("k"),
                black_queenside: fen_parts[2].contains("q"),
            },
            en_passant_square,
            previous_positions: vec![],
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

    pub fn make_move(&self, m: &Move, store: bool) -> Game {
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

        let mut previous_positions: Vec<PositionWithMeta>;
        if !(is_capturing != CapturedPiece::None
            || m.is_promoting_to.is_some()
            || m.is_castling.is_some())
        {
            previous_positions = vec![];
        } else {
            previous_positions = self.previous_positions.clone();
            previous_positions.push(PositionWithMeta {
                white_king: self.position.white.king,
                white_queen: self.position.white.queen,
                white_rook: self.position.white.rook,
                white_bishop: self.position.white.bishop,
                white_knight: self.position.white.knight,
                white_pawn: self.position.white.pawn,
                black_king: self.position.black.king,
                black_queen: self.position.black.queen,
                black_rook: self.position.black.rook,
                black_bishop: self.position.black.bishop,
                black_knight: self.position.black.knight,
                black_pawn: self.position.black.pawn,
                player: self.player,
                castle_white_kingside: self.possible_castles.white_kingside,
                castle_white_queenside: self.possible_castles.white_queenside,
                castle_black_kingside: self.possible_castles.black_kingside,
                castle_black_queenside: self.possible_castles.black_queenside,
                en_passant_square: self.en_passant_square,
            });
        }

        Game {
            position: new_position,
            player,
            last_move: if store { Some(m.index()) } else { None },
            possible_castles,
            en_passant_square,
            previous_positions,
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
                en_passant_square: Bitboard::EMPTY,
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: Some(PromotionPiece::Queen),
            });
            result.push(Move {
                player: self.player,
                piece: Piece::Pawn,
                from_square,
                to_square,
                en_passant_square: Bitboard::EMPTY,
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: Some(PromotionPiece::Rook),
            });
            result.push(Move {
                player: self.player,
                piece: Piece::Pawn,
                from_square,
                to_square,
                en_passant_square: Bitboard::EMPTY,
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: Some(PromotionPiece::Bishop),
            });
            result.push(Move {
                player: self.player,
                piece: Piece::Pawn,
                from_square,
                to_square,
                en_passant_square: Bitboard::EMPTY,
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
                en_passant_square: Bitboard::EMPTY,
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: None,
            });
        };

        result
    }

    pub fn legal_moves(&self, player: bool) -> Vec<Move> {
        let mut result: Vec<Move> = vec![];

        let friendly_pieces = if player {
            self.position.white.all
        } else {
            self.position.black.all
        };
        let enemy_pieces = if player {
            self.position.black.all
        } else {
            self.position.white.all
        };
        let empty_squares = Bitboard::ALL ^ self.position.all;
        let attacked_squares = self.position.attacked_squares(!player);

        let king = if player {
            self.position.white.king
        } else {
            self.position.black.king
        };
        let mut king_moves = king.king_moves() & (Bitboard::ALL ^ attacked_squares);
        king_moves = king_moves ^ (king_moves & friendly_pieces);
        for to_square in king_moves.into_iter() {
            result.push(Move {
                player,
                piece: Piece::King,
                from_square: king,
                to_square,
                en_passant_square: Bitboard::EMPTY,
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: None,
            })
        }

        let attackers = self.position.attackers(!player, king);

        let number_of_attackers = attackers.count_ones();
        if number_of_attackers > 1 {
            // Multiple pieces are giving check, so the king has to move
            return result;
        }

        let mut capture_mask = Bitboard::ALL;
        let mut push_mask = Bitboard::ALL;
        if number_of_attackers == 1 {
            capture_mask = attackers;

            let knight = if player {
                self.position.black.knight
            } else {
                self.position.white.knight
            };
            let pawn = if player {
                self.position.black.pawn
            } else {
                self.position.white.pawn
            };
            if (!(attackers & knight).is_empty()) || (!(attackers & pawn).is_empty()) {
                // checked by knight or pawn, this can't be blocked
                push_mask = Bitboard::EMPTY;
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

        let enemy_queens = if player {
            self.position.black.queen
        } else {
            self.position.white.queen
        };
        let enemy_queens_and_rooks = enemy_queens
            | (if player {
                self.position.black.rook
            } else {
                self.position.white.rook
            });
        let enemy_queens_and_bishops = enemy_queens
            | (if player {
                self.position.black.bishop
            } else {
                self.position.white.bishop
            });

        let queen = if player {
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
                    player,
                    piece: Piece::Queen,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::EMPTY,
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }
        }

        let rook = if player {
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
                    player,
                    piece: Piece::Rook,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::EMPTY,
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }
        }

        let bishop = if player {
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
                    player,
                    piece: Piece::Bishop,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::EMPTY,
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }
        }

        let knight = if player {
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
                    player,
                    piece: Piece::Knight,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::EMPTY,
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }
        }

        let pawn = if player {
            self.position.white.pawn
        } else {
            self.position.black.pawn
        };
        for from_square in pawn.into_iter() {
            let pinned_movement = self.position.pinned_movement(
                from_square,
                king,
                enemy_queens_and_rooks,
                enemy_queens_and_bishops,
            );

            let forward_square = if player {
                from_square.get_top_square()
            } else {
                from_square.get_bottom_square()
            };

            // Pawn single moves
            let to_square = forward_square & empty_squares & pinned_movement & push_mask;
            if !to_square.is_empty() {
                let promotion_squares = if player {
                    Bitboard::new(0xFF00_0000_0000_0000)
                } else {
                    Bitboard::new(0x0000_0000_0000_00FF)
                };
                if !(to_square & promotion_squares).is_empty() {
                    result.push(Move {
                        player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::EMPTY,
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: Some(PromotionPiece::Queen),
                    });
                    result.push(Move {
                        player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::EMPTY,
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: Some(PromotionPiece::Rook),
                    });
                    result.push(Move {
                        player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::EMPTY,
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: Some(PromotionPiece::Bishop),
                    });
                    result.push(Move {
                        player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::EMPTY,
                        is_capturing_en_passant: false,
                        is_castling: None,
                        is_promoting_to: Some(PromotionPiece::Knight),
                    });
                } else {
                    result.push(Move {
                        player,
                        piece: Piece::Pawn,
                        from_square,
                        to_square,
                        en_passant_square: Bitboard::EMPTY,
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
            let double_forward_square = if player {
                (forward_square & Bitboard::new(0x0000_0000_00FF_0000)).get_top_square()
            } else {
                (forward_square & Bitboard::new(0x0000_FF00_0000_0000)).get_bottom_square()
            };
            let to_square = double_forward_square
                & empty_squares
                & (if player {
                    empty_squares.get_top_square()
                } else {
                    empty_squares.get_bottom_square()
                })
                & pinned_movement
                & push_mask;
            if !to_square.is_empty() {
                result.push(Move {
                    player,
                    piece: Piece::Pawn,
                    from_square,
                    to_square,
                    en_passant_square: (if player {
                        to_square.get_bottom_square()
                    } else {
                        to_square.get_top_square()
                    }),
                    is_capturing_en_passant: false,
                    is_castling: None,
                    is_promoting_to: None,
                })
            }

            let en_passant_captures =
                forward_square.get_left_square() | forward_square.get_right_square();
            let to_square = en_passant_captures
                & self.en_passant_square
                & pinned_movement
                & (if player {
                    capture_mask.get_top_square()
                } else {
                    capture_mask.get_bottom_square()
                });
            if !to_square.is_empty() {
                let m = Move {
                    player,
                    piece: Piece::Pawn,
                    from_square,
                    to_square,
                    en_passant_square: Bitboard::EMPTY,
                    is_capturing_en_passant: true,
                    is_castling: None,
                    is_promoting_to: None,
                };
                let position = self.position.make_move(&m).0;
                if !position.is_check(player) {
                    result.push(m);
                }
            }
        }

        let kingside_castle = if player {
            self.possible_castles.white_kingside
        } else {
            self.possible_castles.black_kingside
        };
        let kingside_squares_between = self.position.all
            & (if player {
                Bitboard::new(0x0000_0000_0000_0006)
            } else {
                Bitboard::new(0x0600_0000_0000_0000)
            });
        let kingside_attacks = attacked_squares
            & (if player {
                Bitboard::new(0x0000_0000_0000_000E)
            } else {
                Bitboard::new(0x0E00_0000_0000_0000)
            });
        let can_castle_kingside =
            kingside_castle && kingside_squares_between.is_empty() && kingside_attacks.is_empty();

        if can_castle_kingside {
            result.push(Move {
                player,
                piece: Piece::King,
                from_square: if player {
                    Bitboard::new(0x0000_0000_0000_0008)
                } else {
                    Bitboard::new(0x0800_0000_0000_0000)
                },
                to_square: if player {
                    Bitboard::new(0x0000_0000_0000_0002)
                } else {
                    Bitboard::new(0x0200_0000_0000_0000)
                },
                en_passant_square: Bitboard::EMPTY,
                is_capturing_en_passant: false,
                is_castling: Some(Castle::Kingside),
                is_promoting_to: None,
            })
        }

        let queenside_castle = if player {
            self.possible_castles.white_queenside
        } else {
            self.possible_castles.black_queenside
        };
        let queenside_squares_between = self.position.all
            & (if player {
                Bitboard::new(0x0000_0000_0000_0070)
            } else {
                Bitboard::new(0x7000_0000_0000_0000)
            });
        let queenside_attacks = attacked_squares
            & (if player {
                Bitboard::new(0x0000_0000_0000_0038)
            } else {
                Bitboard::new(0x3800_0000_0000_0000)
            });
        let can_castle_queenside = queenside_castle
            && queenside_squares_between.is_empty()
            && queenside_attacks.is_empty();

        if can_castle_queenside {
            result.push(Move {
                player,
                piece: Piece::King,
                from_square: if player {
                    Bitboard::new(0x0000_0000_0000_0008)
                } else {
                    Bitboard::new(0x0800_0000_0000_0000)
                },
                to_square: if player {
                    Bitboard::new(0x0000_0000_0000_0020)
                } else {
                    Bitboard::new(0x2000_0000_0000_0000)
                },
                en_passant_square: Bitboard::EMPTY,
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

        self.legal_moves(self.player)
            .par_iter()
            .map(|m| self.make_move(m, false).count_legal_moves(depth - 1))
            .sum()
    }

    pub fn count_legal_moves2(&self, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        LegalMoves::new(self)
            .par_bridge()
            .map(|m| self.make_move(&m, false).count_legal_moves2(depth - 1))
            .sum()
    }

    pub fn result(&mut self) -> Option<GameResult> {
        let legal_moves = self.legal_moves(self.player).len();
        if legal_moves == 0 {
            if self.position.is_check(self.player) {
                return if self.player {
                    Some(GameResult::Black)
                } else {
                    Some(GameResult::White)
                };
            }
            return Some(GameResult::Stalemate);
        }

        if self.fifty_move_counter >= 100 {
            return Some(GameResult::FiftyMoveRule);
        }

        self.previous_positions.sort();
        let mut count = 1;
        let mut prev = PositionWithMeta::empty();
        for id in self.previous_positions.iter() {
            if prev == *id {
                count += 1;
            } else {
                count = 1;
                prev = *id;
            }
            if count >= 3 {
                return Some(GameResult::Repitition);
            }
        }

        if self.position.is_dead() {
            return Some(GameResult::DeadPosition);
        }

        return None;
    }
}

#[derive(Clone, Eq, PartialEq)]
enum PieceMove {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    PawnSingleMove,
    PawnAttack,
    PawnDoubleMove,
    PawnEnPassant,
    KingsideCastle,
    QueensideCastle,
}

impl Into<Piece> for PieceMove {
    fn into(self) -> Piece {
        match self {
            PieceMove::King | PieceMove::KingsideCastle | PieceMove::QueensideCastle => Piece::King,
            PieceMove::Queen => Piece::Queen,
            PieceMove::Rook => Piece::Rook,
            PieceMove::Bishop => Piece::Bishop,
            PieceMove::Knight => Piece::Knight,
            PieceMove::PawnSingleMove
            | PieceMove::PawnAttack
            | PieceMove::PawnDoubleMove
            | PieceMove::PawnEnPassant => Piece::Pawn,
        }
    }
}

impl Into<Option<Castle>> for PieceMove {
    fn into(self) -> Option<Castle> {
        match self {
            PieceMove::KingsideCastle => Some(Castle::Kingside),
            PieceMove::QueensideCastle => Some(Castle::Queenside),
            _ => None,
        }
    }
}

struct LegalMoves {
    player: bool,
    position: Position,
    en_passant_square: Bitboard,
    possible_castles: PossibleCastles,

    friendly_pieces: Bitboard,
    enemy_pieces: Bitboard,
    empty_squares: Bitboard,
    attacked_squares: Bitboard,
    attackers: Bitboard,
    capture_mask: Bitboard,
    push_mask: Bitboard,
    capture_or_push_mask: Bitboard,
    king: Bitboard,
    enemy_queens_and_rooks: Bitboard,
    enemy_queens_and_bishops: Bitboard,

    current_piece: (Bitboard, u8, PieceMove),
    current_from: Bitboard,

    current_moves: (Bitboard, u8),

    promotion_squares: Bitboard,
    last_promotion: Option<PromotionPiece>,
}

impl LegalMoves {
    fn new(game: &Game) -> Self {
        let king = if game.player {
            game.position.white.king
        } else {
            game.position.black.king
        };

        let friendly_pieces = if game.player {
            game.position.white.all
        } else {
            game.position.black.all
        };
        let enemy_pieces = if game.player {
            game.position.black.all
        } else {
            game.position.white.all
        };
        let empty_squares = Bitboard::ALL ^ game.position.all;
        let attacked_squares = game.position.attacked_squares(!game.player);
        let attackers = game.position.attackers(!game.player, king);

        let mut king_moves = king.king_moves() & (Bitboard::ALL ^ attacked_squares);
        king_moves = king_moves ^ (king_moves & friendly_pieces);

        LegalMoves {
            position: game.position,
            player: game.player,
            en_passant_square: game.en_passant_square,
            possible_castles: game.possible_castles,

            friendly_pieces,
            enemy_pieces,
            empty_squares,
            attacked_squares,
            attackers,
            capture_mask: Bitboard::ALL,
            push_mask: Bitboard::ALL,
            capture_or_push_mask: Bitboard::ALL,
            king,
            enemy_queens_and_rooks: Bitboard::EMPTY,
            enemy_queens_and_bishops: Bitboard::EMPTY,

            current_piece: (Bitboard::EMPTY, 0, PieceMove::King),
            current_from: king,

            current_moves: (king_moves, 0),

            promotion_squares: if game.player {
                Bitboard::new(0xFF00_0000_0000_0000)
            } else {
                Bitboard::new(0x0000_0000_0000_00FF)
            },
            last_promotion: None,
        }
    }

    pub fn get_push_squares_in_direction(
        &self,
        square: Bitboard,
        attackers: Bitboard,
        direction: Direction,
    ) -> Bitboard {
        let mut moves = Bitboard::EMPTY;
        let mut running = square.get_square_in_direction(direction);

        while !running.is_empty() {
            if !(attackers & running).is_empty() {
                return moves;
            } else {
                moves |= running;
                running = running.get_square_in_direction(direction);
            }
        }

        Bitboard::EMPTY
    }

    fn calcualte_capture_and_push_mask(&mut self) {
        self.capture_mask = self.attackers;

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
        if (!(self.attackers & knight).is_empty()) || (!(self.attackers & pawn).is_empty()) {
            // checked by knight or pawn, this can't be blocked
            self.push_mask = Bitboard::EMPTY;
        } else {
            let king = if self.player {
                self.position.white.king
            } else {
                self.position.black.king
            };
            self.push_mask =
                self.get_push_squares_in_direction(king, self.attackers, Direction::Top)
                    | self.get_push_squares_in_direction(king, self.attackers, Direction::Bottom)
                    | self.get_push_squares_in_direction(king, self.attackers, Direction::Left)
                    | self.get_push_squares_in_direction(king, self.attackers, Direction::Right)
                    | self.get_push_squares_in_direction(king, self.attackers, Direction::TopLeft)
                    | self.get_push_squares_in_direction(king, self.attackers, Direction::TopRight)
                    | self.get_push_squares_in_direction(
                        king,
                        self.attackers,
                        Direction::BottomLeft,
                    )
                    | self.get_push_squares_in_direction(
                        king,
                        self.attackers,
                        Direction::BottomRight,
                    )
        }

        self.capture_or_push_mask = self.capture_mask | self.push_mask;
    }

    fn calculate_enemy_pieces(&mut self) {
        let enemy_queens = if self.player {
            self.position.black.queen
        } else {
            self.position.white.queen
        };
        self.enemy_queens_and_rooks = enemy_queens
            | (if self.player {
                self.position.black.rook
            } else {
                self.position.white.rook
            });
        self.enemy_queens_and_bishops = enemy_queens
            | (if self.player {
                self.position.black.bishop
            } else {
                self.position.white.bishop
            });
    }

    fn queen_moves(&self, from_square: Bitboard) -> Bitboard {
        self.capture_or_push_mask
            & (get_rank_and_file_moves(self.position.all, self.enemy_pieces, from_square)
                | get_diagonal_moves(self.position.all, self.enemy_pieces, from_square))
            & self.position.pinned_movement(
                from_square,
                self.king,
                self.enemy_queens_and_rooks,
                self.enemy_queens_and_bishops,
            )
    }

    fn rook_moves(&self, from_square: Bitboard) -> Bitboard {
        self.capture_or_push_mask
            & get_rank_and_file_moves(self.position.all, self.enemy_pieces, from_square)
            & self.position.pinned_movement(
                from_square,
                self.king,
                self.enemy_queens_and_rooks,
                self.enemy_queens_and_bishops,
            )
    }

    fn bishop_moves(&self, from_square: Bitboard) -> Bitboard {
        self.capture_or_push_mask
            & get_diagonal_moves(self.position.all, self.enemy_pieces, from_square)
            & self.position.pinned_movement(
                from_square,
                self.king,
                self.enemy_queens_and_rooks,
                self.enemy_queens_and_bishops,
            )
    }

    fn knight_moves(&self, from_square: Bitboard) -> Bitboard {
        let knight_moves = from_square.knight_moves();
        self.capture_or_push_mask
            & knight_moves
            & (knight_moves ^ self.friendly_pieces)
            & self.position.pinned_movement(
                from_square,
                self.king,
                self.enemy_queens_and_rooks,
                self.enemy_queens_and_bishops,
            )
    }

    fn pawn_single_moves(&self, from_square: Bitboard) -> Bitboard {
        let pinned_movement = self.position.pinned_movement(
            from_square,
            self.king,
            self.enemy_queens_and_rooks,
            self.enemy_queens_and_bishops,
        );

        let forward_square = if self.player {
            from_square.get_top_square()
        } else {
            from_square.get_bottom_square()
        };

        forward_square & self.empty_squares & pinned_movement & self.push_mask
    }

    fn pawn_attack_moves(&self, from_square: Bitboard) -> Bitboard {
        let forward_square = if self.player {
            from_square.get_top_square()
        } else {
            from_square.get_bottom_square()
        };

        let pinned_movement = self.position.pinned_movement(
            from_square,
            self.king,
            self.enemy_queens_and_rooks,
            self.enemy_queens_and_bishops,
        );

        (forward_square.get_left_square() | forward_square.get_right_square())
            & self.enemy_pieces
            & pinned_movement
            & self.capture_mask
    }

    fn pawn_double_moves(&self, from_square: Bitboard) -> Bitboard {
        let forward_square = if self.player {
            from_square.get_top_square()
        } else {
            from_square.get_bottom_square()
        };

        let pinned_movement = self.position.pinned_movement(
            from_square,
            self.king,
            self.enemy_queens_and_rooks,
            self.enemy_queens_and_bishops,
        );

        let double_forward_square = if self.player {
            (forward_square & Bitboard::new(0x0000_0000_00FF_0000)).get_top_square()
        } else {
            (forward_square & Bitboard::new(0x0000_FF00_0000_0000)).get_bottom_square()
        };
        double_forward_square
            & self.empty_squares
            & (if self.player {
                self.empty_squares.get_top_square()
            } else {
                self.empty_squares.get_bottom_square()
            })
            & pinned_movement
            & self.push_mask
    }

    fn pawn_en_passant_moves(&self, from_square: Bitboard) -> Bitboard {
        let forward_square = if self.player {
            from_square.get_top_square()
        } else {
            from_square.get_bottom_square()
        };

        let pinned_movement = self.position.pinned_movement(
            from_square,
            self.king,
            self.enemy_queens_and_rooks,
            self.enemy_queens_and_bishops,
        );

        let en_passant_captures =
            forward_square.get_left_square() | forward_square.get_right_square();
        let to_square = en_passant_captures
            & self.en_passant_square
            & pinned_movement
            & (if self.player {
                self.capture_mask.get_top_square()
            } else {
                self.capture_mask.get_bottom_square()
            });

        let m = Move {
            player: self.player,
            piece: Piece::Pawn,
            from_square,
            to_square,
            en_passant_square: Bitboard::EMPTY,
            is_capturing_en_passant: true,
            is_castling: None,
            is_promoting_to: None,
        };

        let position = self.position.make_move(&m).0;
        if position.is_check(self.player) {
            Bitboard::EMPTY
        } else {
            to_square
        }
    }

    fn kingside_castle_moves(&self) -> Bitboard {
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
        let kingside_attacks = self.attacked_squares
            & (if self.player {
                Bitboard::new(0x0000_0000_0000_000E)
            } else {
                Bitboard::new(0x0E00_0000_0000_0000)
            });
        let can_castle_kingside =
            kingside_castle && kingside_squares_between.is_empty() && kingside_attacks.is_empty();

        match (can_castle_kingside, self.player) {
            (true, true) => Bitboard::new(0x0000_0000_0000_0002),
            (true, false) => Bitboard::new(0x0200_0000_0000_0000),
            _ => Bitboard::EMPTY,
        }
    }

    fn queenside_castle_moves(&self) -> Bitboard {
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
        let queenside_attacks = self.attacked_squares
            & (if self.player {
                Bitboard::new(0x0000_0000_0000_0038)
            } else {
                Bitboard::new(0x3800_0000_0000_0000)
            });
        let can_castle_queenside = queenside_castle
            && queenside_squares_between.is_empty()
            && queenside_attacks.is_empty();

        match (can_castle_queenside, self.player) {
            (true, true) => Bitboard::new(0x0000_0000_0000_0020),
            (true, false) => Bitboard::new(0x2000_0000_0000_0000),
            _ => Bitboard::EMPTY,
        }
    }
}

impl Iterator for LegalMoves {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_moves.0.is_empty() {
            while self.current_piece.0.is_empty() {
                // Select the next kind of piece once we handled all pieces of the current kind.
                self.current_piece = match self.current_piece.2 {
                    // Multiple pieces are giving check, so the king has to move.
                    // We don't need to check moves for other pieces.
                    PieceMove::King if self.attackers.count_ones() > 1 => return None,
                    PieceMove::King => {
                        // We only calculate capture and push mask here because we don't need to
                        // do that if we return early because of double-check.
                        if self.attackers.count_ones() == 1 {
                            self.calcualte_capture_and_push_mask();
                        }
                        self.calculate_enemy_pieces();

                        (
                            if self.player {
                                self.position.white.queen
                            } else {
                                self.position.black.queen
                            },
                            0,
                            PieceMove::Queen,
                        )
                    }
                    PieceMove::Queen => (
                        if self.player {
                            self.position.white.rook
                        } else {
                            self.position.black.rook
                        },
                        0,
                        PieceMove::Rook,
                    ),
                    PieceMove::Rook => (
                        if self.player {
                            self.position.white.bishop
                        } else {
                            self.position.black.bishop
                        },
                        0,
                        PieceMove::Bishop,
                    ),
                    PieceMove::Bishop => (
                        if self.player {
                            self.position.white.knight
                        } else {
                            self.position.black.knight
                        },
                        0,
                        PieceMove::Knight,
                    ),
                    PieceMove::Knight => (
                        if self.player {
                            self.position.white.pawn
                        } else {
                            self.position.black.pawn
                        },
                        0,
                        PieceMove::PawnSingleMove,
                    ),
                    PieceMove::PawnSingleMove => (
                        if self.player {
                            self.position.white.pawn
                        } else {
                            self.position.black.pawn
                        },
                        0,
                        PieceMove::PawnAttack,
                    ),
                    PieceMove::PawnAttack => (
                        if self.player {
                            self.position.white.pawn
                        } else {
                            self.position.black.pawn
                        },
                        0,
                        PieceMove::PawnDoubleMove,
                    ),
                    PieceMove::PawnDoubleMove => (
                        if self.player {
                            self.position.white.pawn
                        } else {
                            self.position.black.pawn
                        },
                        0,
                        PieceMove::PawnEnPassant,
                    ),
                    PieceMove::PawnEnPassant => (
                        if self.player {
                            self.position.white.king
                        } else {
                            self.position.black.king
                        },
                        0,
                        PieceMove::KingsideCastle,
                    ),
                    PieceMove::KingsideCastle => (
                        if self.player {
                            self.position.white.king
                        } else {
                            self.position.black.king
                        },
                        0,
                        PieceMove::QueensideCastle,
                    ),
                    // Moves for all pieces have been handled
                    PieceMove::QueensideCastle => return None,
                }
            }

            while (self.current_piece.0 & Bitboard::new(1)).is_empty() {
                self.current_piece.0 >>= Bitboard::new(1);
                self.current_piece.1 += 1;
            }
            self.current_piece.0 >>= Bitboard::new(1);
            self.current_piece.1 += 1;
            self.current_from = Bitboard::new(1 << self.current_piece.1 - 1);

            self.current_moves = (
                match self.current_piece.2 {
                    // This is unreachable because we start with the king and there can be only one
                    PieceMove::King => unreachable!(),
                    PieceMove::Queen => self.queen_moves(self.current_from),
                    PieceMove::Rook => self.rook_moves(self.current_from),
                    PieceMove::Bishop => self.bishop_moves(self.current_from),
                    PieceMove::Knight => self.knight_moves(self.current_from),
                    PieceMove::PawnSingleMove => self.pawn_single_moves(self.current_from),
                    PieceMove::PawnAttack => self.pawn_attack_moves(self.current_from),
                    PieceMove::PawnDoubleMove => self.pawn_double_moves(self.current_from),
                    PieceMove::PawnEnPassant => self.pawn_en_passant_moves(self.current_from),
                    PieceMove::KingsideCastle => self.kingside_castle_moves(),
                    PieceMove::QueensideCastle => self.queenside_castle_moves(),
                },
                0,
            );
        }

        while (self.current_moves.0 & Bitboard::new(1)).is_empty() {
            self.current_moves.0 >>= Bitboard::new(1);
            self.current_moves.1 += 1;
        }
        self.current_moves.0 >>= Bitboard::new(1);
        self.current_moves.1 += 1;

        let to_square = Bitboard::new(1 << self.current_moves.1 - 1);

        // Note that double moves can't promote, so we can ignore that here
        let is_pawn = self.current_piece.2 == PieceMove::PawnSingleMove
            || self.current_piece.2 == PieceMove::PawnAttack;
        let will_promote = !(to_square & self.promotion_squares).is_empty();
        if self.last_promotion == Some(PromotionPiece::Knight) {
            self.last_promotion = None;
        }
        if is_pawn && will_promote && self.last_promotion != Some(PromotionPiece::Knight) {
            match self.last_promotion {
                None => self.last_promotion = Some(PromotionPiece::Queen),
                Some(PromotionPiece::Queen) => self.last_promotion = Some(PromotionPiece::Rook),
                Some(PromotionPiece::Rook) => self.last_promotion = Some(PromotionPiece::Bishop),
                Some(PromotionPiece::Bishop) => self.last_promotion = Some(PromotionPiece::Knight),
                Some(PromotionPiece::Knight) => unreachable!(),
            };

            if self.last_promotion != Some(PromotionPiece::Knight) {
                self.current_moves.0 =
                    (self.current_moves.0 << Bitboard::new(1)) | Bitboard::new(1);
                self.current_moves.1 -= 1;
            }

            Some(Move {
                player: self.player,
                piece: self.current_piece.2.clone().into(),
                from_square: self.current_from,
                to_square,
                en_passant_square: Bitboard::EMPTY,
                is_capturing_en_passant: false,
                is_castling: None,
                is_promoting_to: self.last_promotion.clone(),
            })
        } else {
            let en_passant_square = match (&self.current_piece.2, self.player) {
                (PieceMove::PawnDoubleMove, true) => to_square.get_bottom_square(),
                (PieceMove::PawnDoubleMove, false) => to_square.get_top_square(),
                _ => Bitboard::EMPTY,
            };
            Some(Move {
                player: self.player,
                piece: self.current_piece.2.clone().into(),
                from_square: self.current_from,
                to_square,
                en_passant_square,
                is_capturing_en_passant: self.current_piece.2 == PieceMove::PawnEnPassant,
                is_castling: self.current_piece.2.clone().into(),
                is_promoting_to: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_1() {
        let cases = [(1, 20), (2, 400), (3, 8902), (4, 197281), (5, 4865609)];
        for (depth, moves) in cases {
            let game = Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            assert_eq!(game.count_legal_moves(depth,), moves);
            assert_eq!(game.count_legal_moves2(depth,), moves);
        }
    }

    #[test]
    fn test_position_2() {
        let cases = [(1, 48), (2, 2039), (3, 97862), (4, 4085603)];
        for (depth, moves) in cases {
            let game = Game::from_fen(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            );
            assert_eq!(game.count_legal_moves(depth,), moves);
            assert_eq!(game.count_legal_moves2(depth,), moves);
        }
    }

    #[test]
    fn test_position_3() {
        let cases = [(1, 14), (2, 191), (3, 2812), (4, 43238), (5, 674624)];
        for (depth, moves) in cases {
            let game = Game::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
            assert_eq!(game.count_legal_moves(depth,), moves);
            assert_eq!(game.count_legal_moves2(depth,), moves);
        }
    }

    #[test]
    fn test_position_4() {
        let cases = [(1, 6), (2, 264), (3, 9467), (4, 422333)];
        for (depth, moves) in cases {
            let game =
                Game::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
            assert_eq!(game.count_legal_moves(depth,), moves);
            assert_eq!(game.count_legal_moves2(depth,), moves);
        }
    }

    #[test]
    fn test_position_5() {
        let cases = [(1, 44), (2, 1486), (3, 62379), (4, 2103487)];
        for (depth, moves) in cases {
            let game = Game::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
            assert_eq!(game.count_legal_moves(depth,), moves);
            assert_eq!(game.count_legal_moves2(depth,), moves);
        }
    }

    #[test]
    fn test_position_6() {
        let cases = [(1, 46), (2, 2079), (3, 89890), (4, 3894594)];
        for (depth, moves) in cases {
            let game = Game::from_fen(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            );
            assert_eq!(game.count_legal_moves(depth,), moves);
            assert_eq!(game.count_legal_moves2(depth,), moves);
        }
    }
}
