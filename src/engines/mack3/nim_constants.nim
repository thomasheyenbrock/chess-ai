import math
import tables

from nim_bitboard import
    get_top_square,
    get_bottom_square,
    get_left_square,
    get_right_square


var NORTH_RAY* = initTable[uint64, uint64]()
var SOUTH_RAY* = initTable[uint64, uint64]()
var WEST_RAY* = initTable[uint64, uint64]()
var EAST_RAY* = initTable[uint64, uint64]()
var NORTH_WEST_RAY* = initTable[uint64, uint64]()
var NORTH_EAST_RAY* = initTable[uint64, uint64]()
var SOUTH_WEST_RAY* = initTable[uint64, uint64]()
var SOUTH_EAST_RAY* = initTable[uint64, uint64]()

var NORTH_MOVES* = initTable[uint64, Table[uint64, uint64]]()
var SOUTH_MOVES* = initTable[uint64, Table[uint64, uint64]]()
var WEST_MOVES* = initTable[uint64, Table[uint64, uint64]]()
var EAST_MOVES* = initTable[uint64, Table[uint64, uint64]]()
var NORTH_WEST_MOVES* = initTable[uint64, Table[uint64, uint64]]()
var NORTH_EAST_MOVES* = initTable[uint64, Table[uint64, uint64]]()
var SOUTH_WEST_MOVES* = initTable[uint64, Table[uint64, uint64]]()
var SOUTH_EAST_MOVES* = initTable[uint64, Table[uint64, uint64]]()

var NORTH_ATTACKS* = initTable[uint64, Table[uint64, uint64]]()
var SOUTH_ATTACKS* = initTable[uint64, Table[uint64, uint64]]()
var WEST_ATTACKS* = initTable[uint64, Table[uint64, uint64]]()
var EAST_ATTACKS* = initTable[uint64, Table[uint64, uint64]]()
var NORTH_WEST_ATTACKS* = initTable[uint64, Table[uint64, uint64]]()
var NORTH_EAST_ATTACKS* = initTable[uint64, Table[uint64, uint64]]()
var SOUTH_WEST_ATTACKS* = initTable[uint64, Table[uint64, uint64]]()
var SOUTH_EAST_ATTACKS* = initTable[uint64, Table[uint64, uint64]]()

var KING_MOVES* = initTable[uint64, uint64]()
var KNIGHT_MOVES* = initTable[uint64, uint64]()
var PAWN_ATTACKS* = initTable[bool, Table[uint64, uint64]]()
var PAWN_SINGLE_MOVES* = initTable[bool, Table[uint64, uint64]]()
var PAWN_SINGLE_MOVES_PROMOTION* = initTable[bool, Table[uint64, seq[uint64]]]()
var PAWN_DOUBLE_MOVES* = initTable[bool, Table[uint64, uint64]]()
var PAWN_ATTACK_MOVES* = initTable[bool, Table[uint64, seq[uint64]]]()
var PAWN_ATTACK_MOVES_PROMOTION* = initTable[bool, Table[uint64, seq[uint64]]]()
var PAWN_EN_PASSANT_CAPTURES* = initTable[bool, Table[uint64, uint64]]()

PAWN_ATTACKS[true] = initTable[uint64, uint64]()
PAWN_ATTACKS[false] = initTable[uint64, uint64]()
PAWN_SINGLE_MOVES[true] = initTable[uint64, uint64]()
PAWN_SINGLE_MOVES[false] = initTable[uint64, uint64]()
PAWN_SINGLE_MOVES_PROMOTION[true] = initTable[uint64, seq[uint64]]()
PAWN_SINGLE_MOVES_PROMOTION[false] = initTable[uint64, seq[uint64]]()
PAWN_DOUBLE_MOVES[true] = initTable[uint64, uint64]()
PAWN_DOUBLE_MOVES[false] = initTable[uint64, uint64]()
PAWN_ATTACK_MOVES[true] = initTable[uint64, seq[uint64]]()
PAWN_ATTACK_MOVES[false] = initTable[uint64, seq[uint64]]()
PAWN_ATTACK_MOVES_PROMOTION[true] = initTable[uint64, seq[uint64]]()
PAWN_ATTACK_MOVES_PROMOTION[false] = initTable[uint64, seq[uint64]]()
PAWN_EN_PASSANT_CAPTURES[true] = initTable[uint64, uint64]()
PAWN_EN_PASSANT_CAPTURES[false] = initTable[uint64, uint64]()


