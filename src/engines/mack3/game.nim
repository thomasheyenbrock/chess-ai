import hashes
import nimpy
import sequtils
import strutils
import tables

from bitboard import get_bottom_square, get_top_square, split
from constants import
    EAST_MOVES,
    NORTH_EAST_MOVES,
    NORTH_MOVES,
    NORTH_RAY,
    NORTH_WEST_MOVES,
    PAWN_ATTACK_MOVES,
    PAWN_DOUBLE_MOVES,
    PAWN_EN_PASSANT_CAPTURES,
    PAWN_SINGLE_MOVES,
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
    PAWN_ATTACKS


const SQUARES = [
    0x8000_0000_0000_0000'u64,
    0x4000_0000_0000_0000'u64,
    0x2000_0000_0000_0000'u64,
    0x1000_0000_0000_0000'u64,
    0x0800_0000_0000_0000'u64,
    0x0400_0000_0000_0000'u64,
    0x0200_0000_0000_0000'u64,
    0x0100_0000_0000_0000'u64,
    0x0080_0000_0000_0000'u64,
    0x0040_0000_0000_0000'u64,
    0x0020_0000_0000_0000'u64,
    0x0010_0000_0000_0000'u64,
    0x0008_0000_0000_0000'u64,
    0x0004_0000_0000_0000'u64,
    0x0002_0000_0000_0000'u64,
    0x0001_0000_0000_0000'u64,
    0x0000_8000_0000_0000'u64,
    0x0000_4000_0000_0000'u64,
    0x0000_2000_0000_0000'u64,
    0x0000_1000_0000_0000'u64,
    0x0000_0800_0000_0000'u64,
    0x0000_0400_0000_0000'u64,
    0x0000_0200_0000_0000'u64,
    0x0000_0100_0000_0000'u64,
    0x0000_0080_0000_0000'u64,
    0x0000_0040_0000_0000'u64,
    0x0000_0020_0000_0000'u64,
    0x0000_0010_0000_0000'u64,
    0x0000_0008_0000_0000'u64,
    0x0000_0004_0000_0000'u64,
    0x0000_0002_0000_0000'u64,
    0x0000_0001_0000_0000'u64,
    0x0000_0000_8000_0000'u64,
    0x0000_0000_4000_0000'u64,
    0x0000_0000_2000_0000'u64,
    0x0000_0000_1000_0000'u64,
    0x0000_0000_0800_0000'u64,
    0x0000_0000_0400_0000'u64,
    0x0000_0000_0200_0000'u64,
    0x0000_0000_0100_0000'u64,
    0x0000_0000_0080_0000'u64,
    0x0000_0000_0040_0000'u64,
    0x0000_0000_0020_0000'u64,
    0x0000_0000_0010_0000'u64,
    0x0000_0000_0008_0000'u64,
    0x0000_0000_0004_0000'u64,
    0x0000_0000_0002_0000'u64,
    0x0000_0000_0001_0000'u64,
    0x0000_0000_0000_8000'u64,
    0x0000_0000_0000_4000'u64,
    0x0000_0000_0000_2000'u64,
    0x0000_0000_0000_1000'u64,
    0x0000_0000_0000_0800'u64,
    0x0000_0000_0000_0400'u64,
    0x0000_0000_0000_0200'u64,
    0x0000_0000_0000_0100'u64,
    0x0000_0000_0000_0080'u64,
    0x0000_0000_0000_0040'u64,
    0x0000_0000_0000_0020'u64,
    0x0000_0000_0000_0010'u64,
    0x0000_0000_0000_0008'u64,
    0x0000_0000_0000_0004'u64,
    0x0000_0000_0000_0002'u64,
    0x0000_0000_0000_0001'u64
]


const SQUARES_TO_HUMAN = {
    0x8000_0000_0000_0000'u64: "a8",
    0x4000_0000_0000_0000'u64: "b8",
    0x2000_0000_0000_0000'u64: "c8",
    0x1000_0000_0000_0000'u64: "d8",
    0x0800_0000_0000_0000'u64: "e8",
    0x0400_0000_0000_0000'u64: "f8",
    0x0200_0000_0000_0000'u64: "g8",
    0x0100_0000_0000_0000'u64: "h8",
    0x0080_0000_0000_0000'u64: "a7",
    0x0040_0000_0000_0000'u64: "b7",
    0x0020_0000_0000_0000'u64: "c7",
    0x0010_0000_0000_0000'u64: "d7",
    0x0008_0000_0000_0000'u64: "e7",
    0x0004_0000_0000_0000'u64: "f7",
    0x0002_0000_0000_0000'u64: "g7",
    0x0001_0000_0000_0000'u64: "h7",
    0x0000_8000_0000_0000'u64: "a6",
    0x0000_4000_0000_0000'u64: "b6",
    0x0000_2000_0000_0000'u64: "c6",
    0x0000_1000_0000_0000'u64: "d6",
    0x0000_0800_0000_0000'u64: "e6",
    0x0000_0400_0000_0000'u64: "f6",
    0x0000_0200_0000_0000'u64: "g6",
    0x0000_0100_0000_0000'u64: "h6",
    0x0000_0080_0000_0000'u64: "a5",
    0x0000_0040_0000_0000'u64: "b5",
    0x0000_0020_0000_0000'u64: "c5",
    0x0000_0010_0000_0000'u64: "d5",
    0x0000_0008_0000_0000'u64: "e5",
    0x0000_0004_0000_0000'u64: "f5",
    0x0000_0002_0000_0000'u64: "g5",
    0x0000_0001_0000_0000'u64: "h5",
    0x0000_0000_8000_0000'u64: "a4",
    0x0000_0000_4000_0000'u64: "b4",
    0x0000_0000_2000_0000'u64: "c4",
    0x0000_0000_1000_0000'u64: "d4",
    0x0000_0000_0800_0000'u64: "e4",
    0x0000_0000_0400_0000'u64: "f4",
    0x0000_0000_0200_0000'u64: "g4",
    0x0000_0000_0100_0000'u64: "h4",
    0x0000_0000_0080_0000'u64: "a3",
    0x0000_0000_0040_0000'u64: "b3",
    0x0000_0000_0020_0000'u64: "c3",
    0x0000_0000_0010_0000'u64: "d3",
    0x0000_0000_0008_0000'u64: "e3",
    0x0000_0000_0004_0000'u64: "f3",
    0x0000_0000_0002_0000'u64: "g3",
    0x0000_0000_0001_0000'u64: "h3",
    0x0000_0000_0000_8000'u64: "a2",
    0x0000_0000_0000_4000'u64: "b2",
    0x0000_0000_0000_2000'u64: "c2",
    0x0000_0000_0000_1000'u64: "d2",
    0x0000_0000_0000_0800'u64: "e2",
    0x0000_0000_0000_0400'u64: "f2",
    0x0000_0000_0000_0200'u64: "g2",
    0x0000_0000_0000_0100'u64: "h2",
    0x0000_0000_0000_0080'u64: "a1",
    0x0000_0000_0000_0040'u64: "b1",
    0x0000_0000_0000_0020'u64: "c1",
    0x0000_0000_0000_0010'u64: "d1",
    0x0000_0000_0000_0008'u64: "e1",
    0x0000_0000_0000_0004'u64: "f1",
    0x0000_0000_0000_0002'u64: "g1",
    0x0000_0000_0000_0001'u64: "h1"
}.toTable


