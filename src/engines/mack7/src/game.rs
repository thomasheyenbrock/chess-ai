use crate::bitboard::Bitboard;
use crate::constants;
use crate::constants::Constants;
use rayon::prelude::*;
use std::collections::HashMap;

enum Result {
    White,
    Black,
    Stalemate,
    DeadPosition,
    Repitition,
    FiftyMoveRule,
}

fn get_rank_and_file_moves(
    all_pieces: Bitboard,
    enemy_pieces: Bitboard,
    square: Bitboard,
    constants: &Constants,
) -> Bitboard {
    let north_pieces = *constants.north_ray.get(square) & all_pieces;
    let south_pieces = *constants.south_ray.get(square) & all_pieces;
    let west_pieces = *constants.west_ray.get(square) & all_pieces;
    let east_pieces = *constants.east_ray.get(square) & all_pieces;

    let north_moves = *constants.north_moves.get(square).get(north_pieces)
        ^ (*constants.north_attacks.get(square).get(north_pieces) & enemy_pieces);
    let south_moves = *constants.south_moves.get(square).get(south_pieces)
        ^ (*constants.south_attacks.get(square).get(south_pieces) & enemy_pieces);
    let west_moves = *constants.west_moves.get(square).get(west_pieces)
        ^ (*constants.west_attacks.get(square).get(west_pieces) & enemy_pieces);
    let east_moves = *constants.east_moves.get(square).get(east_pieces)
        ^ (*constants.east_attacks.get(square).get(east_pieces) & enemy_pieces);

    return north_moves | south_moves | west_moves | east_moves;
}

