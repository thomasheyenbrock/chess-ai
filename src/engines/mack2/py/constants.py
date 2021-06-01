from typing import Callable, List

from bitboard import (
    get_top_square,
    get_bottom_square,
    get_left_square,
    get_right_square,
)
from enums import Player

NORTH_RAY = {}
SOUTH_RAY = {}
WEST_RAY = {}
EAST_RAY = {}
NORTH_WEST_RAY = {}
SOUTH_WEST_RAY = {}
NORTH_EAST_RAY = {}
SOUTH_EAST_RAY = {}

NORTH_MOVES = {}
SOUTH_MOVES = {}
WEST_MOVES = {}
EAST_MOVES = {}
NORTH_WEST_MOVES = {}
SOUTH_WEST_MOVES = {}
NORTH_EAST_MOVES = {}
SOUTH_EAST_MOVES = {}

NORTH_ATTACKS = {}
SOUTH_ATTACKS = {}
WEST_ATTACKS = {}
EAST_ATTACKS = {}
NORTH_WEST_ATTACKS = {}
SOUTH_WEST_ATTACKS = {}
NORTH_EAST_ATTACKS = {}
SOUTH_EAST_ATTACKS = {}

KING_MOVES = {}
KNIGHT_MOVES = {}
PAWN_ATTACKS = {Player.WHITE: {}, Player.BLACK: {}}


def generate_possibilities(direction: Callable[[int], int]) -> List[int]:
    forward = direction(current)
    possibilities = [0] if forward == 0 else [0, forward]
    while forward != 0:
        forward = direction(forward)
        new_possibilities = []
        for p in possibilities:
            new_possibilities += [p] if forward == 0 else [p, p | forward]
        possibilities = new_possibilities
    return possibilities


for rank in range(8):
    for file in range(8):
        square = 2 ** (8 * rank + file)

        NORTH_RAY[square] = 0x0000_0000_0000_0000
        NORTH_MOVES[square] = {}
        NORTH_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_top_square(square)
        carry = 0
        while current != 0:
            NORTH_RAY[square] |= current
            for p in generate_possibilities(get_top_square):
                NORTH_MOVES[square][p | current] = carry
                NORTH_ATTACKS[square][p | current] = current
            carry |= current
            current = get_top_square(current)
        NORTH_MOVES[square][0] = carry

        SOUTH_RAY[square] = 0x0000_0000_0000_0000
        SOUTH_MOVES[square] = {}
        SOUTH_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_bottom_square(square)
        carry = 0
        while current != 0:
            SOUTH_RAY[square] |= current
            for p in generate_possibilities(get_bottom_square):
                SOUTH_MOVES[square][p | current] = carry
                SOUTH_ATTACKS[square][p | current] = current
            carry |= current
            current = get_bottom_square(current)
        SOUTH_MOVES[square][0] = carry

        WEST_RAY[square] = 0x0000_0000_0000_0000
        WEST_MOVES[square] = {}
        WEST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_left_square(square)
        carry = 0
        while current != 0:
            WEST_RAY[square] |= current
            for p in generate_possibilities(get_left_square):
                WEST_MOVES[square][p | current] = carry
                WEST_ATTACKS[square][p | current] = current
            carry |= current
            current = get_left_square(current)
        WEST_MOVES[square][0] = carry

        EAST_RAY[square] = 0x0000_0000_0000_0000
        EAST_MOVES[square] = {}
        EAST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_right_square(square)
        carry = 0
        while current != 0:
            EAST_RAY[square] |= current
            for p in generate_possibilities(get_right_square):
                EAST_MOVES[square][p | current] = carry
                EAST_ATTACKS[square][p | current] = current
            carry |= current
            current = get_right_square(current)
        EAST_MOVES[square][0] = carry

        NORTH_WEST_RAY[square] = 0x0000_0000_0000_0000
        NORTH_WEST_MOVES[square] = {}
        NORTH_WEST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_top_square(get_left_square(square))
        carry = 0
        while current != 0:
            NORTH_WEST_RAY[square] |= current
            for p in generate_possibilities(
                lambda x: get_top_square(get_left_square(x))
            ):
                NORTH_WEST_MOVES[square][p | current] = carry
                NORTH_WEST_ATTACKS[square][p | current] = current
            carry |= current
            current = get_top_square(get_left_square(current))
        NORTH_WEST_MOVES[square][0] = carry

        NORTH_EAST_RAY[square] = 0x0000_0000_0000_0000
        NORTH_EAST_MOVES[square] = {}
        NORTH_EAST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_top_square(get_right_square(square))
        carry = 0
        while current != 0:
            NORTH_EAST_RAY[square] |= current
            for p in generate_possibilities(
                lambda x: get_top_square(get_right_square(x))
            ):
                NORTH_EAST_MOVES[square][p | current] = carry
                NORTH_EAST_ATTACKS[square][p | current] = current
            carry |= current
            current = get_top_square(get_right_square(current))
        NORTH_EAST_MOVES[square][0] = carry

        SOUTH_WEST_RAY[square] = 0x0000_0000_0000_0000
        SOUTH_WEST_MOVES[square] = {}
        SOUTH_WEST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_bottom_square(get_left_square(square))
        carry = 0
        while current != 0:
            SOUTH_WEST_RAY[square] |= current
            for p in generate_possibilities(
                lambda x: get_bottom_square(get_left_square(x))
            ):
                SOUTH_WEST_MOVES[square][p | current] = carry
                SOUTH_WEST_ATTACKS[square][p | current] = current
            carry |= current
            current = get_bottom_square(get_left_square(current))
        SOUTH_WEST_MOVES[square][0] = carry

        SOUTH_EAST_RAY[square] = 0x0000_0000_0000_0000
        SOUTH_EAST_MOVES[square] = {}
        SOUTH_EAST_ATTACKS[square] = {0x0000_0000_0000_0000: 0x0000_0000_0000_0000}
        current = get_bottom_square(get_right_square(square))
        carry = 0
        while current != 0:
            SOUTH_EAST_RAY[square] |= current
            for p in generate_possibilities(
                lambda x: get_bottom_square(get_right_square(x))
            ):
                SOUTH_EAST_MOVES[square][p | current] = carry
                SOUTH_EAST_ATTACKS[square][p | current] = current
            carry |= current
            current = get_bottom_square(get_right_square(current))
        SOUTH_EAST_MOVES[square][0] = carry

        top = get_top_square(square)
        bottom = get_bottom_square(square)
        left = get_left_square(square)
        right = get_right_square(square)

        KING_MOVES[square] = (
            top
            | get_right_square(top)
            | right
            | get_bottom_square(right)
            | bottom
            | get_left_square(bottom)
            | left
            | get_top_square(left)
        )

        PAWN_ATTACKS[Player.WHITE][square] = get_left_square(bottom) | get_right_square(
            bottom
        )
        PAWN_ATTACKS[Player.BLACK][square] = get_left_square(top) | get_right_square(
            top
        )

        top2 = get_top_square(top)
        bottom2 = get_bottom_square(bottom)
        left2 = get_left_square(left)
        right2 = get_right_square(right)
        KNIGHT_MOVES[square] = (
            get_left_square(top2)
            | get_right_square(top2)
            | get_left_square(bottom2)
            | get_right_square(bottom2)
            | get_top_square(left2)
            | get_bottom_square(left2)
            | get_top_square(right2)
            | get_bottom_square(right2)
        )