const RESULT_WHITE* = "White wins"
const RESULT_BLACK* = "Black wins"
const RESULT_STALEMATE* = "Stalemate"
const RESULT_DEAD_POSITION* = "Dead position"
const RESULT_REPITITION* = "Third repitition of position"
const RESULT_FIFTY_MOVE_RULE* = "Fifty moves without capture or pawn movement"


proc get_rank_and_file_moves(all_pieces: uint64, enemy_pieces: uint64, square: uint64): uint64 =
    let north_pieces = NORTH_RAY[square] and all_pieces
    let south_pieces = SOUTH_RAY[square] and all_pieces
    let west_pieces = WEST_RAY[square] and all_pieces
    let east_pieces = EAST_RAY[square] and all_pieces

    let north_moves = NORTH_MOVES[square][north_pieces] xor (
        NORTH_ATTACKS[square][north_pieces] and enemy_pieces
    )
    let south_moves = SOUTH_MOVES[square][south_pieces] xor (
        SOUTH_ATTACKS[square][south_pieces] and enemy_pieces
    )
    let west_moves = WEST_MOVES[square][west_pieces] xor (
        WEST_ATTACKS[square][west_pieces] and enemy_pieces
    )
    let east_moves = EAST_MOVES[square][east_pieces] xor (
        EAST_ATTACKS[square][east_pieces] and enemy_pieces
    )

    return north_moves or south_moves or west_moves or east_moves


proc get_diagonal_moves(all_pieces: uint64, enemy_pieces: uint64, square: uint64): uint64 =
    let north_west_pieces = NORTH_WEST_RAY[square] and all_pieces
    let south_west_pieces = SOUTH_WEST_RAY[square] and all_pieces
    let north_east_pieces = NORTH_EAST_RAY[square] and all_pieces
    let south_east_pieces = SOUTH_EAST_RAY[square] and all_pieces

    let north_west_moves = NORTH_WEST_MOVES[square][north_west_pieces] xor (
        NORTH_WEST_ATTACKS[square][north_west_pieces] and enemy_pieces
    )
    let north_east_moves = NORTH_EAST_MOVES[square][north_east_pieces] xor (
        NORTH_EAST_ATTACKS[square][north_east_pieces] and enemy_pieces
    )
    let south_west_moves = SOUTH_WEST_MOVES[square][south_west_pieces] xor (
        SOUTH_WEST_ATTACKS[square][south_west_pieces] and enemy_pieces
    )
    let south_east_moves = SOUTH_EAST_MOVES[square][south_east_pieces] xor (
        SOUTH_EAST_ATTACKS[square][south_east_pieces] and enemy_pieces
    )

    return north_west_moves or north_east_moves or south_west_moves or south_east_moves


type Move* = object
    player*: bool
    piece*: char
    from_square*: uint64
    to_square*: uint64
    en_passant_square*: uint64
    is_capturing_en_passant*: bool
    is_castling*: char
    is_promoting_to*: char


proc newMove(
    player: bool,
    piece: char,
    from_square: uint64,
    to_square: uint64,
    en_passant_square: uint64 = 0'u64,
    is_capturing_en_passant: bool = false,
    is_castling: char = '0',
    is_promoting_to: char = '0'
): Move =
    return Move(
        player: player,
        piece: piece,
        from_square: from_square,
        to_square: to_square,
        en_passant_square: en_passant_square,
        is_capturing_en_passant: is_capturing_en_passant,
        is_castling: is_castling,
        is_promoting_to: is_promoting_to
    )


proc to_string*(move: Move): string =
    result = SQUARES_TO_HUMAN[move.from_square] & SQUARES_TO_HUMAN[move.to_square]
    if move.is_promoting_to != '0':
        result &= $move.is_promoting_to


type Position = object
    all_pieces*: uint64
    white_pieces*: uint64
    black_pieces*: uint64
    pieces*: Table[char, uint64]


proc newPosition(
    K: uint64,
    Q: uint64,
    R: uint64,
    B: uint64,
    N: uint64,
    P: uint64,
    k: uint64,
    q: uint64,
    r: uint64,
    b: uint64,
    n: uint64,
    p: uint64,
): Position =
    let white_pieces = K or Q or R or B or N or P
    let black_pieces = k or q or r or b or n or p
    let pieces = {
        'K': K,
        'Q': Q,
        'R': R,
        'B': B,
        'N': N,
        'P': P,
        'k': k,
        'q': q,
        'r': r,
        'b': b,
        'n': n,
        'p': p,
    }.toTable
    return Position(
        all_pieces: white_pieces or black_pieces,
        white_pieces: white_pieces,
        black_pieces: black_pieces,
        pieces: pieces
    )


