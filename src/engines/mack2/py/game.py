from __future__ import annotations
from typing import Iterable, Optional, Tuple

from bitboard import get_bottom_square, get_top_square, split
from constants import (
    EAST_MOVES,
    NORTH_EAST_MOVES,
    NORTH_MOVES,
    NORTH_RAY,
    NORTH_WEST_MOVES,
    PAWN_ATTACK_MOVES,
    PAWN_ATTACK_MOVES_PROMOTION,
    SOUTH_EAST_MOVES,
    SOUTH_MOVES,
    SOUTH_RAY,
    SOUTH_WEST_MOVES,
    WEST_MOVES,
    WEST_RAY,
    EAST_RAY,
    NORTH_WEST_RAY,
    NORTH_EAST_RAY,
    SOUTH_WEST_RAY,
    SOUTH_EAST_RAY,
    NORTH_ATTACKS,
    SOUTH_ATTACKS,
    WEST_ATTACKS,
    EAST_ATTACKS,
    NORTH_WEST_ATTACKS,
    NORTH_EAST_ATTACKS,
    SOUTH_WEST_ATTACKS,
    SOUTH_EAST_ATTACKS,
    KING_MOVES,
    KNIGHT_MOVES,
    PAWN_ATTACKS,
)
from enums import Castle, Piece, Player

map_square_to_human_notation = {
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000001: "h1",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000010: "g1",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000100: "f1",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000: "e1",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000: "d1",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100000: "c1",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000000: "b1",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000000: "a1",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000001_00000000: "h2",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000: "g2",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00000100_00000000: "f2",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00001000_00000000: "e2",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00010000_00000000: "d2",
    0b00000000_00000000_00000000_00000000_00000000_00000000_00100000_00000000: "c2",
    0b00000000_00000000_00000000_00000000_00000000_00000000_01000000_00000000: "b2",
    0b00000000_00000000_00000000_00000000_00000000_00000000_10000000_00000000: "a2",
    0b00000000_00000000_00000000_00000000_00000000_00000001_00000000_00000000: "h3",
    0b00000000_00000000_00000000_00000000_00000000_00000010_00000000_00000000: "g3",
    0b00000000_00000000_00000000_00000000_00000000_00000100_00000000_00000000: "f3",
    0b00000000_00000000_00000000_00000000_00000000_00001000_00000000_00000000: "e3",
    0b00000000_00000000_00000000_00000000_00000000_00010000_00000000_00000000: "d3",
    0b00000000_00000000_00000000_00000000_00000000_00100000_00000000_00000000: "c3",
    0b00000000_00000000_00000000_00000000_00000000_01000000_00000000_00000000: "b3",
    0b00000000_00000000_00000000_00000000_00000000_10000000_00000000_00000000: "a3",
    0b00000000_00000000_00000000_00000000_00000001_00000000_00000000_00000000: "h4",
    0b00000000_00000000_00000000_00000000_00000010_00000000_00000000_00000000: "g4",
    0b00000000_00000000_00000000_00000000_00000100_00000000_00000000_00000000: "f4",
    0b00000000_00000000_00000000_00000000_00001000_00000000_00000000_00000000: "e4",
    0b00000000_00000000_00000000_00000000_00010000_00000000_00000000_00000000: "d4",
    0b00000000_00000000_00000000_00000000_00100000_00000000_00000000_00000000: "c4",
    0b00000000_00000000_00000000_00000000_01000000_00000000_00000000_00000000: "b4",
    0b00000000_00000000_00000000_00000000_10000000_00000000_00000000_00000000: "a4",
    0b00000000_00000000_00000000_00000001_00000000_00000000_00000000_00000000: "h5",
    0b00000000_00000000_00000000_00000010_00000000_00000000_00000000_00000000: "g5",
    0b00000000_00000000_00000000_00000100_00000000_00000000_00000000_00000000: "f5",
    0b00000000_00000000_00000000_00001000_00000000_00000000_00000000_00000000: "e5",
    0b00000000_00000000_00000000_00010000_00000000_00000000_00000000_00000000: "d5",
    0b00000000_00000000_00000000_00100000_00000000_00000000_00000000_00000000: "c5",
    0b00000000_00000000_00000000_01000000_00000000_00000000_00000000_00000000: "b5",
    0b00000000_00000000_00000000_10000000_00000000_00000000_00000000_00000000: "a5",
    0b00000000_00000000_00000001_00000000_00000000_00000000_00000000_00000000: "h6",
    0b00000000_00000000_00000010_00000000_00000000_00000000_00000000_00000000: "g6",
    0b00000000_00000000_00000100_00000000_00000000_00000000_00000000_00000000: "f6",
    0b00000000_00000000_00001000_00000000_00000000_00000000_00000000_00000000: "e6",
    0b00000000_00000000_00010000_00000000_00000000_00000000_00000000_00000000: "d6",
    0b00000000_00000000_00100000_00000000_00000000_00000000_00000000_00000000: "c6",
    0b00000000_00000000_01000000_00000000_00000000_00000000_00000000_00000000: "b6",
    0b00000000_00000000_10000000_00000000_00000000_00000000_00000000_00000000: "a6",
    0b00000000_00000001_00000000_00000000_00000000_00000000_00000000_00000000: "h7",
    0b00000000_00000010_00000000_00000000_00000000_00000000_00000000_00000000: "g7",
    0b00000000_00000100_00000000_00000000_00000000_00000000_00000000_00000000: "f7",
    0b00000000_00001000_00000000_00000000_00000000_00000000_00000000_00000000: "e7",
    0b00000000_00010000_00000000_00000000_00000000_00000000_00000000_00000000: "d7",
    0b00000000_00100000_00000000_00000000_00000000_00000000_00000000_00000000: "c7",
    0b00000000_01000000_00000000_00000000_00000000_00000000_00000000_00000000: "b7",
    0b00000000_10000000_00000000_00000000_00000000_00000000_00000000_00000000: "a7",
    0b00000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000: "h8",
    0b00000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000: "g8",
    0b00000100_00000000_00000000_00000000_00000000_00000000_00000000_00000000: "f8",
    0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000: "e8",
    0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000: "d8",
    0b00100000_00000000_00000000_00000000_00000000_00000000_00000000_00000000: "c8",
    0b01000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000: "b8",
    0b10000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000: "a8",
}


