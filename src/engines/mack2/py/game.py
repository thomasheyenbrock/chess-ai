from enum import Enum
from typing import Optional


class Player(Enum):
    WHITE = "w"
    BLACK = "b"


class Piece(Enum):
    WHITE_KING = "K"
    WHITE_QUEEN = "Q"
    WHITE_ROOK = "R"
    WHITE_BISHOP = "B"
    WHITE_KNIGHT = "N"
    WHITE_PAWN = "P"
    BLACK_KING = "k"
    BLACK_QUEEN = "q"
    BLACK_ROOK = "r"
    BLACK_BISHOP = "b"
    BLACK_KNIGHT = "n"
    BLACK_PAWN = "p"


class PromotionPiece(Enum):
    WHITE_QUEEN = "Q"
    WHITE_ROOK = "R"
    WHITE_BISHOP = "B"
    WHITE_KNIGHT = "N"
    BLACK_QUEEN = "q"
    BLACK_ROOK = "r"
    BLACK_BISHOP = "b"
    BLACK_KNIGHT = "n"


class Castle(Enum):
    WHITE_KINGSIDE = "K"
    WHITE_QUEENSIDE = "Q"
    BLACK_KINGSIDE = "k"
    BLACK_QUEENSIDE = "q"


class Position:
    K: int = None
    Q: int = None
    R: int = None
    B: int = None
    N: int = None
    P: int = None
    k: int = None
    q: int = None
    r: int = None
    b: int = None
    n: int = None
    p: int = None

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
        self.K = K
        self.Q = Q
        self.R = R
        self.B = B
        self.N = N
        self.P = P
        self.k = k
        self.q = q
        self.r = r
        self.b = b
        self.n = n
        self.p = p

    def __str__(self) -> str:
        position = ""
        for i in range(64):
            if self.K & 2 ** i != 0:
                position = "K" + position
            elif self.Q & 2 ** i != 0:
                position = "Q" + position
            elif self.R & 2 ** i != 0:
                position = "R" + position
            elif self.B & 2 ** i != 0:
                position = "B" + position
            elif self.N & 2 ** i != 0:
                position = "N" + position
            elif self.P & 2 ** i != 0:
                position = "P" + position
            elif self.k & 2 ** i != 0:
                position = "k" + position
            elif self.q & 2 ** i != 0:
                position = "q" + position
            elif self.r & 2 ** i != 0:
                position = "r" + position
            elif self.b & 2 ** i != 0:
                position = "b" + position
            elif self.n & 2 ** i != 0:
                position = "n" + position
            elif self.p & 2 ** i != 0:
                position = "p" + position
            else:
                position = " " + position
            if i % 8 == 7 and i != 63:
                position = "\n" + position
        return position


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
    player: Player = None
    piece: Piece = None
    from_square: int = 0
    to_square: int = 0
    is_capturing: Optional[Piece] = None
    is_castling: Optional[Castle] = None
    is_promoting_to: Optional[PromotionPiece] = None

    def __init__(
        self,
        player: Player,
        piece: Piece,
        from_square: int,
        to_square: int,
        is_capturing: Optional[Piece],
        is_castling: Optional[Castle],
        is_promoting_to: Optional[PromotionPiece],
    ):
        self.player = player
        self.piece = piece
        self.from_square = from_square
        self.to_square = to_square
        self.is_capturing = is_capturing
        self.is_castling = is_castling
        self.is_promoting_to = is_promoting_to

    def id(self) -> str:
        return "-".join(
            [
                str(self.from_square),
                str(self.to_square),
                self.is_castling.value if self.is_castling != None else "",
                self.is_promoting_to.value if self.is_promoting_to != None else "",
            ]
        )

    def __str__(self) -> str:
        return (
            self.piece.value
            + map_square_to_human_notation[self.from_square]
            + map_square_to_human_notation[self.to_square]
        )


class Game:
    position: Position = None
    player: Player = None
    last_move: Optional[Move] = None
    possible_castles: dict[Castle, bool] = None
    en_passant_square: int = None
    position_counts: dict[str, int] = None
    move_counter: int = None
    fifty_move_counter: int = None

    def __init__(
        self,
        position: Position,
        player: Player,
        last_move: Optional[Move],
        possible_castles: dict[Castle, bool],
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
            self.position.K,
            self.position.Q,
            self.position.R,
            self.position.B,
            self.position.N,
            self.position.P,
            self.position.k,
            self.position.q,
            self.position.r,
            self.position.b,
            self.position.n,
            self.position.p,
            self.player,
            Castle.WHITE_KINGSIDE.value
            if self.possible_castles.get(Castle.WHITE_KINGSIDE)
            else "",
            Castle.WHITE_QUEENSIDE.value
            if self.possible_castles.get(Castle.WHITE_QUEENSIDE)
            else "",
            Castle.BLACK_KINGSIDE.value
            if self.possible_castles.get(Castle.BLACK_KINGSIDE)
            else "",
            Castle.BLACK_QUEENSIDE.value
            if self.possible_castles.get(Castle.BLACK_QUEENSIDE)
            else "",
            self.en_passant_square,
        )

    def increment_position_count(self):
        if self.last_move != None and (
            self.last_move.is_capturing != None
            or self.last_move.is_promoting_to != None
            or self.last_move.is_castling != None
        ):
            self.position_counts = {}
        else:
            key = self.to_string()
            self.position_counts[key] = (self.position_counts.get(key) or 0) + 1

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
        K={self.possible_castles.get(Castle.WHITE_KINGSIDE)},
        Q={self.possible_castles.get(Castle.WHITE_QUEENSIDE)},
        k={self.possible_castles.get(Castle.BLACK_KINGSIDE)},
        q={self.possible_castles.get(Castle.BLACK_QUEENSIDE)},
    ),
    en_passant_square={self.en_passant_square}
)
"""