proc move(position: Position, move: Move): (Position, char) =
    var new_position = position
    if move.is_castling == 'K':
        new_position.pieces['K'] = 0x0000_0000_0000_0002'u64
        new_position.pieces['R'] = new_position.pieces['R'] xor 0x0000_0000_0000_0005'u64
        new_position.white_pieces = new_position.white_pieces xor 0x0000_0000_0000_000F'u64
        new_position.all_pieces = new_position.all_pieces xor 0x0000_0000_0000_000F'u64
        return (new_position, '0')
    if move.is_castling == 'Q':
        new_position.pieces['K'] = 0x0000_0000_0000_0020'u64
        new_position.pieces['R'] = new_position.pieces['R'] xor 0x0000_0000_0000_0090'u64
        new_position.white_pieces = new_position.white_pieces xor 0x0000_0000_0000_00B8'u64
        new_position.all_pieces = new_position.all_pieces xor 0x0000_0000_0000_00B8'u64
        return (new_position, '0')
    if move.is_castling == 'k':
        new_position.pieces['k'] = 0x0200_0000_0000_0000'u64
        new_position.pieces['r'] = new_position.pieces['r'] xor 0x0500_0000_0000_0000'u64
        new_position.black_pieces = new_position.black_pieces xor 0x0F00_0000_0000_0000'u64
        new_position.all_pieces = new_position.all_pieces xor 0x0F00_0000_0000_0000'u64
        return (new_position, '0')
    if move.is_castling == 'q':
        new_position.pieces['k'] = 0x2000_0000_0000_0000'u64
        new_position.pieces['r'] = new_position.pieces['r'] xor 0x9000_0000_0000_0000'u64
        new_position.black_pieces = new_position.black_pieces xor 0xB800_0000_0000_0000'u64
        new_position.all_pieces = new_position.all_pieces xor 0xB800_0000_0000_0000'u64
        return (new_position, '0')

    var is_capturing = '0'
    if (move.to_square and new_position.pieces['P']) != 0:
        is_capturing = 'P'
    elif (move.to_square and new_position.pieces['p']) != 0:
        is_capturing = 'P'
    elif (move.to_square and new_position.pieces['N']) != 0:
        is_capturing = 'N'
    elif (move.to_square and new_position.pieces['n']) != 0:
        is_capturing = 'N'
    elif (move.to_square and new_position.pieces['B']) != 0:
        is_capturing = 'B'
    elif (move.to_square and new_position.pieces['b']) != 0:
        is_capturing = 'B'
    elif (move.to_square and new_position.pieces['R']) != 0:
        is_capturing = 'R'
    elif (move.to_square and new_position.pieces['r']) != 0:
        is_capturing = 'R'
    elif (move.to_square and new_position.pieces['Q']) != 0:
        is_capturing = 'Q'
    elif (move.to_square and new_position.pieces['q']) != 0:
        is_capturing = 'Q'

    let piece_key = (if move.player: move.piece else: toLowerAscii(move.piece))
    new_position.pieces[piece_key] = (
        new_position.pieces[piece_key] xor move.from_square
    ) or move.to_square
    if move.player:
        new_position.white_pieces = (
            new_position.white_pieces xor move.from_square
        ) or move.to_square
    else:
        new_position.black_pieces = (
            new_position.black_pieces xor move.from_square
        ) or move.to_square
    new_position.all_pieces = (
        new_position.all_pieces xor move.from_square
    ) or move.to_square

    if is_capturing != '0':
        let is_capturing_key = (if move.player: toLowerAscii(is_capturing) else: is_capturing)
        new_position.pieces[is_capturing_key] = new_position.pieces[is_capturing_key] xor move.to_square
        if move.player:
            new_position.black_pieces = new_position.black_pieces xor move.to_square
        else:
            new_position.white_pieces = new_position.white_pieces xor move.to_square

    if move.is_capturing_en_passant:
        let captured_square = (
            if move.player: get_bottom_square(move.to_square)
            else: get_top_square(move.to_square)
        )
        let pawn_key = (if move.player: 'p' else: 'P')
        new_position.pieces[pawn_key] = new_position.pieces[pawn_key] xor captured_square
        if move.player:
            new_position.black_pieces = new_position.black_pieces xor captured_square
        else:
            new_position.white_pieces = new_position.white_pieces xor captured_square
        new_position.all_pieces = new_position.all_pieces xor captured_square

    if move.is_promoting_to != '0':
        let is_promoting_to_key = (
            if move.player: move.is_promoting_to
            else: toLowerAscii(move.is_promoting_to)
        )
        let pawn_key = (if move.player: 'P' else: 'p')
        new_position.pieces[is_promoting_to_key] = new_position.pieces[is_promoting_to_key] or move.to_square
        new_position.pieces[pawn_key] = new_position.pieces[pawn_key] xor move.to_square

    return (new_position, is_capturing)


proc attackers(position: Position, player: bool, square: uint64): uint64 =
    let king = position.pieces[if player: 'K' else: 'k']
    let queen = position.pieces[if player: 'Q' else: 'q']
    let rook = position.pieces[if player: 'R' else: 'r']
    let bishop = position.pieces[if player: 'B' else: 'b']
    let knight = position.pieces[if player: 'N' else: 'n']
    let pawn = position.pieces[if player: 'P' else: 'p']

    let queen_and_rook = queen or rook
    let queen_and_bishop = queen or bishop

    let north_pieces = NORTH_RAY[square] and position.all_pieces
    let south_pieces = SOUTH_RAY[square] and position.all_pieces
    let west_pieces = WEST_RAY[square] and position.all_pieces
    let east_pieces = EAST_RAY[square] and position.all_pieces
    let north_west_pieces = NORTH_WEST_RAY[square] and position.all_pieces
    let south_west_pieces = SOUTH_WEST_RAY[square] and position.all_pieces
    let north_east_pieces = NORTH_EAST_RAY[square] and position.all_pieces
    let south_east_pieces = SOUTH_EAST_RAY[square] and position.all_pieces

    return (
        (KING_MOVES[square] and king) or
        (NORTH_ATTACKS[square][north_pieces] and queen_and_rook) or
        (SOUTH_ATTACKS[square][south_pieces] and queen_and_rook) or
        (WEST_ATTACKS[square][west_pieces] and queen_and_rook) or
        (EAST_ATTACKS[square][east_pieces] and queen_and_rook) or
        (NORTH_WEST_ATTACKS[square][north_west_pieces] and queen_and_bishop) or
        (SOUTH_WEST_ATTACKS[square][south_west_pieces] and queen_and_bishop) or
        (NORTH_EAST_ATTACKS[square][north_east_pieces] and queen_and_bishop) or
        (SOUTH_EAST_ATTACKS[square][south_east_pieces] and queen_and_bishop) or
        (KNIGHT_MOVES[square] and knight) or
        (PAWN_ATTACKS[player][square] and pawn)
    )


