from typing import Callable, List

from py.bitboard import (
    get_top_square,
    get_bottom_square,
    get_left_square,
    get_right_square,
)


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
PAWN_ATTACKS = {True: {}, False: {}}
PAWN_SINGLE_MOVES = {True: {}, False: {}}
PAWN_SINGLE_MOVES_PROMOTION = {True: {}, False: {}}
PAWN_DOUBLE_MOVES = {True: {}, False: {}}
PAWN_ATTACK_MOVES = {True: {}, False: {}}
PAWN_ATTACK_MOVES_PROMOTION = {True: {}, False: {}}
PAWN_EN_PASSANT_CAPTURES = {True: {}, False: {}}


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
        top_left = get_left_square(top)
        top_right = get_right_square(top)
        bottom_left = get_left_square(bottom)
        bottom_right = get_right_square(bottom)

        KING_MOVES[square] = (
            top
            | bottom
            | left
            | right
            | top_left
            | top_right
            | bottom_left
            | bottom_right
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

        PAWN_ATTACKS[True][square] = bottom_left | bottom_right
        PAWN_ATTACKS[False][square] = top_left | top_right

        if square & 0x00FF_0000_0000_0000 == 0:
            PAWN_SINGLE_MOVES[True][square] = top
            PAWN_SINGLE_MOVES_PROMOTION[True][square] = []
        else:
            PAWN_SINGLE_MOVES[True][square] = 0
            PAWN_SINGLE_MOVES_PROMOTION[True][square] = [top]

        if square & 0x0000_0000_0000_FF00 == 0:
            PAWN_SINGLE_MOVES[False][square] = bottom
            PAWN_SINGLE_MOVES_PROMOTION[False][square] = []
        else:
            PAWN_SINGLE_MOVES[False][square] = 0
            PAWN_SINGLE_MOVES_PROMOTION[False][square] = [bottom]

        if square & 0x0000_0000_0000_FF00 == 0:
            PAWN_DOUBLE_MOVES[True][square] = 0
        else:
            PAWN_DOUBLE_MOVES[True][square] = top2

        if square & 0x00FF_0000_0000_0000 == 0:
            PAWN_DOUBLE_MOVES[False][square] = 0
        else:
            PAWN_DOUBLE_MOVES[False][square] = bottom2

        black_pawn_attacks = []
        if bottom_left != 0:
            black_pawn_attacks += [bottom_left]
        if bottom_right != 0:
            black_pawn_attacks += [bottom_right]
        if square & 0x0000_0000_0000_FF00 == 0:
            PAWN_ATTACK_MOVES[False][square] = black_pawn_attacks
            PAWN_ATTACK_MOVES_PROMOTION[False][square] = []
        else:
            PAWN_ATTACK_MOVES[False][square] = []
            PAWN_ATTACK_MOVES_PROMOTION[False][square] = black_pawn_attacks

        white_pawn_attacks = []
        if top_left != 0:
            white_pawn_attacks += [top_left]
        if top_right != 0:
            white_pawn_attacks += [top_right]
        if square & 0x00FF_0000_0000_0000 == 0:
            PAWN_ATTACK_MOVES[True][square] = white_pawn_attacks
            PAWN_ATTACK_MOVES_PROMOTION[True][square] = []
        else:
            PAWN_ATTACK_MOVES[True][square] = []
            PAWN_ATTACK_MOVES_PROMOTION[True][square] = white_pawn_attacks

        black_pawn_attacks = []
        if bottom_left != 0:
            black_pawn_attacks += [bottom_left]
        if bottom_right != 0:
            black_pawn_attacks += [bottom_right]
        if square & 0x0000_0000_0000_FF00 == 0:
            PAWN_ATTACK_MOVES[False][square] = black_pawn_attacks
            PAWN_ATTACK_MOVES_PROMOTION[False][square] = []
        else:
            PAWN_ATTACK_MOVES[False][square] = []
            PAWN_ATTACK_MOVES_PROMOTION[False][square] = black_pawn_attacks

        if square & 0x0000_00FF_0000_0000 == 0:
            PAWN_EN_PASSANT_CAPTURES[True][square] = 0
        else:
            PAWN_EN_PASSANT_CAPTURES[True][square] = top_left | top_right

        if square & 0x0000_0000_FF00_0000 == 0:
            PAWN_EN_PASSANT_CAPTURES[False][square] = 0
        else:
            PAWN_EN_PASSANT_CAPTURES[False][square] = bottom_left | bottom_right

# for s in NORTH_ATTACKS[0x0000_0001_0000_0000]:
#     print(bin(s), bin(NORTH_ATTACKS[0x0000_0001_0000_0000][s]))
# print("")
# for s in NORTH_MOVES[0x0000_0001_0000_0000]:
#     print(bin(s), bin(NORTH_MOVES[0x0000_0001_0000_0000][s]))
# print("")
# for s in SOUTH_ATTACKS[0x0000_0001_0000_0000]:
#     print(bin(s), bin(SOUTH_ATTACKS[0x0000_0001_0000_0000][s]))
# print("")
# for s in SOUTH_MOVES[0x0000_0001_0000_0000]:
#     print(bin(s), bin(SOUTH_MOVES[0x0000_0001_0000_0000][s]))