proc generate_possibilities(current: uint64, direction: proc (bb: uint64): uint64): seq[uint64] =
    var forward = direction(current)
    var possibilities: seq[uint64]
    if forward==0:
        possibilities = @[0'u64]
    else:
        possibilities = @[0'u64, forward]

    while forward != 0:
        forward = direction(forward)
        var new_possibilities: seq[uint64] = @[]
        for p in possibilities:
            new_possibilities.add(p)
            if forward != 0:
                new_possibilities.add(p or forward)
        possibilities = new_possibilities
    return possibilities


proc get_top_left_square(bitboard: uint64): uint64 =
    return get_top_square(get_left_square(bitboard))


proc get_top_right_square(bitboard: uint64): uint64 =
    return get_top_square(get_right_square(bitboard))


proc get_bottom_left_square(bitboard: uint64): uint64 =
    return get_bottom_square(get_left_square(bitboard))


proc get_bottom_right_square(bitboard: uint64): uint64 =
    return get_bottom_square(get_right_square(bitboard))


for rank in 0'u64 .. 7'u64:
    for file in 0'u64 .. 7'u64:
        let square = 2'u64 ^ (8 * rank + file)
        var current: uint64
        var carry: uint64

        let top = get_top_square(square)
        let bottom = get_bottom_square(square)
        let left = get_left_square(square)
        let right = get_right_square(square)
        let top_left = get_left_square(top)
        let top_right = get_right_square(top)
        let bottom_left = get_left_square(bottom)
        let bottom_right = get_right_square(bottom)

        NORTH_RAY[square] = 0x0000_0000_0000_0000'u64
        NORTH_MOVES[square] = initTable[uint64, uint64]()
        NORTH_ATTACKS[square] = {0x0000_0000_0000_0000'u64: 0x0000_0000_0000_0000'u64}.toTable
        current = top
        carry = 0'u64
        while current != 0:
            NORTH_RAY[square] = NORTH_RAY[square] or current
            for p in generate_possibilities(current, get_top_square):
                NORTH_MOVES[square][p or current] = carry
                NORTH_ATTACKS[square][p or current] = current
            carry = carry or current
            current = get_top_square(current)
        NORTH_MOVES[square][0] = carry

        SOUTH_RAY[square] = 0x0000_0000_0000_0000'u64
        SOUTH_MOVES[square] = initTable[uint64, uint64]()
        SOUTH_ATTACKS[square] = {0x0000_0000_0000_0000'u64: 0x0000_0000_0000_0000'u64}.toTable
        current = bottom
        carry = 0'u64
        while current != 0:
            SOUTH_RAY[square] = SOUTH_RAY[square] or current
            for p in generate_possibilities(current, get_bottom_square):
                SOUTH_MOVES[square][p or current] = carry
                SOUTH_ATTACKS[square][p or current] = current
            carry = carry or current
            current = get_bottom_square(current)
        SOUTH_MOVES[square][0] = carry

        WEST_RAY[square] = 0x0000_0000_0000_0000'u64
        WEST_MOVES[square] = initTable[uint64, uint64]()
        WEST_ATTACKS[square] = {0x0000_0000_0000_0000'u64: 0x0000_0000_0000_0000'u64}.toTable
        current = left
        carry = 0'u64
        while current != 0:
            WEST_RAY[square] = WEST_RAY[square] or current
            for p in generate_possibilities(current, get_left_square):
                WEST_MOVES[square][p or current] = carry
                WEST_ATTACKS[square][p or current] = current
            carry = carry or current
            current = get_left_square(current)
        WEST_MOVES[square][0] = carry

        EAST_RAY[square] = 0x0000_0000_0000_0000'u64
        EAST_MOVES[square] = initTable[uint64, uint64]()
        EAST_ATTACKS[square] = {0x0000_0000_0000_0000'u64: 0x0000_0000_0000_0000'u64}.toTable
        current = right
        carry = 0'u64
        while current != 0:
            EAST_RAY[square] = EAST_RAY[square] or current
            for p in generate_possibilities(current, get_right_square):
                EAST_MOVES[square][p or current] = carry
                EAST_ATTACKS[square][p or current] = current
            carry = carry or current
            current = get_right_square(current)
        EAST_MOVES[square][0] = carry

        NORTH_WEST_RAY[square] = 0x0000_0000_0000_0000'u64
        NORTH_WEST_MOVES[square] = initTable[uint64, uint64]()
        NORTH_WEST_ATTACKS[square] = {0x0000_0000_0000_0000'u64: 0x0000_0000_0000_0000'u64}.toTable
        current = top_left
        carry = 0'u64
        while current != 0:
            NORTH_WEST_RAY[square] = NORTH_WEST_RAY[square] or current
            for p in generate_possibilities(current, get_top_left_square):
                NORTH_WEST_MOVES[square][p or current] = carry
                NORTH_WEST_ATTACKS[square][p or current] = current
            carry = carry or current
            current = get_top_square(get_left_square(current))
        NORTH_WEST_MOVES[square][0] = carry

        NORTH_EAST_RAY[square] = 0x0000_0000_0000_0000'u64
        NORTH_EAST_MOVES[square] = initTable[uint64, uint64]()
        NORTH_EAST_ATTACKS[square] = {0x0000_0000_0000_0000'u64: 0x0000_0000_0000_0000'u64}.toTable
        current = top_right
        carry = 0'u64
        while current != 0:
            NORTH_EAST_RAY[square] = NORTH_EAST_RAY[square] or current
            for p in generate_possibilities(current, get_top_right_square):
                NORTH_EAST_MOVES[square][p or current] = carry
                NORTH_EAST_ATTACKS[square][p or current] = current
            carry = carry or current
            current = get_top_square(get_right_square(current))
        NORTH_EAST_MOVES[square][0] = carry

        SOUTH_WEST_RAY[square] = 0x0000_0000_0000_0000'u64
        SOUTH_WEST_MOVES[square] = initTable[uint64, uint64]()
        SOUTH_WEST_ATTACKS[square] = {0x0000_0000_0000_0000'u64: 0x0000_0000_0000_0000'u64}.toTable
        current = bottom_left
        carry = 0'u64
        while current != 0:
            SOUTH_WEST_RAY[square] = SOUTH_WEST_RAY[square] or current
            for p in generate_possibilities(current, get_bottom_left_square):
                SOUTH_WEST_MOVES[square][p or current] = carry
                SOUTH_WEST_ATTACKS[square][p or current] = current
            carry = carry or current
            current = get_bottom_square(get_left_square(current))
        SOUTH_WEST_MOVES[square][0] = carry

        SOUTH_EAST_RAY[square] = 0x0000_0000_0000_0000'u64
        SOUTH_EAST_MOVES[square] = initTable[uint64, uint64]()
        SOUTH_EAST_ATTACKS[square] = {0x0000_0000_0000_0000'u64: 0x0000_0000_0000_0000'u64}.toTable
        current = bottom_right
        carry = 0'u64
        while current != 0:
            SOUTH_EAST_RAY[square] = SOUTH_EAST_RAY[square] or current
            for p in generate_possibilities(current, get_bottom_right_square):
                SOUTH_EAST_MOVES[square][p or current] = carry
                SOUTH_EAST_ATTACKS[square][p or current] = current
            carry = carry or current
            current = get_bottom_square(get_right_square(current))
        SOUTH_EAST_MOVES[square][0] = carry

        KING_MOVES[square] = (
            top or
            bottom or
            left or
            right or
            top_left or
            top_right or
            bottom_left or
            bottom_right
        )

        let top2 = get_top_square(top)
        let bottom2 = get_bottom_square(bottom)
        let left2 = get_left_square(left)
        let right2 = get_right_square(right)
        KNIGHT_MOVES[square] = (
            get_left_square(top2) or
            get_right_square(top2) or
            get_left_square(bottom2) or
            get_right_square(bottom2) or
            get_top_square(left2) or
            get_bottom_square(left2) or
            get_top_square(right2) or
            get_bottom_square(right2)
        )

        PAWN_ATTACKS[true][square] = bottom_left or bottom_right
        PAWN_ATTACKS[false][square] = top_left or top_right

        if (square and 0x00FF_0000_0000_0000'u64) == 0:
            PAWN_SINGLE_MOVES[true][square] = top
            PAWN_SINGLE_MOVES_PROMOTION[true][square] = @[]
        else:
            PAWN_SINGLE_MOVES[true][square] = 0
            PAWN_SINGLE_MOVES_PROMOTION[true][square] = @[top]

        if (square and 0x0000_0000_0000_FF00'u64) == 0:
            PAWN_SINGLE_MOVES[false][square] = bottom
            PAWN_SINGLE_MOVES_PROMOTION[false][square] = @[]
        else:
            PAWN_SINGLE_MOVES[false][square] = 0
            PAWN_SINGLE_MOVES_PROMOTION[false][square] = @[bottom]

        if (square and 0x0000_0000_0000_FF00'u64) == 0:
            PAWN_DOUBLE_MOVES[true][square] = 0
        else:
            PAWN_DOUBLE_MOVES[true][square] = top2

        if (square and 0x00FF_0000_0000_0000'u64) == 0:
            PAWN_DOUBLE_MOVES[false][square] = 0
        else:
            PAWN_DOUBLE_MOVES[false][square] = bottom2

        var white_pawn_attacks: seq[uint64] = @[]
        if top_left != 0:
            white_pawn_attacks.add(top_left)
        if top_right != 0:
            white_pawn_attacks.add(top_right)
        if (square and 0x00FF_0000_0000_0000'u64) == 0:
            PAWN_ATTACK_MOVES[true][square] = white_pawn_attacks
            PAWN_ATTACK_MOVES_PROMOTION[true][square] = @[]
        else:
            PAWN_ATTACK_MOVES[true][square] = @[]
            PAWN_ATTACK_MOVES_PROMOTION[true][square] = white_pawn_attacks

        var black_pawn_attacks: seq[uint64] = @[]
        if bottom_left != 0:
            black_pawn_attacks.add(bottom_left)
        if bottom_right != 0:
            black_pawn_attacks.add(bottom_right)
        if (square and 0x0000_0000_0000_FF00'u64) == 0:
            PAWN_ATTACK_MOVES[false][square] = black_pawn_attacks
            PAWN_ATTACK_MOVES_PROMOTION[false][square] = @[]
        else:
            PAWN_ATTACK_MOVES[false][square] = @[]
            PAWN_ATTACK_MOVES_PROMOTION[false][square] = black_pawn_attacks

        if (square and 0x0000_00FF_0000_0000'u64) == 0:
            PAWN_EN_PASSANT_CAPTURES[true][square] = 0
        else:
            PAWN_EN_PASSANT_CAPTURES[true][square] = top_left or top_right

        if (square and 0x0000_0000_FF00_0000'u64) == 0:
            PAWN_EN_PASSANT_CAPTURES[false][square] = 0
        else:
            PAWN_EN_PASSANT_CAPTURES[false][square] = bottom_left or bottom_right