iterator checkers(position: Position, player: bool, king: uint64): uint64 =
    let queen = position.pieces[if player: 'Q' else: 'q']
    let rook = position.pieces[if player: 'R' else: 'r']
    let bishop = position.pieces[if player: 'B' else: 'b']
    let knight = position.pieces[if player: 'N' else: 'n']
    let pawn = position.pieces[if player: 'P' else: 'p']

    let queen_and_rook = queen or rook
    let queen_and_bishop = queen or bishop

    let north_pieces = NORTH_RAY[king] and position.all_pieces
    let south_pieces = SOUTH_RAY[king] and position.all_pieces
    let west_pieces = WEST_RAY[king] and position.all_pieces
    let east_pieces = EAST_RAY[king] and position.all_pieces
    let north_west_pieces = NORTH_WEST_RAY[king] and position.all_pieces
    let south_west_pieces = SOUTH_WEST_RAY[king] and position.all_pieces
    let north_east_pieces = NORTH_EAST_RAY[king] and position.all_pieces
    let south_east_pieces = SOUTH_EAST_RAY[king] and position.all_pieces

    for square in split(
        (NORTH_ATTACKS[king][north_pieces] and queen_and_rook) or
        (SOUTH_ATTACKS[king][south_pieces] and queen_and_rook) or
        (WEST_ATTACKS[king][west_pieces] and queen_and_rook) or
        (EAST_ATTACKS[king][east_pieces] and queen_and_rook) or
        (NORTH_WEST_ATTACKS[king][north_west_pieces] and queen_and_bishop) or
        (SOUTH_WEST_ATTACKS[king][south_west_pieces] and queen_and_bishop) or
        (NORTH_EAST_ATTACKS[king][north_east_pieces] and queen_and_bishop) or
        (SOUTH_EAST_ATTACKS[king][south_east_pieces] and queen_and_bishop) or
        (KNIGHT_MOVES[king] and knight) or
        (PAWN_ATTACKS[player][king] and pawn)
    ):
        yield square


proc attacked_squares(position: Position, player: bool, exclude_king: bool = false): uint64 =
    let king_key = (if player: 'K' else: 'k')
    let queen_key = (if player: 'Q' else: 'q')
    let rook_key = (if player: 'R' else: 'r')
    let bishop_key = (if player: 'B' else: 'b')
    let knight_key = (if player: 'N' else: 'n')
    let pawn_key = (if player: 'P' else: 'p')

    var all_pieces = position.all_pieces
    if exclude_king:
        all_pieces = all_pieces xor position.pieces[if player: 'k' else: 'K']

    var attacked = KING_MOVES[position.pieces[king_key]]

    for queen in split(position.pieces[queen_key]):
        let north_pieces = NORTH_RAY[queen] and all_pieces
        attacked = attacked or NORTH_MOVES[queen][north_pieces] or NORTH_ATTACKS[queen][north_pieces]
        let south_pieces = SOUTH_RAY[queen] and all_pieces
        attacked = attacked or SOUTH_MOVES[queen][south_pieces] or SOUTH_ATTACKS[queen][south_pieces]
        let west_pieces = WEST_RAY[queen] and all_pieces
        attacked = attacked or WEST_MOVES[queen][west_pieces] or WEST_ATTACKS[queen][west_pieces]
        let east_pieces = EAST_RAY[queen] and all_pieces
        attacked = attacked or EAST_MOVES[queen][east_pieces] or EAST_ATTACKS[queen][east_pieces]
        let north_west_pieces = NORTH_WEST_RAY[queen] and all_pieces
        attacked = attacked or NORTH_WEST_MOVES[queen][north_west_pieces] or NORTH_WEST_ATTACKS[queen][north_west_pieces]
        let north_east_pieces = NORTH_EAST_RAY[queen] and all_pieces
        attacked = attacked or NORTH_EAST_MOVES[queen][north_east_pieces] or NORTH_EAST_ATTACKS[queen][north_east_pieces]
        let south_west_pieces = SOUTH_WEST_RAY[queen] and all_pieces
        attacked = attacked or SOUTH_WEST_MOVES[queen][south_west_pieces] or SOUTH_WEST_ATTACKS[queen][south_west_pieces]
        let south_east_pieces = SOUTH_EAST_RAY[queen] and all_pieces
        attacked = attacked or SOUTH_EAST_MOVES[queen][south_east_pieces] or SOUTH_EAST_ATTACKS[queen][south_east_pieces]

    for rook in split(position.pieces[rook_key]):
        let north_pieces = NORTH_RAY[rook] and all_pieces
        attacked = attacked or NORTH_MOVES[rook][north_pieces] or NORTH_ATTACKS[rook][north_pieces]
        let south_pieces = SOUTH_RAY[rook] and all_pieces
        attacked = attacked or SOUTH_MOVES[rook][south_pieces] or SOUTH_ATTACKS[rook][south_pieces]
        let west_pieces = WEST_RAY[rook] and all_pieces
        attacked = attacked or WEST_MOVES[rook][west_pieces] or WEST_ATTACKS[rook][west_pieces]
        let east_pieces = EAST_RAY[rook] and all_pieces
        attacked = attacked or EAST_MOVES[rook][east_pieces] or EAST_ATTACKS[rook][east_pieces]

    for bishop in split(position.pieces[bishop_key]):
        let north_west_pieces = NORTH_WEST_RAY[bishop] and all_pieces
        attacked = attacked or NORTH_WEST_MOVES[bishop][north_west_pieces] or NORTH_WEST_ATTACKS[bishop][north_west_pieces]
        let north_east_pieces = NORTH_EAST_RAY[bishop] and all_pieces
        attacked = attacked or NORTH_EAST_MOVES[bishop][north_east_pieces] or NORTH_EAST_ATTACKS[bishop][north_east_pieces]
        let south_west_pieces = SOUTH_WEST_RAY[bishop] and all_pieces
        attacked = attacked or SOUTH_WEST_MOVES[bishop][south_west_pieces] or SOUTH_WEST_ATTACKS[bishop][south_west_pieces]
        let south_east_pieces = SOUTH_EAST_RAY[bishop] and all_pieces
        attacked = attacked or SOUTH_EAST_MOVES[bishop][south_east_pieces] or SOUTH_EAST_ATTACKS[bishop][south_east_pieces]

    for knight in split(position.pieces[knight_key]):
        attacked = attacked or KNIGHT_MOVES[knight]

    for pawn in split(position.pieces[pawn_key]):
        for s in PAWN_ATTACK_MOVES[player][pawn]:
            attacked = attacked or s

    return attacked


