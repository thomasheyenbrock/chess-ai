use crate::{
    bitboard::Bitboard,
    chess_move::{Castle, Move},
    direction::Direction,
    piece::{CapturedPiece, Piece, PromotionPiece},
};

#[derive(Clone, Copy, Debug)]
pub struct Pieces {
    pub all: Bitboard,
    pub king: Bitboard,
    pub queen: Bitboard,
    pub rook: Bitboard,
    pub bishop: Bitboard,
    pub knight: Bitboard,
    pub pawn: Bitboard,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub all: Bitboard,
    pub white: Pieces,
    pub black: Pieces,
}

impl Position {
    pub fn make_move(self, m: &Move) -> (Position, CapturedPiece) {
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
        let mut attackers = Bitboard::EMPTY;
        let mut running = square.get_square_in_direction(direction);

        while !running.is_empty() {
            if !(pieces & running).is_empty() {
                attackers |= running;
                running = Bitboard::EMPTY;
            } else if (self.all & running).is_empty() {
                running = running.get_square_in_direction(direction);
            } else {
                running = Bitboard::EMPTY;
            }
        }

        attackers
    }

    pub fn attackers(self, player: bool, square: Bitboard) -> Bitboard {
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
        let mut attacked_squares = Bitboard::EMPTY;
        let mut running = square.get_square_in_direction(direction);

        while !running.is_empty() {
            attacked_squares |= running;
            running = if (all_pieces & running).is_empty() {
                running.get_square_in_direction(direction)
            } else {
                Bitboard::EMPTY
            };
        }

        attacked_squares
    }

    pub fn attacked_squares(self, player: bool) -> Bitboard {
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
        attacked |= knight_pieces.knight_moves();

        let pawn_pieces = if player {
            self.white.pawn
        } else {
            self.black.pawn
        };
        let forward_square = if player {
            pawn_pieces.get_top_square()
        } else {
            pawn_pieces.get_bottom_square()
        };
        attacked |= forward_square.get_left_square() | forward_square.get_right_square();

        attacked
    }

    pub fn is_check(self, player: bool) -> bool {
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
        let mut moves = Bitboard::EMPTY;
        let mut running = square.get_square_in_direction(direction);
        let mut found_king = false;
        let mut found_attacker = false;

        while !running.is_empty() {
            if !(king & running).is_empty() {
                found_king = true;
                running = Bitboard::EMPTY;
            } else if !(attackers & running).is_empty() {
                found_attacker = true;
                moves |= running;
                running = Bitboard::EMPTY;
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

    pub fn pinned_movement(
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

        Bitboard::ALL
    }

    pub fn get_push_squares_in_direction(
        self,
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

    pub fn is_dead(self) -> bool {
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
