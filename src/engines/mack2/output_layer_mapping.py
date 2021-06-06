from typing import Iterable

from py.bitboard import split
from py.constants import (
    EAST_RAY,
    KNIGHT_MOVES,
    NORTH_EAST_RAY,
    NORTH_RAY,
    NORTH_WEST_RAY,
    SOUTH_EAST_RAY,
    SOUTH_RAY,
    SOUTH_WEST_RAY,
    WEST_RAY,
)
from py.game import Move


def get_queen_moves(from_square: int) -> Iterable[int]:
    moveable_squares = (
        NORTH_RAY[from_square]
        | SOUTH_RAY[from_square]
        | WEST_RAY[from_square]
        | EAST_RAY[from_square]
        | NORTH_WEST_RAY[from_square]
        | NORTH_EAST_RAY[from_square]
        | SOUTH_WEST_RAY[from_square]
        | SOUTH_EAST_RAY[from_square]
    )
    return split(moveable_squares)


white_promotion_moves = [
    (0x0001_0000_0000_0000, 0x0100_0000_0000_0000),
    (0x0001_0000_0000_0000, 0x0200_0000_0000_0000),
    (0x0002_0000_0000_0000, 0x0100_0000_0000_0000),
    (0x0002_0000_0000_0000, 0x0200_0000_0000_0000),
    (0x0002_0000_0000_0000, 0x0400_0000_0000_0000),
    (0x0004_0000_0000_0000, 0x0200_0000_0000_0000),
    (0x0004_0000_0000_0000, 0x0400_0000_0000_0000),
    (0x0004_0000_0000_0000, 0x0800_0000_0000_0000),
    (0x0008_0000_0000_0000, 0x0400_0000_0000_0000),
    (0x0008_0000_0000_0000, 0x0800_0000_0000_0000),
    (0x0008_0000_0000_0000, 0x1000_0000_0000_0000),
    (0x0010_0000_0000_0000, 0x0800_0000_0000_0000),
    (0x0010_0000_0000_0000, 0x1000_0000_0000_0000),
    (0x0010_0000_0000_0000, 0x2000_0000_0000_0000),
    (0x0020_0000_0000_0000, 0x1000_0000_0000_0000),
    (0x0020_0000_0000_0000, 0x2000_0000_0000_0000),
    (0x0020_0000_0000_0000, 0x4000_0000_0000_0000),
    (0x0040_0000_0000_0000, 0x2000_0000_0000_0000),
    (0x0040_0000_0000_0000, 0x4000_0000_0000_0000),
    (0x0040_0000_0000_0000, 0x8000_0000_0000_0000),
    (0x0080_0000_0000_0000, 0x4000_0000_0000_0000),
    (0x0080_0000_0000_0000, 0x8000_0000_0000_0000),
]

black_promotion_moves = [
    (0x0000_0000_0000_0100, 0x0000_0000_0000_0001),
    (0x0000_0000_0000_0100, 0x0000_0000_0000_0002),
    (0x0000_0000_0000_0200, 0x0000_0000_0000_0001),
    (0x0000_0000_0000_0200, 0x0000_0000_0000_0002),
    (0x0000_0000_0000_0200, 0x0000_0000_0000_0004),
    (0x0000_0000_0000_0400, 0x0000_0000_0000_0002),
    (0x0000_0000_0000_0400, 0x0000_0000_0000_0004),
    (0x0000_0000_0000_0400, 0x0000_0000_0000_0008),
    (0x0000_0000_0000_0800, 0x0000_0000_0000_0004),
    (0x0000_0000_0000_0800, 0x0000_0000_0000_0008),
    (0x0000_0000_0000_0800, 0x0000_0000_0000_0010),
    (0x0000_0000_0000_1000, 0x0000_0000_0000_0008),
    (0x0000_0000_0000_1000, 0x0000_0000_0000_0010),
    (0x0000_0000_0000_1000, 0x0000_0000_0000_0020),
    (0x0000_0000_0000_2000, 0x0000_0000_0000_0010),
    (0x0000_0000_0000_2000, 0x0000_0000_0000_0020),
    (0x0000_0000_0000_2000, 0x0000_0000_0000_0040),
    (0x0000_0000_0000_4000, 0x0000_0000_0000_0020),
    (0x0000_0000_0000_4000, 0x0000_0000_0000_0040),
    (0x0000_0000_0000_4000, 0x0000_0000_0000_0080),
    (0x0000_0000_0000_8000, 0x0000_0000_0000_0040),
    (0x0000_0000_0000_8000, 0x0000_0000_0000_0080),
]

move_for_output_index = []

for from_square in split(0xFFFF_FFFF_FFFF_FFFF):
    for to_square in get_queen_moves(from_square):
        move_for_output_index += [
            Move(
                player=True, piece="Q", from_square=from_square, to_square=to_square
            ).id()
        ]

for from_square in split(0xFFFF_FFFF_FFFF_FFFF):
    for to_square in split(KNIGHT_MOVES[from_square]):
        move_for_output_index += [
            Move(
                player=True, piece="N", from_square=from_square, to_square=to_square
            ).id()
        ]

for (from_square, to_square) in white_promotion_moves:
    move_for_output_index += [
        Move(
            player=True,
            piece="P",
            from_square=from_square,
            to_square=to_square,
            is_promoting_to="Q",
        ).id(),
        Move(
            player=True,
            piece="P",
            from_square=from_square,
            to_square=to_square,
            is_promoting_to="R",
        ).id(),
        Move(
            player=True,
            piece="P",
            from_square=from_square,
            to_square=to_square,
            is_promoting_to="B",
        ).id(),
        Move(
            player=True,
            piece="P",
            from_square=from_square,
            to_square=to_square,
            is_promoting_to="K",
        ).id(),
    ]

for (from_square, to_square) in black_promotion_moves:
    move_for_output_index += [
        Move(
            player=False,
            piece="P",
            from_square=from_square,
            to_square=to_square,
            is_promoting_to="Q",
        ).id(),
        Move(
            player=False,
            piece="P",
            from_square=from_square,
            to_square=to_square,
            is_promoting_to="R",
        ).id(),
        Move(
            player=False,
            piece="P",
            from_square=from_square,
            to_square=to_square,
            is_promoting_to="B",
        ).id(),
        Move(
            player=False,
            piece="P",
            from_square=from_square,
            to_square=to_square,
            is_promoting_to="N",
        ).id(),
    ]

move_for_output_index += [
    Move(
        player=True,
        piece="K",
        from_square=0x0000_0000_0000_0008,
        to_square=0x0000_0000_0000_0002,
        is_castling="K",
    ).id(),
    Move(
        player=True,
        piece="K",
        from_square=0x0000_0000_0000_0008,
        to_square=0x0000_0000_0000_0020,
        is_castling="Q",
    ).id(),
    Move(
        player=False,
        piece="K",
        from_square=0x0800_0000_0000_0000,
        to_square=0x0200_0000_0000_0000,
        is_castling="k",
    ).id(),
    Move(
        player=False,
        piece="K",
        from_square=0x0800_0000_0000_0000,
        to_square=0x2000_0000_0000_0000,
        is_castling="q",
    ).id(),
]