proc is_check(position: Position, player: bool): bool =
    return position.attackers(not player, position.pieces[if player: 'K' else: 'k']) != 0


proc pinned_movement(
    position: Position,
    square: uint64,
    king: uint64,
    enemy_queens_and_rooks: uint64,
    enemy_queens_and_bishops: uint64,
): uint64 =
    let north_pieces = NORTH_RAY[square] and position.all_pieces
    let south_pieces = SOUTH_RAY[square] and position.all_pieces
    let first_piece_to_north = NORTH_ATTACKS[square][north_pieces]
    let first_piece_to_south = SOUTH_ATTACKS[square][south_pieces]

    let is_pinned_from_north = (first_piece_to_south == king) and (
        (first_piece_to_north and enemy_queens_and_rooks) != 0
    )
    if is_pinned_from_north:
        return (
            first_piece_to_north or
            NORTH_MOVES[square][north_pieces] or
            SOUTH_MOVES[square][south_pieces]
        )

    let is_pinned_from_south = (first_piece_to_north == king) and (
        (first_piece_to_south and enemy_queens_and_rooks) != 0
    )
    if is_pinned_from_south:
        return (
            first_piece_to_south or
            SOUTH_MOVES[square][south_pieces] or
            NORTH_MOVES[square][north_pieces]
        )

    let west_pieces = WEST_RAY[square] and position.all_pieces
    let east_pieces = EAST_RAY[square] and position.all_pieces
    let first_piece_to_west = WEST_ATTACKS[square][west_pieces]
    let first_piece_to_east = EAST_ATTACKS[square][east_pieces]

    let is_pinned_from_west = (first_piece_to_east == king) and (
        (first_piece_to_west and enemy_queens_and_rooks) != 0
    )
    if is_pinned_from_west:
        return (
            first_piece_to_west or
            WEST_MOVES[square][west_pieces] or
            EAST_MOVES[square][east_pieces]
        )

    let is_pinned_from_east = (first_piece_to_west == king) and (
        (first_piece_to_east and enemy_queens_and_rooks) != 0
    )
    if is_pinned_from_east:
        return (
            first_piece_to_east or
            EAST_MOVES[square][east_pieces] or
            WEST_MOVES[square][west_pieces]
        )

    let north_west_pieces = NORTH_WEST_RAY[square] and position.all_pieces
    let south_east_pieces = SOUTH_EAST_RAY[square] and position.all_pieces
    let first_piece_to_north_west = NORTH_WEST_ATTACKS[square][north_west_pieces]
    let first_piece_to_south_east = SOUTH_EAST_ATTACKS[square][south_east_pieces]

    let is_pinned_from_north_west = (first_piece_to_south_east == king) and (
        (first_piece_to_north_west and enemy_queens_and_bishops) != 0
    )
    if is_pinned_from_north_west:
        return (
            first_piece_to_north_west or
            NORTH_WEST_MOVES[square][north_west_pieces] or
            SOUTH_EAST_MOVES[square][south_east_pieces]
        )

    let is_pinned_from_south_east = (first_piece_to_north_west == king) and (
        (first_piece_to_south_east and enemy_queens_and_bishops) != 0
    )
    if is_pinned_from_south_east:
        return (
            first_piece_to_south_east or
            SOUTH_EAST_MOVES[square][south_east_pieces] or
            NORTH_WEST_MOVES[square][north_west_pieces]
        )

    let north_east_pieces = NORTH_EAST_RAY[square] and position.all_pieces
    let south_west_pieces = SOUTH_WEST_RAY[square] and position.all_pieces
    let first_piece_to_north_east = NORTH_EAST_ATTACKS[square][north_east_pieces]
    let first_piece_to_south_west = SOUTH_WEST_ATTACKS[square][south_west_pieces]

    let is_pinned_from_north_east = (first_piece_to_south_west == king) and (
        (first_piece_to_north_east and enemy_queens_and_bishops) != 0
    )
    if is_pinned_from_north_east:
        return (
            first_piece_to_north_east or
            NORTH_EAST_MOVES[square][north_east_pieces] or
            SOUTH_WEST_MOVES[square][south_west_pieces]
        )

    let is_pinned_from_south_west = (first_piece_to_north_east == king) and (
        (first_piece_to_south_west and enemy_queens_and_bishops) != 0
    )
    if is_pinned_from_south_west:
        return (
            first_piece_to_south_west or
            SOUTH_WEST_MOVES[square][south_west_pieces] or
            NORTH_EAST_MOVES[square][north_east_pieces]
        )

    return 0xFFFF_FFFF_FFFF_FFFF'u64