fn get_diagonal_moves(
    all_pieces: Bitboard,
    enemy_pieces: Bitboard,
    square: Bitboard,
    constants: &Constants,
) -> Bitboard {
    let north_west_pieces = *constants.north_west_ray.get(square) & all_pieces;
    let south_west_pieces = *constants.south_west_ray.get(square) & all_pieces;
    let north_east_pieces = *constants.north_east_ray.get(square) & all_pieces;
    let south_east_pieces = *constants.south_east_ray.get(square) & all_pieces;

    let north_west_moves = *constants
        .north_west_moves
        .get(square)
        .get(north_west_pieces)
        ^ (*constants
            .north_west_attacks
            .get(square)
            .get(north_west_pieces)
            & enemy_pieces);
    let north_east_moves = *constants
        .north_east_moves
        .get(square)
        .get(north_east_pieces)
        ^ (*constants
            .north_east_attacks
            .get(square)
            .get(north_east_pieces)
            & enemy_pieces);
    let south_west_moves = *constants
        .south_west_moves
        .get(square)
        .get(south_west_pieces)
        ^ (*constants
            .south_west_attacks
            .get(square)
            .get(south_west_pieces)
            & enemy_pieces);
    let south_east_moves = *constants
        .south_east_moves
        .get(square)
        .get(south_east_pieces)
        ^ (*constants
            .south_east_attacks
            .get(square)
            .get(south_east_pieces)
            & enemy_pieces);

    return north_west_moves | north_east_moves | south_west_moves | south_east_moves;
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

impl Move {
    fn new(
        player: bool,
        piece: Piece,
        from_square: Bitboard,
        to_square: Bitboard,
        en_passant_square: Bitboard,
        is_capturing_en_passant: bool,
        is_castling: Option<Castle>,
        is_promoting_to: Option<PromotionPiece>,
    ) -> Move {
        Move {
            player,
            piece,
            from_square,
            to_square,
            en_passant_square,
            is_capturing_en_passant,
            is_castling,
            is_promoting_to,
        }
    }
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
    fn new(
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
    ) -> Position {
        let white_pieces =
            white_king | white_queen | white_rook | white_bishop | white_knight | white_pawn;
        let black_pieces =
            black_king | black_queen | black_rook | black_bishop | black_knight | black_pawn;
        Position {
            all: white_pieces | black_pieces,
            white: Pieces {
                all: white_pieces,
                king: white_king,
                queen: white_queen,
                rook: white_rook,
                bishop: white_bishop,
                knight: white_knight,
                pawn: white_pawn,
            },
            black: Pieces {
                all: black_pieces,
                king: black_king,
                queen: black_queen,
                rook: black_rook,
                bishop: black_bishop,
                knight: black_knight,
                pawn: black_pawn,
            },
        }
    }

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

    fn attackers(self, player: bool, square: Bitboard, constants: &Constants) -> Bitboard {
        let pieces = if player { self.white } else { self.black };

        let queen_and_rook = pieces.queen | pieces.rook;
        let queen_and_bishop = pieces.queen | pieces.bishop;

        let north_pieces = *constants.north_ray.get(square) & self.all;
        let south_pieces = *constants.south_ray.get(square) & self.all;
        let west_pieces = *constants.west_ray.get(square) & self.all;
        let east_pieces = *constants.east_ray.get(square) & self.all;
        let north_west_pieces = *constants.north_west_ray.get(square) & self.all;
        let south_west_pieces = *constants.south_west_ray.get(square) & self.all;
        let north_east_pieces = *constants.north_east_ray.get(square) & self.all;
        let south_east_pieces = *constants.south_east_ray.get(square) & self.all;

        (*constants.king_moves.get(square) & pieces.king)
            | (*constants.north_attacks.get(square).get(north_pieces) & queen_and_rook)
            | (*constants.south_attacks.get(square).get(south_pieces) & queen_and_rook)
            | (*constants.west_attacks.get(square).get(west_pieces) & queen_and_rook)
            | (*constants.east_attacks.get(square).get(east_pieces) & queen_and_rook)
            | (*constants
                .north_west_attacks
                .get(square)
                .get(north_west_pieces)
                & queen_and_bishop)
            | (*constants
                .south_west_attacks
                .get(square)
                .get(south_west_pieces)
                & queen_and_bishop)
            | (*constants
                .north_east_attacks
                .get(square)
                .get(north_east_pieces)
                & queen_and_bishop)
            | (*constants
                .south_east_attacks
                .get(square)
                .get(south_east_pieces)
                & queen_and_bishop)
            | (*constants.knight_moves.get(square) & pieces.knight)
            | (*constants.pawn_attacks.get(&player).unwrap().get(square) & pieces.pawn)
    }

    fn checkers(self, player: bool, king: Bitboard, constants: &Constants) -> Bitboard {
        let pieces = if player { self.white } else { self.black };

        let queen_and_rook = pieces.queen | pieces.rook;
        let queen_and_bishop = pieces.queen | pieces.bishop;

        let north_pieces = *constants.north_ray.get(king) & self.all;
        let south_pieces = *constants.south_ray.get(king) & self.all;
        let west_pieces = *constants.west_ray.get(king) & self.all;
        let east_pieces = *constants.east_ray.get(king) & self.all;
        let north_west_pieces = *constants.north_west_ray.get(king) & self.all;
        let south_west_pieces = *constants.south_west_ray.get(king) & self.all;
        let north_east_pieces = *constants.north_east_ray.get(king) & self.all;
        let south_east_pieces = *constants.south_east_ray.get(king) & self.all;

        (*constants.north_attacks.get(king).get(north_pieces) & queen_and_rook)
            | (*constants.south_attacks.get(king).get(south_pieces) & queen_and_rook)
            | (*constants.west_attacks.get(king).get(west_pieces) & queen_and_rook)
            | (*constants.east_attacks.get(king).get(east_pieces) & queen_and_rook)
            | (*constants
                .north_west_attacks
                .get(king)
                .get(north_west_pieces)
                & queen_and_bishop)
            | (*constants
                .south_west_attacks
                .get(king)
                .get(south_west_pieces)
                & queen_and_bishop)
            | (*constants
                .north_east_attacks
                .get(king)
                .get(north_east_pieces)
                & queen_and_bishop)
            | (*constants
                .south_east_attacks
                .get(king)
                .get(south_east_pieces)
                & queen_and_bishop)
            | (*constants.knight_moves.get(king) & pieces.knight)
            | (*constants.pawn_attacks.get(&player).unwrap().get(king) & pieces.pawn)
    }

    fn attacked_squares(
        self,
        player: bool,
        exclude_king: bool, // = false
        constants: &Constants,
    ) -> Bitboard {
        let mut all_pieces = self.all;
        if exclude_king {
            all_pieces ^= if player {
                self.black.king
            } else {
                self.white.king
            };
        }

        let mut attacked = *constants.king_moves.get(if player {
            self.white.king
        } else {
            self.black.king
        });

        let queen_pieces = if player {
            self.white.queen
        } else {
            self.black.queen
        };
        for queen in queen_pieces.split().iter() {
            let north_pieces = *constants.north_ray.get(*queen) & all_pieces;
            let north_moves = *constants.north_moves.get(*queen).get(north_pieces);
            let north_attacks = *constants.north_attacks.get(*queen).get(north_pieces);

            let south_pieces = *constants.south_ray.get(*queen) & all_pieces;
            let south_moves = *constants.south_moves.get(*queen).get(south_pieces);
            let south_attacks = *constants.south_attacks.get(*queen).get(south_pieces);

            let west_pieces = *constants.west_ray.get(*queen) & all_pieces;
            let west_moves = *constants.west_moves.get(*queen).get(west_pieces);
            let west_attacks = *constants.west_attacks.get(*queen).get(west_pieces);

            let east_pieces = *constants.east_ray.get(*queen) & all_pieces;
            let east_moves = *constants.east_moves.get(*queen).get(east_pieces);
            let east_attacks = *constants.east_attacks.get(*queen).get(east_pieces);

            let north_west_pieces = *constants.north_west_ray.get(*queen) & all_pieces;
            let north_west_moves = *constants
                .north_west_moves
                .get(*queen)
                .get(north_west_pieces);
            let north_west_attacks = *constants
                .north_west_attacks
                .get(*queen)
                .get(north_west_pieces);

            let north_east_pieces = *constants.north_east_ray.get(*queen) & all_pieces;
            let north_east_moves = *constants
                .north_east_moves
                .get(*queen)
                .get(north_east_pieces);
            let north_east_attacks = *constants
                .north_east_attacks
                .get(*queen)
                .get(north_east_pieces);

            let south_west_pieces = *constants.south_west_ray.get(*queen) & all_pieces;
            let south_west_moves = *constants
                .south_west_moves
                .get(*queen)
                .get(south_west_pieces);
            let south_west_attacks = *constants
                .south_west_attacks
                .get(*queen)
                .get(south_west_pieces);

            let south_east_pieces = *constants.south_east_ray.get(*queen) & all_pieces;
            let south_east_moves = *constants
                .south_east_moves
                .get(*queen)
                .get(south_east_pieces);
            let south_east_attacks = *constants
                .south_east_attacks
                .get(*queen)
                .get(south_east_pieces);

            attacked = attacked
                | north_moves
                | north_attacks
                | south_moves
                | south_attacks
                | west_moves
                | west_attacks
                | east_moves
                | east_attacks
                | north_west_moves
                | north_west_attacks
                | north_east_moves
                | north_east_attacks
                | south_west_moves
                | south_west_attacks
                | south_east_moves
                | south_east_attacks;
        }

        let rook_pieces = if player {
            self.white.rook
        } else {
            self.black.rook
        };
        for rook in rook_pieces.split().iter() {
            let north_pieces = *constants.north_ray.get(*rook) & all_pieces;
            let north_moves = *constants.north_moves.get(*rook).get(north_pieces);
            let north_attacks = *constants.north_attacks.get(*rook).get(north_pieces);

            let south_pieces = *constants.south_ray.get(*rook) & all_pieces;
            let south_moves = *constants.south_moves.get(*rook).get(south_pieces);
            let south_attacks = *constants.south_attacks.get(*rook).get(south_pieces);

            let west_pieces = *constants.west_ray.get(*rook) & all_pieces;
            let west_moves = *constants.west_moves.get(*rook).get(west_pieces);
            let west_attacks = *constants.west_attacks.get(*rook).get(west_pieces);

            let east_pieces = *constants.east_ray.get(*rook) & all_pieces;
            let east_moves = *constants.east_moves.get(*rook).get(east_pieces);
            let east_attacks = *constants.east_attacks.get(*rook).get(east_pieces);

            attacked = attacked
                | north_moves
                | north_attacks
                | south_moves
                | south_attacks
                | west_moves
                | west_attacks
                | east_moves
                | east_attacks;
        }

        let bishop_pieces = if player {
            self.white.bishop
        } else {
            self.black.bishop
        };
        for bishop in bishop_pieces.split().iter() {
            let north_west_pieces = *constants.north_west_ray.get(*bishop) & all_pieces;
            let north_west_moves = *constants
                .north_west_moves
                .get(*bishop)
                .get(north_west_pieces);
            let north_west_attacks = *constants
                .north_west_attacks
                .get(*bishop)
                .get(north_west_pieces);

            let north_east_pieces = *constants.north_east_ray.get(*bishop) & all_pieces;
            let north_east_moves = *constants
                .north_east_moves
                .get(*bishop)
                .get(north_east_pieces);
            let north_east_attacks = *constants
                .north_east_attacks
                .get(*bishop)
                .get(north_east_pieces);

            let south_west_pieces = *constants.south_west_ray.get(*bishop) & all_pieces;
            let south_west_moves = *constants
                .south_west_moves
                .get(*bishop)
                .get(south_west_pieces);
            let south_west_attacks = *constants
                .south_west_attacks
                .get(*bishop)
                .get(south_west_pieces);

            let south_east_pieces = *constants.south_east_ray.get(*bishop) & all_pieces;
            let south_east_moves = *constants
                .south_east_moves
                .get(*bishop)
                .get(south_east_pieces);
            let south_east_attacks = *constants
                .south_east_attacks
                .get(*bishop)
                .get(south_east_pieces);

            attacked = attacked
                | north_west_moves
                | north_west_attacks
                | north_east_moves
                | north_east_attacks
                | south_west_moves
                | south_west_attacks
                | south_east_moves
                | south_east_attacks;
        }

        let knight_pieces = if player {
            self.white.knight
        } else {
            self.black.knight
        };
        for knight in knight_pieces.split().iter() {
            attacked |= *constants.knight_moves.get(*knight);
        }

        let pawn_pieces = if player {
            self.white.pawn
        } else {
            self.black.pawn
        };
        for pawn in pawn_pieces.split().iter() {
            for s in constants
                .pawn_attack_moves
                .get(&player)
                .unwrap()
                .get(*pawn)
                .iter()
            {
                attacked |= *s;
            }
        }

        attacked
    }

    fn is_check(self, player: bool, constants: &Constants) -> bool {
        let king = if player {
            self.white.king
        } else {
            self.black.king
        };
        !self.attackers(!player, king, constants).is_empty()
    }

    fn pinned_movement(
        self,
        square: Bitboard,
        king: Bitboard,
        enemy_queens_and_rooks: Bitboard,
        enemy_queens_and_bishops: Bitboard,
        constants: &Constants,
    ) -> Bitboard {
        let north_pieces = *constants.north_ray.get(square) & self.all;
        let south_pieces = *constants.south_ray.get(square) & self.all;
        let first_piece_to_north = *constants.north_attacks.get(square).get(north_pieces);
        let first_piece_to_south = *constants.south_attacks.get(square).get(south_pieces);

        let is_pinned_from_north = (first_piece_to_south == king)
            && (!(first_piece_to_north & enemy_queens_and_rooks).is_empty());
        if is_pinned_from_north {
            return first_piece_to_north
                | *constants.north_moves.get(square).get(north_pieces)
                | *constants.south_moves.get(square).get(south_pieces);
        }

        let is_pinned_from_south = (first_piece_to_north == king)
            && (!(first_piece_to_south & enemy_queens_and_rooks).is_empty());
        if is_pinned_from_south {
            return first_piece_to_south
                | *constants.south_moves.get(square).get(south_pieces)
                | *constants.north_moves.get(square).get(north_pieces);
        }

        let west_pieces = *constants.west_ray.get(square) & self.all;
        let east_pieces = *constants.east_ray.get(square) & self.all;
        let first_piece_to_west = *constants.west_attacks.get(square).get(west_pieces);
        let first_piece_to_east = *constants.east_attacks.get(square).get(east_pieces);

        let is_pinned_from_west = (first_piece_to_east == king)
            && (!(first_piece_to_west & enemy_queens_and_rooks).is_empty());
        if is_pinned_from_west {
            return first_piece_to_west
                | *constants.west_moves.get(square).get(west_pieces)
                | *constants.east_moves.get(square).get(east_pieces);
        }

        let is_pinned_from_east = (first_piece_to_west == king)
            && (!(first_piece_to_east & enemy_queens_and_rooks).is_empty());
        if is_pinned_from_east {
            return first_piece_to_east
                | *constants.east_moves.get(square).get(east_pieces)
                | *constants.west_moves.get(square).get(west_pieces);
        }

        let north_west_pieces = *constants.north_west_ray.get(square) & self.all;
        let south_east_pieces = *constants.south_east_ray.get(square) & self.all;
        let first_piece_to_north_west = *constants
            .north_west_attacks
            .get(square)
            .get(north_west_pieces);
        let first_piece_to_south_east = *constants
            .south_east_attacks
            .get(square)
            .get(south_east_pieces);

        let is_pinned_from_north_west = (first_piece_to_south_east == king)
            && (!(first_piece_to_north_west & enemy_queens_and_bishops).is_empty());
        if is_pinned_from_north_west {
            return first_piece_to_north_west
                | *constants
                    .north_west_moves
                    .get(square)
                    .get(north_west_pieces)
                | *constants
                    .south_east_moves
                    .get(square)
                    .get(south_east_pieces);
        }

        let is_pinned_from_south_east = (first_piece_to_north_west == king)
            && (!(first_piece_to_south_east & enemy_queens_and_bishops).is_empty());
        if is_pinned_from_south_east {
            return first_piece_to_south_east
                | *constants
                    .south_east_moves
                    .get(square)
                    .get(south_east_pieces)
                | *constants
                    .north_west_moves
                    .get(square)
                    .get(north_west_pieces);
        }

        let north_east_pieces = *constants.north_east_ray.get(square) & self.all;
        let south_west_pieces = *constants.south_west_ray.get(square) & self.all;
        let first_piece_to_north_east = *constants
            .north_east_attacks
            .get(square)
            .get(north_east_pieces);
        let first_piece_to_south_west = *constants
            .south_west_attacks
            .get(square)
            .get(south_west_pieces);

        let is_pinned_from_north_east = (first_piece_to_south_west == king)
            && (!(first_piece_to_north_east & enemy_queens_and_bishops).is_empty());
        if is_pinned_from_north_east {
            return first_piece_to_north_east
                | *constants
                    .north_east_moves
                    .get(square)
                    .get(north_east_pieces)
                | *constants
                    .south_west_moves
                    .get(square)
                    .get(south_west_pieces);
        }

        let is_pinned_from_south_west = (first_piece_to_north_east == king)
            && (!(first_piece_to_south_west & enemy_queens_and_bishops).is_empty());
        if is_pinned_from_south_west {
            return first_piece_to_south_west
                | *constants
                    .south_west_moves
                    .get(square)
                    .get(south_west_pieces)
                | *constants
                    .north_east_moves
                    .get(square)
                    .get(north_east_pieces);
        }

        Bitboard::new(0xFFFF_FFFF_FFFF_FFFF)
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
    position_counts: HashMap<String, i32>,
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

        let mut position_counts: HashMap<String, i32>;
        if !(is_capturing != CapturedPiece::None
            || m.is_promoting_to.is_some()
            || m.is_castling.is_some())
        {
            position_counts = HashMap::new();
        } else {
            let key = self.id();
            let current = self.position_counts.get(&key).unwrap_or(&0);
            position_counts = self.position_counts.clone();
            position_counts.insert(key, current + 1);
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

    fn legal_moves(&self, constants: &Constants) -> Vec<Move> {
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
        let attacked_squares = self
            .position
            .attacked_squares(!self.player, true, constants);

        let king = if self.player {
            self.position.white.king
        } else {
            self.position.black.king
        };
        let mut king_moves = *constants.king_moves.get(king)
            & (Bitboard::new(0xFFFF_FFFF_FFFF_FFFF) ^ attacked_squares);
        king_moves = king_moves ^ (king_moves & friendly_pieces);
        for to_square in king_moves.split() {
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

        let attackers = self.position.checkers(!self.player, king, constants);

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
                // checked by slider, this can be blocked
                // 🦖 before this was ".getOrDefault(attacker, 0)"
                push_mask = *constants
                    .north_moves
                    .get(king)
                    .get_or_default(attackers, &Bitboard::new(0))
                    | *constants
                        .south_moves
                        .get(king)
                        .get_or_default(attackers, &Bitboard::new(0))
                    | *constants
                        .west_moves
                        .get(king)
                        .get_or_default(attackers, &Bitboard::new(0))
                    | *constants
                        .east_moves
                        .get(king)
                        .get_or_default(attackers, &Bitboard::new(0))
                    | *constants
                        .north_west_moves
                        .get(king)
                        .get_or_default(attackers, &Bitboard::new(0))
                    | *constants
                        .north_east_moves
                        .get(king)
                        .get_or_default(attackers, &Bitboard::new(0))
                    | *constants
                        .south_west_moves
                        .get(king)
                        .get_or_default(attackers, &Bitboard::new(0))
                    | *constants
                        .south_east_moves
                        .get(king)
                        .get_or_default(attackers, &Bitboard::new(0))
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
        for from_square in queen.split() {
            let moveable_squares = capture_or_push_mask
                & (get_rank_and_file_moves(
                    self.position.all,
                    enemy_pieces,
                    from_square,
                    constants,
                ) | get_diagonal_moves(self.position.all, enemy_pieces, from_square, constants))
                & self.position.pinned_movement(
                    from_square,
                    king,
                    enemy_queens_and_rooks,
                    enemy_queens_and_bishops,
                    constants,
                );
            for to_square in moveable_squares.split() {
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
        for from_square in rook.split() {
            let moveable_squares = capture_or_push_mask
                & get_rank_and_file_moves(self.position.all, enemy_pieces, from_square, constants)
                & self.position.pinned_movement(
                    from_square,
                    king,
                    enemy_queens_and_rooks,
                    enemy_queens_and_bishops,
                    constants,
                );
            for to_square in moveable_squares.split() {
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
        for from_square in bishop.split() {
            let moveable_squares = capture_or_push_mask
                & get_diagonal_moves(self.position.all, enemy_pieces, from_square, constants)
                & self.position.pinned_movement(
                    from_square,
                    king,
                    enemy_queens_and_rooks,
                    enemy_queens_and_bishops,
                    constants,
                );
            for to_square in moveable_squares.split() {
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
        for from_square in knight.split() {
            let moveable_squares = capture_or_push_mask
                & *constants.knight_moves.get(from_square)
                & (*constants.knight_moves.get(from_square) ^ friendly_pieces)
                & self.position.pinned_movement(
                    from_square,
                    king,
                    enemy_queens_and_rooks,
                    enemy_queens_and_bishops,
                    constants,
                );
            for to_square in moveable_squares.split() {
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
        for from_square in pawn.split() {
            let mut to_square: Bitboard;

            let pinned_movement = self.position.pinned_movement(
                from_square,
                king,
                enemy_queens_and_rooks,
                enemy_queens_and_bishops,
                constants,
            );
            to_square = *constants
                .pawn_single_moves
                .get(&self.player)
                .unwrap()
                .get(from_square)
                & empty_squares
                & pinned_movement
                & push_mask;
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

            let attacks = constants
                .pawn_attack_moves
                .get(&self.player)
                .unwrap()
                .get(from_square);
            for p in attacks {
                let to_square = *p & enemy_pieces & pinned_movement & capture_mask;
                if to_square.is_empty() {
                    continue;
                }

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
            }

            to_square = *constants
                .pawn_double_moves
                .get(&self.player)
                .unwrap()
                .get(from_square)
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

            to_square = *constants
                .pawn_en_passant_captures
                .get(&self.player)
                .unwrap()
                .get(from_square)
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
                if !position.is_check(self.player, constants) {
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

    pub fn count_legal_moves(&self, depth: u64, constants: &Constants) -> u64 {
        if depth == 0 {
            return 1;
        }

        self.legal_moves(&constants)
            .par_iter()
            .map(|m| self.make_move(m).count_legal_moves(depth - 1, constants))
            .sum()
    }

    fn result(&self, legal_moves: u64, constants: &Constants) -> Option<Result> {
        if legal_moves == 0 {
            if self.position.is_check(self.player, constants) {
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

        for count in self.position_counts.values() {
            if *count >= 3 {
                return Some(Result::Repitition);
            }
        }

        if self.position.is_dead() {
            return Some(Result::DeadPosition);
        }

        return None;
    }
}

pub fn game_from_fen(fen: &str, constants: &Constants) -> Game {
    let fen_parts: Vec<&str> = fen.split(" ").collect();
    let mut position = Position::new(
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
        Bitboard::new(0),
    );

    for (rank_index, rank) in fen_parts[0].split("/").into_iter().enumerate() {
        let mut file_index = 0;
        for piece in rank.chars() {
            match piece.to_digit(10) {
                Some(digit) => file_index += digit as usize,
                None => {
                    let square = constants.squares[rank_index * 8 + file_index];
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

    let en_passant_square = match constants.human_to_squares.get(fen_parts[3]) {
        Some(square) => *square,
        None => Bitboard::new(0x0000_0000_0000_0000),
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
        position_counts: HashMap::new(),
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
        let c1 = constants::get();
        let cases = [(1, 20), (2, 400), (3, 8902), (4, 197281), (5, 4865609)];
        for (depth, moves) in cases {
            let game = game_from_fen(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                &c1,
            );
            assert_eq!(game.count_legal_moves(depth, &c1), moves);
        }
    }

    #[test]
    fn test_position_2() {
        let c = constants::get();
        let cases = [(1, 48), (2, 2039), (3, 97862), (4, 4085603)];
        for (depth, moves) in cases {
            let game = game_from_fen(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
                &c,
            );
            assert_eq!(game.count_legal_moves(depth, &c), moves)
        }
    }

    #[test]
    fn test_position_3() {
        let c = constants::get();
        let cases = [(1, 14), (2, 191), (3, 2812), (4, 43238), (5, 674624)];
        for (depth, moves) in cases {
            let game = game_from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &c);
            assert_eq!(game.count_legal_moves(depth, &c), moves)
        }
    }

    #[test]
    fn test_position_4() {
        let c = constants::get();
        let cases = [(1, 6), (2, 264), (3, 9467), (4, 422333)];
        for (depth, moves) in cases {
            let game = game_from_fen(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
                &c,
            );
            assert_eq!(game.count_legal_moves(depth, &c), moves)
        }
    }

    #[test]
    fn test_position_5() {
        let c = constants::get();
        let cases = [(1, 44), (2, 1486), (3, 62379), (4, 2103487)];
        for (depth, moves) in cases {
            let game = game_from_fen(
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
                &c,
            );
            assert_eq!(game.count_legal_moves(depth, &c), moves)
        }
    }

    #[test]
    fn test_position_6() {
        let c = constants::get();
        let cases = [(1, 46), (2, 2079), (3, 89890), (4, 3894594)];
        for (depth, moves) in cases {
            let game = game_from_fen(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
                &c,
            );
            assert_eq!(game.count_legal_moves(depth, &c), moves)
        }
    }
}
