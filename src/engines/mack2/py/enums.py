from enum import Enum


class Player(Enum):
    WHITE = "w"
    BLACK = "b"


class Piece(Enum):
    KING = "K"
    QUEEN = "Q"
    ROOK = "R"
    BISHOP = "B"
    KNIGHT = "N"
    PAWN = "P"


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


class Result(Enum):
    WHITE = "White wins"
    BLACK = "Black wins"
    STALEMATE = "Stalemate"
    DEAD_POSITION = "Dead position"
    REPITITION = "Third repitition of position"
    FIFTY_MOVE_RULE = "Fifty moves without capture or pawn movement"