proc is_dead(position: Position): bool =
    let white_queens = toSeq(split(position.pieces['Q'])).len
    if white_queens > 0:
        return false

    let black_queens = toSeq(split(position.pieces['q'])).len
    if black_queens > 0:
        return false

    let white_rooks = toSeq(split(position.pieces['R'])).len
    if white_rooks > 0:
        return false

    let black_rooks = toSeq(split(position.pieces['r'])).len
    if black_rooks > 0:
        return false

    let white_pawns = toSeq(split(position.pieces['P'])).len
    if white_pawns > 0:
        return false

    let black_pawns = toSeq(split(position.pieces['p'])).len
    if black_pawns > 0:
        return false

    let white_bishops = toSeq(split(position.pieces['B']))
    if white_bishops.len > 1:
        return false

    let black_bishops = toSeq(split(position.pieces['b']))
    if black_bishops.len > 1:
        return false

    let white_knights = toSeq(split(position.pieces['N'])).len
    if white_knights > 1:
        return false

    let black_knights = toSeq(split(position.pieces['n'])).len
    if black_knights > 1:
        return false

    let number_of_white_pieces = white_queens +
        white_rooks +
        white_bishops.len +
        white_knights +
        white_pawns
    let number_of_black_pieces = black_queens +
        black_rooks +
        black_bishops.len +
        black_knights +
        black_pawns

    # king against king
    if number_of_white_pieces + number_of_black_pieces == 0:
        return true

    # king against king and bishop
    if (
        number_of_white_pieces == 0 and
        number_of_black_pieces == 1 and
        black_bishops.len == 1
    ):
        return true
    if (
        number_of_black_pieces == 0 and
        number_of_white_pieces == 1 and
        white_bishops.len == 1
    ):
        return true

    # king against king and knight
    if (
        number_of_white_pieces == 0 and
        number_of_black_pieces == 1 and
        black_knights == 1
    ):
        return true
    if (
        number_of_black_pieces == 0 and
        number_of_white_pieces == 1 and
        white_knights == 1
    ):
        return true

    # king and bishop against king and bishop, with both bishops on squares of the same color
    if (
        number_of_white_pieces == 1 and
        number_of_black_pieces == 1 and
        white_bishops.len == 1 and
        black_bishops.len == 1
    ):
        let is_white_bishop_on_white_square = (
            white_bishops[0] and 0xAA55_AA55_AA55_AA55'u64
        ) == 0
        let is_black_bishop_on_white_square = (
            black_bishops[0] and 0xAA55_AA55_AA55_AA55'u64
        ) == 0
        return is_white_bishop_on_white_square == is_black_bishop_on_white_square

    return false


type Game* = object
    position*: Position
    player*: bool
    last_move*: Move
    possible_castles*: Table[char, bool]
    en_passant_square*: uint64
    position_counts*: Table[int, int]
    move_counter*: int
    fifty_move_counter*: int


proc newGame(
    position: Position,
    player: bool,
    last_move: Move,
    possible_castles: Table[char, bool],
    en_passant_square: uint64,
    position_counts: Table[int, int],
    move_counter: int,
    fifty_move_counter: int,
): Game =
    return Game(
        position: position,
        player: player,
        last_move: last_move,
        possible_castles: possible_castles,
        en_passant_square: en_passant_square,
        position_counts: position_counts,
        move_counter: move_counter,
        fifty_move_counter: fifty_move_counter
    )


proc id(game: Game): int =
    let pieces = game.position.pieces
    var str = $pieces['K']
    str.add("-")
    str.add($pieces['Q'])
    str.add("-")
    str.add($pieces['R'])
    str.add("-")
    str.add($pieces['B'])
    str.add("-")
    str.add($pieces['N'])
    str.add("-")
    str.add($pieces['P'])
    str.add("-")
    str.add($pieces['k'])
    str.add("-")
    str.add($pieces['q'])
    str.add("-")
    str.add($pieces['r'])
    str.add("-")
    str.add($pieces['b'])
    str.add("-")
    str.add($pieces['n'])
    str.add("-")
    str.add($pieces['p'])
    str.add("-")
    str.add($game.player)
    str.add("-")
    str.add(if game.possible_castles['K']: "K" else: "")
    str.add("-")
    str.add(if game.possible_castles['Q']: "Q" else: "")
    str.add("-")
    str.add(if game.possible_castles['k']: "k" else: "")
    str.add("-")
    str.add(if game.possible_castles['q']: "q" else: "")
    str.add("-")
    str.add($game.en_passant_square)
    return hash(str)


proc move*(game: Game, move: Move): Game {.exportpy.} =
    let (new_position, is_capturing) = game.position.move(move)

    let possible_castles = {
        'K': game.possible_castles['K'] and
            not (game.player and move.piece == 'K') and
            not (
                game.player and
                move.piece == 'R' and
                move.from_square == 0x0000_0000_0000_0001'u64
            ) and
            not (
                not game.player and
                is_capturing == 'R' and
                move.to_square == 0x0000_0000_0000_0001'u64
            ),
        'Q': game.possible_castles['Q'] and
            not (game.player and move.piece == 'K') and
            not (
                game.player and
                move.piece == 'R' and
                move.from_square == 0x0000_0000_0000_0080'u64
            ) and
            not (
                not game.player and
                is_capturing == 'R' and
                move.to_square == 0x0000_0000_0000_0080'u64
            ),
        'k': game.possible_castles['k'] and
            not (not game.player and move.piece == 'K') and
            not (
                not game.player and
                move.piece == 'R' and
                move.from_square == 0x0100_0000_0000_0000'u64
            ) and
            not (
                game.player and
                is_capturing == 'R' and
                move.to_square == 0x0100_0000_0000_0000'u64
            ),
        'q': game.possible_castles['q'] and
            not (not game.player and move.piece == 'K') and
            not (
                not game.player and
                move.piece == 'R' and
                move.from_square == 0x8000_0000_0000_0000'u64
            ) and
            not (
                game.player and
                is_capturing == 'R' and
                move.to_square == 0x8000_0000_0000_0000'u64
            )
    }.toTable

    var position_counts: Table[int, int]
    if not (
        is_capturing != '0' or
        move.is_promoting_to != '0' or
        move.is_castling != '0'
    ):
        position_counts = initTable[int, int]()
    else:
        let key = game.id()
        position_counts = game.position_counts
        position_counts[key] = game.position_counts.getOrDefault(key, 0) + 1

    result = newGame(
        position=new_position,
        player=not game.player,
        last_move=move,
        possible_castles=possible_castles,
        en_passant_square=move.en_passant_square,
        position_counts=position_counts,
        move_counter=game.move_counter + (if game.player: 0 else: 1),
        fifty_move_counter=(
            if move.piece == 'P' or is_capturing != '0' or move.is_capturing_en_passant: 0
            else: game.fifty_move_counter + 1
        )
    )