class Move:
    player: str = None
    piece: str = None
    from_square: int = 0
    to_square: int = 0
    en_passant_square: int = 0
    is_capturing_en_passant: bool = None
    is_castling: Optional[str] = None
    is_promoting_to: Optional[str] = None

    def __init__(
        self,
        player: str,
        piece: str,
        from_square: int,
        to_square: int,
        en_passant_square: int,
        is_capturing_en_passant: bool,
        is_castling: Optional[str],
        is_promoting_to: Optional[str],
    ):
        self.player = player
        self.piece = piece
        self.from_square = from_square
        self.to_square = to_square
        self.en_passant_square = en_passant_square
        self.is_capturing_en_passant = is_capturing_en_passant
        self.is_castling = is_castling
        self.is_promoting_to = is_promoting_to

    def id(self) -> str:
        return "-".join(
            [
                str(self.from_square),
                str(self.to_square),
                self.is_castling or "",
                self.is_promoting_to or "",
            ]
        )

    def __str__(self) -> str:
        return (
            map_square_to_human_notation[self.from_square]
            + map_square_to_human_notation[self.to_square]
        )


class Position:
    all_pieces: int = None

    white_pieces: int = None
    black_pieces: int = None

    pieces: dict[str, dict[str, int]]

    def __init__(
        self,
        K: int,
        Q: int,
        R: int,
        B: int,
        N: int,
        P: int,
        k: int,
        q: int,
        r: int,
        b: int,
        n: int,
        p: int,
    ):
        self.white_pieces = K | Q | R | B | N | P
        self.black_pieces = k | q | r | b | n | p

        self.all_pieces = self.white_pieces | self.black_pieces

        self.pieces = {
            Piece["KING"]: {Player["WHITE"]: K, Player["BLACK"]: k},
            Piece["QUEEN"]: {Player["WHITE"]: Q, Player["BLACK"]: q},
            Piece["ROOK"]: {Player["WHITE"]: R, Player["BLACK"]: r},
            Piece["BISHOP"]: {Player["WHITE"]: B, Player["BLACK"]: b},
            Piece["KNIGHT"]: {Player["WHITE"]: N, Player["BLACK"]: n},
            Piece["PAWN"]: {Player["WHITE"]: P, Player["BLACK"]: p},
        }

    def __str__(self) -> str:
        position = ""
        for i in range(64):
            if self.pieces[Piece["KING"]][Player["WHITE"]] & 2 ** i != 0:
                position = "K" + position
            elif self.pieces[Piece["QUEEN"]][Player["WHITE"]] & 2 ** i != 0:
                position = "Q" + position
            elif self.pieces[Piece["ROOK"]][Player["WHITE"]] & 2 ** i != 0:
                position = "R" + position
            elif self.pieces[Piece["BISHOP"]][Player["WHITE"]] & 2 ** i != 0:
                position = "B" + position
            elif self.pieces[Piece["KNIGHT"]][Player["WHITE"]] & 2 ** i != 0:
                position = "N" + position
            elif self.pieces[Piece["PAWN"]][Player["WHITE"]] & 2 ** i != 0:
                position = "P" + position
            elif self.pieces[Piece["KING"]][Player["BLACK"]] & 2 ** i != 0:
                position = "k" + position
            elif self.pieces[Piece["QUEEN"]][Player["BLACK"]] & 2 ** i != 0:
                position = "q" + position
            elif self.pieces[Piece["ROOK"]][Player["BLACK"]] & 2 ** i != 0:
                position = "r" + position
            elif self.pieces[Piece["BISHOP"]][Player["BLACK"]] & 2 ** i != 0:
                position = "b" + position
            elif self.pieces[Piece["KNIGHT"]][Player["BLACK"]] & 2 ** i != 0:
                position = "n" + position
            elif self.pieces[Piece["PAWN"]][Player["BLACK"]] & 2 ** i != 0:
                position = "p" + position
            else:
                position = " " + position
            if i % 8 == 7 and i != 63:
                position = "\n" + position
        return position

    def move(self, move: Move) -> Tuple[Position, Optional[str]]:
        new_position = Position(
            K=self.pieces[Piece["KING"]][Player["WHITE"]],
            Q=self.pieces[Piece["QUEEN"]][Player["WHITE"]],
            R=self.pieces[Piece["ROOK"]][Player["WHITE"]],
            B=self.pieces[Piece["BISHOP"]][Player["WHITE"]],
            N=self.pieces[Piece["KNIGHT"]][Player["WHITE"]],
            P=self.pieces[Piece["PAWN"]][Player["WHITE"]],
            k=self.pieces[Piece["KING"]][Player["BLACK"]],
            q=self.pieces[Piece["QUEEN"]][Player["BLACK"]],
            r=self.pieces[Piece["ROOK"]][Player["BLACK"]],
            b=self.pieces[Piece["BISHOP"]][Player["BLACK"]],
            n=self.pieces[Piece["KNIGHT"]][Player["BLACK"]],
            p=self.pieces[Piece["PAWN"]][Player["BLACK"]],
        )
        if move.is_castling == Castle["WHITE_KINGSIDE"]:
            new_position.pieces[Piece["KING"]][Player["WHITE"]] = 0x0000_0000_0000_0002
            new_position.pieces[Piece["ROOK"]][Player["WHITE"]] ^= 0x0000_0000_0000_0005
            new_position.white_pieces ^= 0x0000_0000_0000_000F
            new_position.all_pieces ^= 0x0000_0000_0000_000F
            return new_position, None
        if move.is_castling == Castle["WHITE_QUEENSIDE"]:
            new_position.pieces[Piece["KING"]][Player["WHITE"]] = 0x0000_0000_0000_0020
            new_position.pieces[Piece["ROOK"]][Player["WHITE"]] ^= 0x0000_0000_0000_0090
            new_position.white_pieces ^= 0x0000_0000_0000_00B8
            new_position.all_pieces ^= 0x0000_0000_0000_00B8
            return new_position, None
        if move.is_castling == Castle["BLACK_KINGSIDE"]:
            new_position.pieces[Piece["KING"]][Player["BLACK"]] = 0x0200_0000_0000_0000
            new_position.pieces[Piece["ROOK"]][Player["BLACK"]] ^= 0x0500_0000_0000_0000
            new_position.black_pieces ^= 0x0F00_0000_0000_0000
            new_position.all_pieces ^= 0x0F00_0000_0000_0000
            return new_position, None
        if move.is_castling == Castle["BLACK_QUEENSIDE"]:
            new_position.pieces[Piece["KING"]][Player["BLACK"]] = 0x2000_0000_0000_0000
            new_position.pieces[Piece["ROOK"]][Player["BLACK"]] ^= 0x9000_0000_0000_0000
            new_position.black_pieces ^= 0xB800_0000_0000_0000
            new_position.all_pieces ^= 0xB800_0000_0000_0000
            return new_position, None

        is_white = move.player == Player["WHITE"]

        is_capturing = None
        if (move.to_square & new_position.pieces[Piece["PAWN"]][Player["WHITE"]]) != 0:
            is_capturing = Piece["PAWN"]
        elif (
            move.to_square & new_position.pieces[Piece["PAWN"]][Player["BLACK"]]
        ) != 0:
            is_capturing = Piece["PAWN"]
        elif (
            move.to_square & new_position.pieces[Piece["KNIGHT"]][Player["WHITE"]]
        ) != 0:
            is_capturing = Piece["KNIGHT"]
        elif (
            move.to_square & new_position.pieces[Piece["KNIGHT"]][Player["BLACK"]]
        ) != 0:
            is_capturing = Piece["KNIGHT"]
        elif (
            move.to_square & new_position.pieces[Piece["BISHOP"]][Player["WHITE"]]
        ) != 0:
            is_capturing = Piece["BISHOP"]
        elif (
            move.to_square & new_position.pieces[Piece["BISHOP"]][Player["BLACK"]]
        ) != 0:
            is_capturing = Piece["BISHOP"]
        elif (
            move.to_square & new_position.pieces[Piece["ROOK"]][Player["WHITE"]]
        ) != 0:
            is_capturing = Piece["ROOK"]
        elif (
            move.to_square & new_position.pieces[Piece["ROOK"]][Player["BLACK"]]
        ) != 0:
            is_capturing = Piece["ROOK"]
        elif (
            move.to_square & new_position.pieces[Piece["QUEEN"]][Player["WHITE"]]
        ) != 0:
            is_capturing = Piece["QUEEN"]
        elif (
            move.to_square & new_position.pieces[Piece["QUEEN"]][Player["BLACK"]]
        ) != 0:
            is_capturing = Piece["QUEEN"]

        new_position.pieces[move.piece][move.player] = (
            new_position.pieces[move.piece][move.player] ^ move.from_square
        ) | move.to_square
        if is_white:
            new_position.white_pieces = (
                new_position.white_pieces ^ move.from_square
            ) | move.to_square
        else:
            new_position.black_pieces = (
                new_position.black_pieces ^ move.from_square
            ) | move.to_square
        new_position.all_pieces = (
            new_position.all_pieces ^ move.from_square
        ) | move.to_square

        opposite_player = Player["BLACK"] if is_white else Player["WHITE"]
        if is_capturing != None:
            new_position.pieces[is_capturing][opposite_player] ^= move.to_square
            if is_white:
                new_position.black_pieces ^= move.to_square
            else:
                new_position.white_pieces ^= move.to_square

        if move.is_capturing_en_passant:
            captured_square = (
                get_bottom_square(move.to_square)
                if is_white
                else get_top_square(move.to_square)
            )
            new_position.pieces[Piece["PAWN"]][opposite_player] ^= captured_square
            if is_white:
                new_position.black_pieces ^= captured_square
            else:
                new_position.white_pieces ^= captured_square
            new_position.all_pieces ^= captured_square

        if move.is_promoting_to:
            new_position.pieces[move.is_promoting_to][move.player] |= move.to_square
            new_position.pieces[Piece["PAWN"]][move.player] ^= move.to_square

        return new_position, is_capturing

    def attackers(self, player: str, square: int) -> int:
        king = self.pieces[Piece["KING"]][player]
        queen = self.pieces[Piece["QUEEN"]][player]
        rook = self.pieces[Piece["ROOK"]][player]
        bishop = self.pieces[Piece["BISHOP"]][player]
        knight = self.pieces[Piece["KNIGHT"]][player]
        pawn = self.pieces[Piece["PAWN"]][player]

        queen_and_rook = queen | rook
        queen_and_bishop = queen | bishop

        north_pieces = NORTH_RAY[square] & self.all_pieces
        south_pieces = SOUTH_RAY[square] & self.all_pieces
        west_pieces = WEST_RAY[square] & self.all_pieces
        east_pieces = EAST_RAY[square] & self.all_pieces
        north_west_pieces = NORTH_WEST_RAY[square] & self.all_pieces
        south_west_pieces = SOUTH_WEST_RAY[square] & self.all_pieces
        north_east_pieces = NORTH_EAST_RAY[square] & self.all_pieces
        south_east_pieces = SOUTH_EAST_RAY[square] & self.all_pieces

        return (
            (KING_MOVES[square] & king)
            | (NORTH_ATTACKS[square][north_pieces] & queen_and_rook)
            | (SOUTH_ATTACKS[square][south_pieces] & queen_and_rook)
            | (WEST_ATTACKS[square][west_pieces] & queen_and_rook)
            | (EAST_ATTACKS[square][east_pieces] & queen_and_rook)
            | (NORTH_WEST_ATTACKS[square][north_west_pieces] & queen_and_bishop)
            | (SOUTH_WEST_ATTACKS[square][south_west_pieces] & queen_and_bishop)
            | (NORTH_EAST_ATTACKS[square][north_east_pieces] & queen_and_bishop)
            | (SOUTH_EAST_ATTACKS[square][south_east_pieces] & queen_and_bishop)
            | (KNIGHT_MOVES[square] & knight)
            | (PAWN_ATTACKS[player][square] & pawn)
        )

    def checkers(self, player: str, king: int) -> Iterable[int]:
        queen = self.pieces[Piece["QUEEN"]][player]
        rook = self.pieces[Piece["ROOK"]][player]
        bishop = self.pieces[Piece["BISHOP"]][player]
        knight = self.pieces[Piece["KNIGHT"]][player]
        pawn = self.pieces[Piece["PAWN"]][player]

        queen_and_rook = queen | rook
        queen_and_bishop = queen | bishop

        north_pieces = NORTH_RAY[king] & self.all_pieces
        south_pieces = SOUTH_RAY[king] & self.all_pieces
        west_pieces = WEST_RAY[king] & self.all_pieces
        east_pieces = EAST_RAY[king] & self.all_pieces
        north_west_pieces = NORTH_WEST_RAY[king] & self.all_pieces
        south_west_pieces = SOUTH_WEST_RAY[king] & self.all_pieces
        north_east_pieces = NORTH_EAST_RAY[king] & self.all_pieces
        south_east_pieces = SOUTH_EAST_RAY[king] & self.all_pieces

        return split(
            (NORTH_ATTACKS[king][north_pieces] & queen_and_rook)
            | (SOUTH_ATTACKS[king][south_pieces] & queen_and_rook)
            | (WEST_ATTACKS[king][west_pieces] & queen_and_rook)
            | (EAST_ATTACKS[king][east_pieces] & queen_and_rook)
            | (NORTH_WEST_ATTACKS[king][north_west_pieces] & queen_and_bishop)
            | (SOUTH_WEST_ATTACKS[king][south_west_pieces] & queen_and_bishop)
            | (NORTH_EAST_ATTACKS[king][north_east_pieces] & queen_and_bishop)
            | (SOUTH_EAST_ATTACKS[king][south_east_pieces] & queen_and_bishop)
            | (KNIGHT_MOVES[king] & knight)
            | (PAWN_ATTACKS[player][king] & pawn)
        )

    def attacked_squares(self, player: str, exclude_king: bool = False) -> int:
        all_pieces = self.all_pieces
        if exclude_king:
            opposite_player = (
                Player["BLACK"] if player == Player["WHITE"] else Player["WHITE"]
            )
            all_pieces ^= (
                self.pieces[Piece["KING"]][opposite_player] if exclude_king else 0
            )

        attacked = KING_MOVES[self.pieces[Piece["KING"]][player]]

        for queen in split(self.pieces[Piece["QUEEN"]][player]):
            north_pieces = NORTH_RAY[queen] & all_pieces
            attacked |= (
                NORTH_MOVES[queen][north_pieces] | NORTH_ATTACKS[queen][north_pieces]
            )
            south_pieces = SOUTH_RAY[queen] & all_pieces
            attacked |= (
                SOUTH_MOVES[queen][south_pieces] | SOUTH_ATTACKS[queen][south_pieces]
            )
            west_pieces = WEST_RAY[queen] & all_pieces
            attacked |= (
                WEST_MOVES[queen][west_pieces] | WEST_ATTACKS[queen][west_pieces]
            )
            east_pieces = EAST_RAY[queen] & all_pieces
            attacked |= (
                EAST_MOVES[queen][east_pieces] | EAST_ATTACKS[queen][east_pieces]
            )
            north_west_pieces = NORTH_WEST_RAY[queen] & all_pieces
            attacked |= (
                NORTH_WEST_MOVES[queen][north_west_pieces]
                | NORTH_WEST_ATTACKS[queen][north_west_pieces]
            )
            north_east_pieces = NORTH_EAST_RAY[queen] & all_pieces
            attacked |= (
                NORTH_EAST_MOVES[queen][north_east_pieces]
                | NORTH_EAST_ATTACKS[queen][north_east_pieces]
            )
            south_west_pieces = SOUTH_WEST_RAY[queen] & all_pieces
            attacked |= (
                SOUTH_WEST_MOVES[queen][south_west_pieces]
                | SOUTH_WEST_ATTACKS[queen][south_west_pieces]
            )
            south_east_pieces = SOUTH_EAST_RAY[queen] & all_pieces
            attacked |= (
                SOUTH_EAST_MOVES[queen][south_east_pieces]
                | SOUTH_EAST_ATTACKS[queen][south_east_pieces]
            )

        for rook in split(self.pieces[Piece["ROOK"]][player]):
            north_pieces = NORTH_RAY[rook] & all_pieces
            attacked |= (
                NORTH_MOVES[rook][north_pieces] | NORTH_ATTACKS[rook][north_pieces]
            )
            south_pieces = SOUTH_RAY[rook] & all_pieces
            attacked |= (
                SOUTH_MOVES[rook][south_pieces] | SOUTH_ATTACKS[rook][south_pieces]
            )
            west_pieces = WEST_RAY[rook] & all_pieces
            attacked |= WEST_MOVES[rook][west_pieces] | WEST_ATTACKS[rook][west_pieces]
            east_pieces = EAST_RAY[rook] & all_pieces
            attacked |= EAST_MOVES[rook][east_pieces] | EAST_ATTACKS[rook][east_pieces]

        for bishop in split(self.pieces[Piece["BISHOP"]][player]):
            north_west_pieces = NORTH_WEST_RAY[bishop] & all_pieces
            attacked |= (
                NORTH_WEST_MOVES[bishop][north_west_pieces]
                | NORTH_WEST_ATTACKS[bishop][north_west_pieces]
            )
            north_east_pieces = NORTH_EAST_RAY[bishop] & all_pieces
            attacked |= (
                NORTH_EAST_MOVES[bishop][north_east_pieces]
                | NORTH_EAST_ATTACKS[bishop][north_east_pieces]
            )
            south_west_pieces = SOUTH_WEST_RAY[bishop] & all_pieces
            attacked |= (
                SOUTH_WEST_MOVES[bishop][south_west_pieces]
                | SOUTH_WEST_ATTACKS[bishop][south_west_pieces]
            )
            south_east_pieces = SOUTH_EAST_RAY[bishop] & all_pieces
            attacked |= (
                SOUTH_EAST_MOVES[bishop][south_east_pieces]
                | SOUTH_EAST_ATTACKS[bishop][south_east_pieces]
            )

        for knight in split(self.pieces[Piece["KNIGHT"]][player]):
            attacked |= KNIGHT_MOVES[knight]

        for pawn in split(self.pieces[Piece["PAWN"]][player]):
            for s in PAWN_ATTACK_MOVES[player][pawn]:
                attacked |= s
            for s in PAWN_ATTACK_MOVES_PROMOTION[player][pawn]:
                attacked |= s

        return attacked

    def is_check(self, player: str):
        attackers = self.attackers(
            Player["BLACK"] if player == Player["WHITE"] else Player["WHITE"],
            self.pieces[Piece["KING"]][player],
        )
        return attackers != 0

    def pinned_movement(
        self,
        square: int,
        king: int,
        enemy_queens_and_rooks: int,
        enemy_queens_and_bishops: int,
    ) -> int:
        north_pieces = NORTH_RAY[square] & self.all_pieces
        south_pieces = SOUTH_RAY[square] & self.all_pieces
        first_piece_to_north = NORTH_ATTACKS[square][north_pieces]
        first_piece_to_south = SOUTH_ATTACKS[square][south_pieces]

        is_pinned_from_north = first_piece_to_south == king and (
            first_piece_to_north & enemy_queens_and_rooks != 0
        )
        if is_pinned_from_north:
            return (
                first_piece_to_north
                | NORTH_MOVES[square][north_pieces]
                | SOUTH_MOVES[square][south_pieces]
            )

        is_pinned_from_south = first_piece_to_north == king and (
            first_piece_to_south & enemy_queens_and_rooks != 0
        )
        if is_pinned_from_south:
            return (
                first_piece_to_south
                | SOUTH_MOVES[square][south_pieces]
                | NORTH_MOVES[square][north_pieces]
            )

        west_pieces = WEST_RAY[square] & self.all_pieces
        east_pieces = EAST_RAY[square] & self.all_pieces
        first_piece_to_west = WEST_ATTACKS[square][west_pieces]
        first_piece_to_east = EAST_ATTACKS[square][east_pieces]

        is_pinned_from_west = first_piece_to_east == king and (
            first_piece_to_west & enemy_queens_and_rooks != 0
        )
        if is_pinned_from_west:
            return (
                first_piece_to_west
                | WEST_MOVES[square][west_pieces]
                | EAST_MOVES[square][east_pieces]
            )

        is_pinned_from_east = first_piece_to_west == king and (
            first_piece_to_east & enemy_queens_and_rooks != 0
        )
        if is_pinned_from_east:
            return (
                first_piece_to_east
                | EAST_MOVES[square][east_pieces]
                | WEST_MOVES[square][west_pieces]
            )

        north_west_pieces = NORTH_WEST_RAY[square] & self.all_pieces
        south_east_pieces = SOUTH_EAST_RAY[square] & self.all_pieces
        first_piece_to_north_west = NORTH_WEST_ATTACKS[square][north_west_pieces]
        first_piece_to_south_east = SOUTH_EAST_ATTACKS[square][south_east_pieces]

        is_pinned_from_north_west = first_piece_to_south_east == king and (
            first_piece_to_north_west & enemy_queens_and_bishops != 0
        )
        if is_pinned_from_north_west:
            return (
                first_piece_to_north_west
                | NORTH_WEST_MOVES[square][north_west_pieces]
                | SOUTH_EAST_MOVES[square][south_east_pieces]
            )

        is_pinned_from_south_east = first_piece_to_north_west == king and (
            first_piece_to_south_east & enemy_queens_and_bishops != 0
        )
        if is_pinned_from_south_east:
            return (
                first_piece_to_south_east
                | SOUTH_EAST_MOVES[square][south_east_pieces]
                | NORTH_WEST_MOVES[square][north_west_pieces]
            )

        north_east_pieces = NORTH_EAST_RAY[square] & self.all_pieces
        south_west_pieces = SOUTH_WEST_RAY[square] & self.all_pieces
        first_piece_to_north_east = NORTH_EAST_ATTACKS[square][north_east_pieces]
        first_piece_to_south_west = SOUTH_WEST_ATTACKS[square][south_west_pieces]

        is_pinned_from_north_east = first_piece_to_south_west == king and (
            first_piece_to_north_east & enemy_queens_and_bishops != 0
        )
        if is_pinned_from_north_east:
            return (
                first_piece_to_north_east
                | NORTH_EAST_MOVES[square][north_east_pieces]
                | SOUTH_WEST_MOVES[square][south_west_pieces]
            )

        is_pinned_from_south_west = first_piece_to_north_east == king and (
            first_piece_to_south_west & enemy_queens_and_bishops != 0
        )
        if is_pinned_from_south_west:
            return (
                first_piece_to_south_west
                | SOUTH_WEST_MOVES[square][south_west_pieces]
                | NORTH_EAST_MOVES[square][north_east_pieces]
            )

        return 0xFFFF_FFFF_FFFF_FFFF

    def is_dead(self) -> bool:
        white_queens = list(split(self.pieces[Piece["QUEEN"]][Player["WHITE"]]))
        white_rooks = list(split(self.pieces[Piece["ROOK"]][Player["WHITE"]]))
        white_bishops = list(split(self.pieces[Piece["BISHOP"]][Player["WHITE"]]))
        white_knights = list(split(self.pieces[Piece["KNIGHT"]][Player["WHITE"]]))
        white_pawns = list(split(self.pieces[Piece["PAWN"]][Player["WHITE"]]))
        black_queens = list(split(self.pieces[Piece["QUEEN"]][Player["BLACK"]]))
        black_rooks = list(split(self.pieces[Piece["ROOK"]][Player["BLACK"]]))
        black_bishops = list(split(self.pieces[Piece["BISHOP"]][Player["BLACK"]]))
        black_knights = list(split(self.pieces[Piece["KNIGHT"]][Player["BLACK"]]))
        black_pawns = list(split(self.pieces[Piece["PAWN"]][Player["BLACK"]]))

        number_of_white_pieces = (
            len(white_queens)
            + len(white_rooks)
            + len(white_bishops)
            + len(white_knights)
            + len(white_pawns)
        )
        number_of_black_pieces = (
            len(black_queens)
            + len(black_rooks)
            + len(black_bishops)
            + len(black_knights)
            + len(black_pawns)
        )

        # king against king
        if number_of_white_pieces + number_of_black_pieces == 0:
            return True

        # king against king and bishop
        if (
            number_of_white_pieces == 0
            and number_of_black_pieces == 1
            and black_bishops.length == 1
        ):
            return True
        if (
            number_of_black_pieces == 0
            and number_of_white_pieces == 1
            and white_bishops.length == 1
        ):
            return True

        # king against king and knight
        if (
            number_of_white_pieces == 0
            and number_of_black_pieces == 1
            and black_knights.length == 1
        ):
            return True
        if (
            number_of_black_pieces == 0
            and number_of_white_pieces == 1
            and white_knights.length == 1
        ):
            return True

        # king and bishop against king and bishop, with both bishops on squares of the same color
        if (
            number_of_white_pieces == 1
            and number_of_black_pieces == 1
            and white_bishops.length == 1
            and black_bishops.length == 1
        ):
            is_white_bishop_on_white_square = (
                white_bishops[0] & 0xAA55_AA55_AA55_AA55
            ) == 0
            is_black_bishop_on_white_square = (
                black_bishops[0] & 0xAA55_AA55_AA55_AA55
            ) == 0
            return is_white_bishop_on_white_square == is_black_bishop_on_white_square

        return False