proc legal_moves*(game: Game): seq[Move] {.exportpy.} =
    let friendly_pieces = (
        if game.player: game.position.white_pieces
        else: game.position.black_pieces
    )
    let enemy_pieces = (
        if game.player: game.position.black_pieces
        else: game.position.white_pieces
    )
    let empty_squares = 0xFFFF_FFFF_FFFF_FFFF'u64 xor game.position.all_pieces
    let attacked_squares = game.position.attacked_squares(
        player=not game.player, exclude_king=true
    )

    let king = game.position.pieces[if game.player: 'K' else: 'k']
    var king_moves = KING_MOVES[king] and (0xFFFF_FFFF_FFFF_FFFF'u64 xor attacked_squares)
    king_moves = king_moves xor (king_moves and friendly_pieces)
    for to_square in split(king_moves):
        result.add(
            newMove(
                player=game.player,
                piece='K',
                from_square=king,
                to_square=to_square
            )
        )

    var attackers: seq[uint64] = @[]
    for attacker in game.position.checkers(player=not game.player, king=king):
        attackers.add(attacker)

    let number_of_attackers = len(attackers)
    if number_of_attackers > 1:
        # Multiple pieces are giving check, so the king has to move
        return

    var capture_mask = 0xFFFF_FFFF_FFFF_FFFF'u64
    var push_mask = 0xFFFF_FFFF_FFFF_FFFF'u64
    if number_of_attackers == 1:
        let attacker = attackers[0]
        capture_mask = attacker
        if ((attacker and game.position.pieces[if game.player: 'n' else: 'N']) != 0) or
            ((attacker and game.position.pieces[if game.player: 'p' else: 'P']) != 0):
            # checked by knight or pawn, this can't be blocked
            push_mask = 0
        else:
            # checked by slider, this can be blocked
            push_mask = (
                NORTH_MOVES[king].getOrDefault(attacker, 0) or
                SOUTH_MOVES[king].getOrDefault(attacker, 0) or
                WEST_MOVES[king].getOrDefault(attacker, 0) or
                EAST_MOVES[king].getOrDefault(attacker, 0) or
                NORTH_WEST_MOVES[king].getOrDefault(attacker, 0) or
                NORTH_EAST_MOVES[king].getOrDefault(attacker, 0) or
                SOUTH_WEST_MOVES[king].getOrDefault(attacker, 0) or
                SOUTH_EAST_MOVES[king].getOrDefault(attacker, 0)
            )

    let capture_or_push_mask = capture_mask or push_mask

    let enemy_queens = game.position.pieces[if game.player: 'q' else: 'Q']
    let enemy_queens_and_rooks = (
        enemy_queens or game.position.pieces[if game.player: 'r' else: 'R']
    )
    let enemy_queens_and_bishops = (
        enemy_queens or game.position.pieces[if game.player: 'b' else: 'B']
    )

    for from_square in split(game.position.pieces[if game.player: 'Q' else: 'q']):
        let moveable_squares = (
            capture_or_push_mask and
            (
                get_rank_and_file_moves(
                    game.position.all_pieces, enemy_pieces, from_square
                ) or
                get_diagonal_moves(
                    game.position.all_pieces, enemy_pieces, from_square
                )
            ) and
            game.position.pinned_movement(
                square=from_square,
                king=king,
                enemy_queens_and_rooks=enemy_queens_and_rooks,
                enemy_queens_and_bishops=enemy_queens_and_bishops,
            )
        )
        for to_square in split(moveable_squares):
            result.add(
                newMove(
                    player=game.player,
                    piece='Q',
                    from_square=from_square,
                    to_square=to_square
                )
            )

    for from_square in split(game.position.pieces[if game.player: 'R' else: 'r']):
        let moveable_squares = (
            capture_or_push_mask and
            get_rank_and_file_moves(
                game.position.all_pieces, enemy_pieces, from_square
            ) and
            game.position.pinned_movement(
                square=from_square,
                king=king,
                enemy_queens_and_rooks=enemy_queens_and_rooks,
                enemy_queens_and_bishops=enemy_queens_and_bishops,
            )
        )
        for to_square in split(moveable_squares):
            result.add(
                newMove(
                    player=game.player,
                    piece='R',
                    from_square=from_square,
                    to_square=to_square
                )
            )

    for from_square in split(game.position.pieces[if game.player: 'B' else: 'b']):
        let moveable_squares = (
            capture_or_push_mask and
            get_diagonal_moves(
                game.position.all_pieces, enemy_pieces, from_square
            ) and
            game.position.pinned_movement(
                square=from_square,
                king=king,
                enemy_queens_and_rooks=enemy_queens_and_rooks,
                enemy_queens_and_bishops=enemy_queens_and_bishops,
            )
        )
        for to_square in split(moveable_squares):
            result.add(
                newMove(
                    player=game.player,
                    piece='B',
                    from_square=from_square,
                    to_square=to_square
                )
            )

    for from_square in split(game.position.pieces[if game.player: 'N' else: 'n']):
        let moveable_squares = (
            capture_or_push_mask and
            KNIGHT_MOVES[from_square] and
            (KNIGHT_MOVES[from_square] xor friendly_pieces) and
            game.position.pinned_movement(
                square=from_square,
                king=king,
                enemy_queens_and_rooks=enemy_queens_and_rooks,
                enemy_queens_and_bishops=enemy_queens_and_bishops,
            )
        )
        for to_square in split(moveable_squares):
            result.add(
                newMove(
                    player=game.player,
                    piece='N',
                    from_square=from_square,
                    to_square=to_square
                )
            )

    for from_square in split(game.position.pieces[if game.player: 'P' else: 'p']):
        var to_square: uint64

        let pinned_movement = game.position.pinned_movement(
            square=from_square,
            king=king,
            enemy_queens_and_rooks=enemy_queens_and_rooks,
            enemy_queens_and_bishops=enemy_queens_and_bishops,
        )
        to_square = (
            PAWN_SINGLE_MOVES[game.player][from_square] and
            empty_squares and
            pinned_movement and
            push_mask
        )
        if to_square != 0:
            if (to_square and (
                if game.player: 0xFF00_0000_0000_0000'u64
                else: 0x0000_0000_0000_00FF'u64
            )) != 0:
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square,
                        is_promoting_to='Q'
                    )
                )
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square,
                        is_promoting_to='R'
                    )
                )
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square,
                        is_promoting_to='B'
                    )
                )
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square,
                        is_promoting_to='N'
                    )
                )
            else:
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square
                    )
                )

        var attacks: seq[uint64] = @[]
        for p in PAWN_ATTACK_MOVES[game.player][from_square]:
            attacks.add(p and enemy_pieces and pinned_movement and capture_mask)
        for to_square in attacks:
            if to_square == 0:
                continue
            if (to_square and (
                if game.player: 0xFF00_0000_0000_0000'u64
                else: 0x0000_0000_0000_00FF'u64
            )) != 0:
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square,
                        is_promoting_to='Q'
                    )
                )
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square,
                        is_promoting_to='R'
                    )
                )
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square,
                        is_promoting_to='B'
                    )
                )
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square,
                        is_promoting_to='N'
                    )
                )
            else:
                result.add(
                    newMove(
                        player=game.player,
                        piece='P',
                        from_square=from_square,
                        to_square=to_square
                    )
                )

        to_square = (
            PAWN_DOUBLE_MOVES[game.player][from_square] and
            empty_squares and
            (
                if game.player: get_top_square(empty_squares)
                else: get_bottom_square(empty_squares)
            ) and
            pinned_movement and
            push_mask
        )
        if to_square!=0:
            result.add(
                newMove(
                    player=game.player,
                    piece='P',
                    from_square=from_square,
                    to_square=to_square,
                    en_passant_square=(
                        if game.player: get_bottom_square(to_square)
                        else: get_top_square(to_square)
                    )
                )
            )

        to_square = (
            PAWN_EN_PASSANT_CAPTURES[game.player][from_square] and
            game.en_passant_square and
            pinned_movement and
            (
                if game.player: get_top_square(capture_mask)
                else: get_bottom_square(capture_mask)
            )
        )
        if to_square != 0:
            let move = newMove(
                player=game.player,
                piece='P',
                from_square=from_square,
                to_square=to_square,
                is_capturing_en_passant=true
            )
            let position = game.position.move(move)[0]
            if not position.is_check(game.player):
                result.add(move)

    let can_castle_kingside = (
        game.possible_castles[if game.player: 'K' else: 'k'] and
        (
            game.position.all_pieces and (
                if game.player: 0x0000_0000_0000_0006'u64
                else: 0x0600_0000_0000_0000'u64
            )
        ) == 0 and
        (
            attacked_squares and (
                if game.player: 0x0000_0000_0000_000E'u64
                else: 0x0E00_0000_0000_0000'u64
            )
        ) == 0
    )

    if can_castle_kingside:
        result.add(
            newMove(
                player=game.player,
                piece='K',
                from_square=if game.player: 0x0000_0000_0000_0008'u64 else: 0x0800_0000_0000_0000'u64,
                to_square=if game.player: 0x0000_0000_0000_0002'u64 else: 0x0200_0000_0000_0000'u64,
                is_castling=if game.player: 'K' else: 'k'
            )
        )

    let can_castle_queenside = (
        game.possible_castles[if game.player: 'Q' else: 'q'] and
        (
            game.position.all_pieces and (
                if game.player: 0x0000_0000_0000_0070'u64
                else: 0x7000_0000_0000_0000'u64
            )
        ) == 0 and
        (
            attacked_squares and (
                if game.player: 0x0000_0000_0000_0038'u64
                else: 0x3800_0000_0000_0000'u64
            )
        ) == 0
    )

    if can_castle_queenside:
        result.add(
            newMove(
                player=game.player,
                piece='K',
                from_square=if game.player: 0x0000_0000_0000_0008'u64 else: 0x0800_0000_0000_0000'u64,
                to_square=if game.player: 0x0000_0000_0000_0020'u64 else: 0x2000_0000_0000_0000'u64,
                is_castling=if game.player: 'Q' else: 'q',
            )
        )


proc count_legal_moves(game: Game, depth: int = 1): int {.exportpy.} =
    if depth == 0:
        return 1

    var sum = 0
    for move in game.legal_moves():
        let next_game = game.move(move)
        let add = next_game.count_legal_moves(depth - 1)
        # if depth == 1:
        #     print(next_game.last_move.__str__() + ":", add)
        sum += add

    return sum

proc result*(game: Game, legal_moves: int): string {.exportpy.} =
    if legal_moves == 0:
        if game.position.is_check(game.player):
            return if game.player: RESULT_BLACK else: RESULT_WHITE
        return RESULT_STALEMATE

    if game.fifty_move_counter >= 100:
        return RESULT_FIFTY_MOVE_RULE

    for count in game.position_counts.values:
        if count >= 3:
            return RESULT_REPITITION

    if game.position.is_dead():
        return RESULT_DEAD_POSITION

    return ""


proc game_from_fen*(fen: string): Game {.exportpy.} =
    let fen_parts = fen.split(" ")
    var position = newPosition(K=0, Q=0, R=0, B=0, N=0, P=0, k=0, q=0, r=0, b=0, n=0, p=0)

    let board = fen_parts[0].split("/")
    for rankIndex, rank in board:
        var fileIndex = 0
        var r = rank
        while r != "":
            let piece = r[0]
            if piece.isDigit:
                fileIndex += parseInt($piece)
            else:
                let square = SQUARES[rankIndex * 8 + fileIndex]
                position.pieces[piece] = position.pieces[piece] or square
                if piece == toLowerAscii(piece):
                    position.black_pieces = position.black_pieces or square
                else:
                    position.white_pieces = position.white_pieces or square
                position.all_pieces = position.all_pieces or square
                fileIndex += 1
            r = r[1 .. r.len - 1]

    var en_passant_square = 0x0000_0000_0000_0000'u64
    for (square, human) in SQUARES_TO_HUMAN.pairs:
        if human == fen_parts[3]:
            en_passant_square = square

    return newGame(
        position=position,
        player=fen_parts[1] == "w",
        last_move=newMove(player=false, piece='K', from_square=0, to_square=0),
        possible_castles={
            'K': "K" in fen_parts[2],
            'Q': "Q" in fen_parts[2],
            'k': "k" in fen_parts[2],
            'q': "q" in fen_parts[2],
        }.toTable,
        en_passant_square=en_passant_square,
        position_counts=initTable[int, int](),
        move_counter=fen_parts[5].parseInt,
        fifty_move_counter=fen_parts[4].parseInt
    )