class Game:
    position: Position = None
    player: str = None
    last_move: Optional[Move] = None
    possible_castles: dict[str, bool] = None
    en_passant_square: int = None
    position_counts: dict[str, int] = None
    move_counter: int = None
    fifty_move_counter: int = None

    def __init__(
        self,
        position: Position,
        player: str,
        last_move: Optional[Move],
        possible_castles: dict[str, bool],
        en_passant_square: int,
        position_counts: dict[str, int],
        move_counter: int,
        fifty_move_counter: int,
    ):
        self.position = position
        self.player = player
        self.last_move = last_move
        self.possible_castles = possible_castles
        self.en_passant_square = en_passant_square
        self.position_counts = position_counts
        self.move_counter = move_counter
        self.fifty_move_counter = fifty_move_counter

    def to_string(self) -> str:
        return "%d-%d-%d-%d-%d-%d-%d-%d-%d-%d-%d-%d-%s-%s%s%s%s-%d" % (
            self.position.pieces[Piece["KING"]][Player["WHITE"]],
            self.position.pieces[Piece["QUEEN"]][Player["WHITE"]],
            self.position.pieces[Piece["ROOK"]][Player["WHITE"]],
            self.position.pieces[Piece["BISHOP"]][Player["WHITE"]],
            self.position.pieces[Piece["KNIGHT"]][Player["WHITE"]],
            self.position.pieces[Piece["PAWN"]][Player["WHITE"]],
            self.position.pieces[Piece["KING"]][Player["BLACK"]],
            self.position.pieces[Piece["QUEEN"]][Player["BLACK"]],
            self.position.pieces[Piece["ROOK"]][Player["BLACK"]],
            self.position.pieces[Piece["BISHOP"]][Player["BLACK"]],
            self.position.pieces[Piece["KNIGHT"]][Player["BLACK"]],
            self.position.pieces[Piece["PAWN"]][Player["BLACK"]],
            self.player,
            Castle["WHITE_KINGSIDE"]
            if self.possible_castles.get(Castle["WHITE_KINGSIDE"])
            else "",
            Castle["WHITE_QUEENSIDE"]
            if self.possible_castles.get(Castle["WHITE_QUEENSIDE"])
            else "",
            Castle["BLACK_KINGSIDE"]
            if self.possible_castles.get(Castle["BLACK_KINGSIDE"])
            else "",
            Castle["BLACK_QUEENSIDE"]
            if self.possible_castles.get(Castle["BLACK_QUEENSIDE"])
            else "",
            self.en_passant_square,
        )

    def move(self, move: Move) -> Game:
        is_white = self.player == Player["WHITE"]

        new_position, is_capturing = self.position.move(move)

        if (
            is_capturing != None
            or move.is_promoting_to != None
            or move.is_castling != None
        ):
            new_position_counts = {}
        else:
            new_position_counts = self.position_counts.copy()
            key = self.to_string()
            new_position_counts[key] = (new_position_counts.get(key) or 0) + 1

        return Game(
            position=new_position,
            player=Player["BLACK"] if is_white else Player["WHITE"],
            last_move=move,
            possible_castles={
                Castle["WHITE_KINGSIDE"]: self.possible_castles[
                    Castle["WHITE_KINGSIDE"]
                ]
                and not (is_white and move.piece == Piece["KING"])
                and not (
                    is_white
                    and move.piece == Piece["ROOK"]
                    and move.from_square == 0x0000_0000_0000_0001
                )
                and not (
                    not is_white
                    and is_capturing == Piece["ROOK"]
                    and move.to_square == 0x0000_0000_0000_0001
                ),
                Castle["WHITE_QUEENSIDE"]: self.possible_castles[
                    Castle["WHITE_QUEENSIDE"]
                ]
                and not (is_white and move.piece == Piece["KING"])
                and not (
                    is_white
                    and move.piece == Piece["ROOK"]
                    and move.from_square == 0x0000_0000_0000_0080
                )
                and not (
                    not is_white
                    and is_capturing == Piece["ROOK"]
                    and move.to_square == 0x0000_0000_0000_0080
                ),
                Castle["BLACK_KINGSIDE"]: self.possible_castles[
                    Castle["BLACK_KINGSIDE"]
                ]
                and not (not is_white and move.piece == Piece["KING"])
                and not (
                    not is_white
                    and move.piece == Piece["ROOK"]
                    and move.from_square == 0x0100_0000_0000_0000
                )
                and not (
                    is_white
                    and is_capturing == Piece["ROOK"]
                    and move.to_square == 0x0100_0000_0000_0000
                ),
                Castle["BLACK_QUEENSIDE"]: self.possible_castles[
                    Castle["BLACK_QUEENSIDE"]
                ]
                and not (not is_white and move.piece == Piece["KING"])
                and not (
                    not is_white
                    and move.piece == Piece["ROOK"]
                    and move.from_square == 0x8000_0000_0000_0000
                )
                and not (
                    is_white
                    and is_capturing == Piece["ROOK"]
                    and move.to_square == 0x8000_0000_0000_0000
                ),
            },
            en_passant_square=move.en_passant_square,
            position_counts=new_position_counts,
            move_counter=self.move_counter + (0 if is_white else 1),
            fifty_move_counter=0
            if move.piece == Piece["PAWN"]
            or is_capturing
            or move.is_capturing_en_passant
            else self.fifty_move_counter + 1,
        )

    def __str__(self) -> str:
        return f"""
Game(
    player={self.player},
    position=(
--------
{self.position}
--------
    ),
    possible_castles=(
        K={self.possible_castles.get(Castle["WHITE_KINGSIDE"])},
        Q={self.possible_castles.get(Castle["WHITE_QUEENSIDE"])},
        k={self.possible_castles.get(Castle["BLACK_KINGSIDE"])},
        q={self.possible_castles.get(Castle["BLACK_QUEENSIDE"])},
    ),
    en_passant_square={self.en_passant_square}
)
"""
