module Chess

include("./bitboard.jl")
include("./constants.jl")

using .Bitboard: get_bottom_square, get_top_square, split_bitboard
using .Constants:
    NORTH_RAY,
    SOUTH_RAY,
    WEST_RAY,
    EAST_RAY,
    NORTH_WEST_RAY,
    NORTH_EAST_RAY,
    SOUTH_WEST_RAY,
    SOUTH_EAST_RAY,
    NORTH_MOVES,
    SOUTH_MOVES,
    WEST_MOVES,
    EAST_MOVES,
    NORTH_WEST_MOVES,
    NORTH_EAST_MOVES,
    SOUTH_WEST_MOVES,
    SOUTH_EAST_MOVES,
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
    PAWN_SINGLE_MOVES,
    PAWN_DOUBLE_MOVES,
    PAWN_ATTACK_MOVES,
    PAWN_EN_PASSANT_CAPTURES

SQUARES = [
    0x8000_0000_0000_0000,
    0x4000_0000_0000_0000,
    0x2000_0000_0000_0000,
    0x1000_0000_0000_0000,
    0x0800_0000_0000_0000,
    0x0400_0000_0000_0000,
    0x0200_0000_0000_0000,
    0x0100_0000_0000_0000,
    0x0080_0000_0000_0000,
    0x0040_0000_0000_0000,
    0x0020_0000_0000_0000,
    0x0010_0000_0000_0000,
    0x0008_0000_0000_0000,
    0x0004_0000_0000_0000,
    0x0002_0000_0000_0000,
    0x0001_0000_0000_0000,
    0x0000_8000_0000_0000,
    0x0000_4000_0000_0000,
    0x0000_2000_0000_0000,
    0x0000_1000_0000_0000,
    0x0000_0800_0000_0000,
    0x0000_0400_0000_0000,
    0x0000_0200_0000_0000,
    0x0000_0100_0000_0000,
    0x0000_0080_0000_0000,
    0x0000_0040_0000_0000,
    0x0000_0020_0000_0000,
    0x0000_0010_0000_0000,
    0x0000_0008_0000_0000,
    0x0000_0004_0000_0000,
    0x0000_0002_0000_0000,
    0x0000_0001_0000_0000,
    0x0000_0000_8000_0000,
    0x0000_0000_4000_0000,
    0x0000_0000_2000_0000,
    0x0000_0000_1000_0000,
    0x0000_0000_0800_0000,
    0x0000_0000_0400_0000,
    0x0000_0000_0200_0000,
    0x0000_0000_0100_0000,
    0x0000_0000_0080_0000,
    0x0000_0000_0040_0000,
    0x0000_0000_0020_0000,
    0x0000_0000_0010_0000,
    0x0000_0000_0008_0000,
    0x0000_0000_0004_0000,
    0x0000_0000_0002_0000,
    0x0000_0000_0001_0000,
    0x0000_0000_0000_8000,
    0x0000_0000_0000_4000,
    0x0000_0000_0000_2000,
    0x0000_0000_0000_1000,
    0x0000_0000_0000_0800,
    0x0000_0000_0000_0400,
    0x0000_0000_0000_0200,
    0x0000_0000_0000_0100,
    0x0000_0000_0000_0080,
    0x0000_0000_0000_0040,
    0x0000_0000_0000_0020,
    0x0000_0000_0000_0010,
    0x0000_0000_0000_0008,
    0x0000_0000_0000_0004,
    0x0000_0000_0000_0002,
    0x0000_0000_0000_0001
]

const SQUARES_TO_HUMAN = Dict(
    0x8000_0000_0000_0000=>"a8",
    0x4000_0000_0000_0000=>"b8",
    0x2000_0000_0000_0000=>"c8",
    0x1000_0000_0000_0000=>"d8",
    0x0800_0000_0000_0000=>"e8",
    0x0400_0000_0000_0000=>"f8",
    0x0200_0000_0000_0000=>"g8",
    0x0100_0000_0000_0000=>"h8",
    0x0080_0000_0000_0000=>"a7",
    0x0040_0000_0000_0000=>"b7",
    0x0020_0000_0000_0000=>"c7",
    0x0010_0000_0000_0000=>"d7",
    0x0008_0000_0000_0000=>"e7",
    0x0004_0000_0000_0000=>"f7",
    0x0002_0000_0000_0000=>"g7",
    0x0001_0000_0000_0000=>"h7",
    0x0000_8000_0000_0000=>"a6",
    0x0000_4000_0000_0000=>"b6",
    0x0000_2000_0000_0000=>"c6",
    0x0000_1000_0000_0000=>"d6",
    0x0000_0800_0000_0000=>"e6",
    0x0000_0400_0000_0000=>"f6",
    0x0000_0200_0000_0000=>"g6",
    0x0000_0100_0000_0000=>"h6",
    0x0000_0080_0000_0000=>"a5",
    0x0000_0040_0000_0000=>"b5",
    0x0000_0020_0000_0000=>"c5",
    0x0000_0010_0000_0000=>"d5",
    0x0000_0008_0000_0000=>"e5",
    0x0000_0004_0000_0000=>"f5",
    0x0000_0002_0000_0000=>"g5",
    0x0000_0001_0000_0000=>"h5",
    0x0000_0000_8000_0000=>"a4",
    0x0000_0000_4000_0000=>"b4",
    0x0000_0000_2000_0000=>"c4",
    0x0000_0000_1000_0000=>"d4",
    0x0000_0000_0800_0000=>"e4",
    0x0000_0000_0400_0000=>"f4",
    0x0000_0000_0200_0000=>"g4",
    0x0000_0000_0100_0000=>"h4",
    0x0000_0000_0080_0000=>"a3",
    0x0000_0000_0040_0000=>"b3",
    0x0000_0000_0020_0000=>"c3",
    0x0000_0000_0010_0000=>"d3",
    0x0000_0000_0008_0000=>"e3",
    0x0000_0000_0004_0000=>"f3",
    0x0000_0000_0002_0000=>"g3",
    0x0000_0000_0001_0000=>"h3",
    0x0000_0000_0000_8000=>"a2",
    0x0000_0000_0000_4000=>"b2",
    0x0000_0000_0000_2000=>"c2",
    0x0000_0000_0000_1000=>"d2",
    0x0000_0000_0000_0800=>"e2",
    0x0000_0000_0000_0400=>"f2",
    0x0000_0000_0000_0200=>"g2",
    0x0000_0000_0000_0100=>"h2",
    0x0000_0000_0000_0080=>"a1",
    0x0000_0000_0000_0040=>"b1",
    0x0000_0000_0000_0020=>"c1",
    0x0000_0000_0000_0010=>"d1",
    0x0000_0000_0000_0008=>"e1",
    0x0000_0000_0000_0004=>"f1",
    0x0000_0000_0000_0002=>"g1",
    0x0000_0000_0000_0001=>"h1"
)

RESULT_WHITE = "White wins"
RESULT_BLACK = "Black wins"
RESULT_STALEMATE = "Stalemate"
RESULT_DEAD_POSITION = "Dead position"
RESULT_REPITITION = "Third repitition of position"
RESULT_FIFTY_MOVE_RULE = "Fifty moves without capture or pawn movement"


function get_rank_and_file_moves(all_pieces::UInt64, enemy_pieces::UInt64, square::UInt64)::UInt64
    north_pieces = NORTH_RAY[square] & all_pieces
    south_pieces = SOUTH_RAY[square] & all_pieces
    west_pieces = WEST_RAY[square] & all_pieces
    east_pieces = EAST_RAY[square] & all_pieces

    north_moves = NORTH_MOVES[square][north_pieces] ⊻ (
        NORTH_ATTACKS[square][north_pieces] & enemy_pieces
    )
    south_moves = SOUTH_MOVES[square][south_pieces] ⊻ (
        SOUTH_ATTACKS[square][south_pieces] & enemy_pieces
    )
    west_moves = WEST_MOVES[square][west_pieces] ⊻ (
        WEST_ATTACKS[square][west_pieces] & enemy_pieces
    )
    east_moves = EAST_MOVES[square][east_pieces] ⊻ (
        EAST_ATTACKS[square][east_pieces] & enemy_pieces
    )

    return north_moves | south_moves | west_moves | east_moves
end

function get_diagonal_moves(all_pieces::UInt64, enemy_pieces::UInt64, square::UInt64)::UInt64
    north_west_pieces = NORTH_WEST_RAY[square] & all_pieces
    south_west_pieces = SOUTH_WEST_RAY[square] & all_pieces
    north_east_pieces = NORTH_EAST_RAY[square] & all_pieces
    south_east_pieces = SOUTH_EAST_RAY[square] & all_pieces

    north_west_moves = NORTH_WEST_MOVES[square][north_west_pieces] ⊻ (
        NORTH_WEST_ATTACKS[square][north_west_pieces] & enemy_pieces
    )
    north_east_moves = NORTH_EAST_MOVES[square][north_east_pieces] ⊻ (
        NORTH_EAST_ATTACKS[square][north_east_pieces] & enemy_pieces
    )
    south_west_moves = SOUTH_WEST_MOVES[square][south_west_pieces] ⊻ (
        SOUTH_WEST_ATTACKS[square][south_west_pieces] & enemy_pieces
    )
    south_east_moves = SOUTH_EAST_MOVES[square][south_east_pieces] ⊻ (
        SOUTH_EAST_ATTACKS[square][south_east_pieces] & enemy_pieces
    )

    return north_west_moves | north_east_moves | south_west_moves | south_east_moves
end

struct Move
    player::Bool
    piece::Char
    from_square::UInt64
    to_square::UInt64
    en_passant_square::UInt64
    is_capturing_en_passant::Bool
    is_castling::Char
    is_promoting_to::Char
end

function new_move(
    player::Bool,
    piece::Char,
    from_square::UInt64,
    to_square::UInt64;
    en_passant_square::UInt64=0x0000_0000_0000_0000,
    is_capturing_en_passant::Bool=false,
    is_castling::Char='0',
    is_promoting_to::Char='0'
)::Move
    return Move(
        player,
        piece,
        from_square,
        to_square,
        en_passant_square,
        is_capturing_en_passant,
        is_castling,
        is_promoting_to
    )
end

function to_string(move::Move)::string
    result = SQUARES_TO_HUMAN[move.from_square] & SQUARES_TO_HUMAN[move.to_square]
    if move.is_promoting_to != '0'
        result += string(move.is_promoting_to)
    end
    return result
end


struct Position
    all_pieces::UInt64
    white_pieces::UInt64
    black_pieces::UInt64
    pieces::Dict{Char, UInt64}
end

function new_position(
    K::UInt64,
    Q::UInt64,
    R::UInt64,
    B::UInt64,
    N::UInt64,
    P::UInt64,
    k::UInt64,
    q::UInt64,
    r::UInt64,
    b::UInt64,
    n::UInt64,
    p::UInt64,
)::Position
    white_pieces = K | Q | R | B | N | P
    black_pieces = k | q | r | b | n | p
    pieces = Dict(
        'K'=>K,
        'Q'=>Q,
        'R'=>R,
        'B'=>B,
        'N'=>N,
        'P'=>P,
        'k'=>k,
        'q'=>q,
        'r'=>r,
        'b'=>b,
        'n'=>n,
        'p'=>p,
    )
    return Position(white_pieces | black_pieces, white_pieces, black_pieces, pieces)
end

function do_move(position::Position, move::Move)::Tuple{Position, Char}
    pieces = Dict(
        'K'=>position.pieces['K'],
        'Q'=>position.pieces['Q'],
        'R'=>position.pieces['R'],
        'B'=>position.pieces['B'],
        'N'=>position.pieces['N'],
        'P'=>position.pieces['P'],
        'k'=>position.pieces['k'],
        'q'=>position.pieces['q'],
        'r'=>position.pieces['r'],
        'b'=>position.pieces['b'],
        'n'=>position.pieces['n'],
        'p'=>position.pieces['p'],
    )

    if move.is_castling == 'K'
        pieces['K'] = 0x0000_0000_0000_0002
        pieces['R'] ⊻= 0x0000_0000_0000_0005
        return (new_position(
            pieces['K'], pieces['Q'], pieces['R'], pieces['B'], pieces['N'], pieces['P'],
            pieces['k'], pieces['q'], pieces['r'], pieces['b'], pieces['n'], pieces['p']
        ), '0')
    end
    if move.is_castling == 'Q'
        pieces['K'] = 0x0000_0000_0000_0020
        pieces['R'] ⊻= 0x0000_0000_0000_0090
        return (new_position(
            pieces['K'], pieces['Q'], pieces['R'], pieces['B'], pieces['N'], pieces['P'],
            pieces['k'], pieces['q'], pieces['r'], pieces['b'], pieces['n'], pieces['p']
        ), '0')
    end
    if move.is_castling == 'k'
        pieces['k'] = 0x0200_0000_0000_0000
        pieces['r'] ⊻= 0x0500_0000_0000_0000
        return (new_position(
            pieces['K'], pieces['Q'], pieces['R'], pieces['B'], pieces['N'], pieces['P'],
            pieces['k'], pieces['q'], pieces['r'], pieces['b'], pieces['n'], pieces['p']
        ), '0')
    end
    if move.is_castling == 'q'
        pieces['k'] = 0x2000_0000_0000_0000
        pieces['r'] ⊻= 0x9000_0000_0000_0000
        return (new_position(
            pieces['K'], pieces['Q'], pieces['R'], pieces['B'], pieces['N'], pieces['P'],
            pieces['k'], pieces['q'], pieces['r'], pieces['b'], pieces['n'], pieces['p']
        ), '0')
    end

    is_capturing = '0'
    if move.to_square & pieces['P'] != 0
        is_capturing = 'P'
    elseif move.to_square & pieces['p'] != 0
        is_capturing = 'P'
    elseif move.to_square & pieces['N'] != 0
        is_capturing = 'N'
    elseif move.to_square & pieces['n'] != 0
        is_capturing = 'N'
    elseif move.to_square & pieces['B'] != 0
        is_capturing = 'B'
    elseif move.to_square & pieces['b'] != 0
        is_capturing = 'B'
    elseif move.to_square & pieces['R'] != 0
        is_capturing = 'R'
    elseif move.to_square & pieces['r'] != 0
        is_capturing = 'R'
    elseif move.to_square & pieces['Q'] != 0
        is_capturing = 'Q'
    elseif move.to_square & pieces['q'] != 0
        is_capturing = 'Q'
    end

    piece_key = move.player ? move.piece : lowercase(move.piece)
    pieces[piece_key] = (pieces[piece_key] ⊻ move.from_square) | move.to_square

    if is_capturing != '0'
        is_capturing_key = move.player ? lowercase(is_capturing) : is_capturing
        pieces[is_capturing_key] = pieces[is_capturing_key] ⊻ move.to_square
    end

    if move.is_capturing_en_passant
        captured_square = move.player ? get_bottom_square(move.to_square) : get_top_square(move.to_square)
        pawn_key = move.player ? 'p' : 'P'
        pieces[pawn_key] = pieces[pawn_key] ⊻ captured_square
    end

    if move.is_promoting_to != '0'
        is_promoting_to_key = move.player ? move.is_promoting_to : lowercase(move.is_promoting_to)
        pawn_key = move.player ? 'P' : 'p'
        pieces[is_promoting_to_key] = pieces[is_promoting_to_key] | move.to_square
        pieces[pawn_key] = pieces[pawn_key] ⊻ move.to_square
    end

    return (new_position(
        pieces['K'], pieces['Q'], pieces['R'], pieces['B'], pieces['N'], pieces['P'],
        pieces['k'], pieces['q'], pieces['r'], pieces['b'], pieces['n'], pieces['p']
    ), is_capturing)
end

function attacking_squares(position::Position, player::Bool, square::UInt64)::UInt64
    king = position.pieces[player ? 'K' : 'k']
    queen = position.pieces[player ? 'Q' : 'q']
    rook = position.pieces[player ? 'R' : 'r']
    bishop = position.pieces[player ? 'B' : 'b']
    knight = position.pieces[player ? 'N' : 'n']
    pawn = position.pieces[player ? 'P' : 'p']

    queen_and_rook = queen | rook
    queen_and_bishop = queen | bishop

    north_pieces = NORTH_RAY[square] & position.all_pieces
    south_pieces = SOUTH_RAY[square] & position.all_pieces
    west_pieces = WEST_RAY[square] & position.all_pieces
    east_pieces = EAST_RAY[square] & position.all_pieces
    north_west_pieces = NORTH_WEST_RAY[square] & position.all_pieces
    south_west_pieces = SOUTH_WEST_RAY[square] & position.all_pieces
    north_east_pieces = NORTH_EAST_RAY[square] & position.all_pieces
    south_east_pieces = SOUTH_EAST_RAY[square] & position.all_pieces

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
end

function checkers(position::Position, player::Bool, king::UInt64)::Vector{UInt64}
    queen = position.pieces[player ? 'Q' : 'q']
    rook = position.pieces[player ? 'R' : 'r']
    bishop = position.pieces[player ? 'B' : 'b']
    knight = position.pieces[player ? 'N' : 'n']
    pawn = position.pieces[player ? 'P' : 'p']

    queen_and_rook = queen | rook
    queen_and_bishop = queen | bishop

    north_pieces = NORTH_RAY[king] & position.all_pieces
    south_pieces = SOUTH_RAY[king] & position.all_pieces
    west_pieces = WEST_RAY[king] & position.all_pieces
    east_pieces = EAST_RAY[king] & position.all_pieces
    north_west_pieces = NORTH_WEST_RAY[king] & position.all_pieces
    south_west_pieces = SOUTH_WEST_RAY[king] & position.all_pieces
    north_east_pieces = NORTH_EAST_RAY[king] & position.all_pieces
    south_east_pieces = SOUTH_EAST_RAY[king] & position.all_pieces

    return split_bitboard(
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
end

function attacked_squares(position::Position, player::Bool, exclude_king::Bool=false)::UInt64
    king_key = player ? 'K' : 'k'
    queen_key = player ? 'Q' : 'q'
    rook_key = player ? 'R' : 'r'
    bishop_key = player ? 'B' : 'b'
    knight_key = player ? 'N' : 'n'
    pawn_key = player ? 'P' : 'p'

    all_pieces = position.all_pieces
    if exclude_king
        all_pieces ⊻= position.pieces[player ? 'k' : 'K']
    end

    attacked = KING_MOVES[position.pieces[king_key]]

    for queen in split_bitboard(position.pieces[queen_key])
        north_pieces = NORTH_RAY[queen] & all_pieces
        attacked |= NORTH_MOVES[queen][north_pieces] | NORTH_ATTACKS[queen][north_pieces]
        south_pieces = SOUTH_RAY[queen] & all_pieces
        attacked |= SOUTH_MOVES[queen][south_pieces] | SOUTH_ATTACKS[queen][south_pieces]
        west_pieces = WEST_RAY[queen] & all_pieces
        attacked |= WEST_MOVES[queen][west_pieces] | WEST_ATTACKS[queen][west_pieces]
        east_pieces = EAST_RAY[queen] & all_pieces
        attacked |= EAST_MOVES[queen][east_pieces] | EAST_ATTACKS[queen][east_pieces]
        north_west_pieces = NORTH_WEST_RAY[queen] & all_pieces
        attacked |= NORTH_WEST_MOVES[queen][north_west_pieces] | NORTH_WEST_ATTACKS[queen][north_west_pieces]
        north_east_pieces = NORTH_EAST_RAY[queen] & all_pieces
        attacked |= NORTH_EAST_MOVES[queen][north_east_pieces] | NORTH_EAST_ATTACKS[queen][north_east_pieces]
        south_west_pieces = SOUTH_WEST_RAY[queen] & all_pieces
        attacked |= SOUTH_WEST_MOVES[queen][south_west_pieces] | SOUTH_WEST_ATTACKS[queen][south_west_pieces]
        south_east_pieces = SOUTH_EAST_RAY[queen] & all_pieces
        attacked |= SOUTH_EAST_MOVES[queen][south_east_pieces] | SOUTH_EAST_ATTACKS[queen][south_east_pieces]
    end

    for rook in split_bitboard(position.pieces[rook_key])
        north_pieces = NORTH_RAY[rook] & all_pieces
        attacked |= NORTH_MOVES[rook][north_pieces] | NORTH_ATTACKS[rook][north_pieces]
        south_pieces = SOUTH_RAY[rook] & all_pieces
        attacked |= SOUTH_MOVES[rook][south_pieces] | SOUTH_ATTACKS[rook][south_pieces]
        west_pieces = WEST_RAY[rook] & all_pieces
        attacked |= WEST_MOVES[rook][west_pieces] | WEST_ATTACKS[rook][west_pieces]
        east_pieces = EAST_RAY[rook] & all_pieces
        attacked |= EAST_MOVES[rook][east_pieces] | EAST_ATTACKS[rook][east_pieces]
    end

    for bishop in split_bitboard(position.pieces[bishop_key])
        north_west_pieces = NORTH_WEST_RAY[bishop] & all_pieces
        attacked |= NORTH_WEST_MOVES[bishop][north_west_pieces] | NORTH_WEST_ATTACKS[bishop][north_west_pieces]
        north_east_pieces = NORTH_EAST_RAY[bishop] & all_pieces
        attacked |= NORTH_EAST_MOVES[bishop][north_east_pieces] | NORTH_EAST_ATTACKS[bishop][north_east_pieces]
        south_west_pieces = SOUTH_WEST_RAY[bishop] & all_pieces
        attacked |= SOUTH_WEST_MOVES[bishop][south_west_pieces] | SOUTH_WEST_ATTACKS[bishop][south_west_pieces]
        south_east_pieces = SOUTH_EAST_RAY[bishop] & all_pieces
        attacked |= SOUTH_EAST_MOVES[bishop][south_east_pieces] | SOUTH_EAST_ATTACKS[bishop][south_east_pieces]
    end

    for knight in split_bitboard(position.pieces[knight_key])
        attacked |= KNIGHT_MOVES[knight]
    end

    for pawn in split_bitboard(position.pieces[pawn_key])
        for s in PAWN_ATTACK_MOVES[player][pawn]
            attacked |= s
        end
    end

    return attacked
end

function is_check(position::Position, player::Bool)::Bool
    return attacking_squares(position, !player, position.pieces[player ? 'K' : 'k']) != 0
end

function pinned_movement(
    position::Position,
    square::UInt64,
    king::UInt64,
    enemy_queens_and_rooks::UInt64,
    enemy_queens_and_bishops::UInt64,
)::UInt64
    north_pieces = NORTH_RAY[square] & position.all_pieces
    south_pieces = SOUTH_RAY[square] & position.all_pieces
    first_piece_to_north = NORTH_ATTACKS[square][north_pieces]
    first_piece_to_south = SOUTH_ATTACKS[square][south_pieces]

    is_pinned_from_north = first_piece_to_south == king && (first_piece_to_north & enemy_queens_and_rooks != 0)
    if is_pinned_from_north
        return (
            first_piece_to_north
            | NORTH_MOVES[square][north_pieces]
            | SOUTH_MOVES[square][south_pieces]
        )
    end

    is_pinned_from_south = first_piece_to_north == king && (first_piece_to_south & enemy_queens_and_rooks != 0)
    if is_pinned_from_south
        return (
            first_piece_to_south
            | SOUTH_MOVES[square][south_pieces]
            | NORTH_MOVES[square][north_pieces]
        )
    end

    west_pieces = WEST_RAY[square] & position.all_pieces
    east_pieces = EAST_RAY[square] & position.all_pieces
    first_piece_to_west = WEST_ATTACKS[square][west_pieces]
    first_piece_to_east = EAST_ATTACKS[square][east_pieces]

    is_pinned_from_west = first_piece_to_east == king && (first_piece_to_west & enemy_queens_and_rooks != 0)
    if is_pinned_from_west
        return (
            first_piece_to_west
            | WEST_MOVES[square][west_pieces]
            | EAST_MOVES[square][east_pieces]
        )
    end

    is_pinned_from_east = first_piece_to_west == king && (first_piece_to_east & enemy_queens_and_rooks != 0)
    if is_pinned_from_east
        return (
            first_piece_to_east
            | EAST_MOVES[square][east_pieces]
            | WEST_MOVES[square][west_pieces]
        )
    end

    north_west_pieces = NORTH_WEST_RAY[square] & position.all_pieces
    south_east_pieces = SOUTH_EAST_RAY[square] & position.all_pieces
    first_piece_to_north_west = NORTH_WEST_ATTACKS[square][north_west_pieces]
    first_piece_to_south_east = SOUTH_EAST_ATTACKS[square][south_east_pieces]

    is_pinned_from_north_west = first_piece_to_south_east == king && (first_piece_to_north_west & enemy_queens_and_bishops != 0)
    if is_pinned_from_north_west
        return (
            first_piece_to_north_west
            | NORTH_WEST_MOVES[square][north_west_pieces]
            | SOUTH_EAST_MOVES[square][south_east_pieces]
        )
    end

    is_pinned_from_south_east = first_piece_to_north_west == king && (first_piece_to_south_east & enemy_queens_and_bishops != 0)
    if is_pinned_from_south_east
        return (
            first_piece_to_south_east
            | SOUTH_EAST_MOVES[square][south_east_pieces]
            | NORTH_WEST_MOVES[square][north_west_pieces]
        )
    end

    north_east_pieces = NORTH_EAST_RAY[square] & position.all_pieces
    south_west_pieces = SOUTH_WEST_RAY[square] & position.all_pieces
    first_piece_to_north_east = NORTH_EAST_ATTACKS[square][north_east_pieces]
    first_piece_to_south_west = SOUTH_WEST_ATTACKS[square][south_west_pieces]

    is_pinned_from_north_east = first_piece_to_south_west == king && (first_piece_to_north_east & enemy_queens_and_bishops != 0)
    if is_pinned_from_north_east
        return (
            first_piece_to_north_east
            | NORTH_EAST_MOVES[square][north_east_pieces]
            | SOUTH_WEST_MOVES[square][south_west_pieces]
        )
    end

    is_pinned_from_south_west = first_piece_to_north_east == king && (first_piece_to_south_west & enemy_queens_and_bishops != 0)
    if is_pinned_from_south_west
        return (
            first_piece_to_south_west
            | SOUTH_WEST_MOVES[square][south_west_pieces]
            | NORTH_EAST_MOVES[square][north_east_pieces]
        )
    end

    return 0xFFFF_FFFF_FFFF_FFFF
end

function is_dead(position::Position)::Bool
    white_queens = length(split_bitboard(position.pieces['Q']))
    if white_queens > 0
        return false
    end

    black_queens = length(split_bitboard(position.pieces['q']))
    if black_queens > 0
        return false
    end

    white_rooks = length(split_bitboard(position.pieces['R']))
    if white_rooks > 0
        return false
    end

    black_rooks = length(split_bitboard(position.pieces['r']))
    if black_rooks > 0
        return false
    end

    white_pawns = length(split_bitboard(position.pieces['P']))
    if white_pawns > 0
        return false
    end

    black_pawns = length(split_bitboard(position.pieces['p']))
    if black_pawns > 0
        return false
    end

    white_bishops = split_bitboard(position.pieces['B'])
    if length(white_bishops) > 1
        return false
    end

    black_bishops = split_bitboard(position.pieces['b'])
    if length(black_bishops) > 1
        return false
    end

    white_knights = length(split_bitboard(position.pieces['N']))
    if white_knights > 1
        return false
    end

    black_knights = length(split_bitboard(position.pieces['n']))
    if black_knights > 1
        return false
    end

    number_of_white_pieces = white_queens +
        white_rooks +
        length(white_bishops) +
        white_knights +
        white_pawns
    number_of_black_pieces = black_queens +
        black_rooks +
        length(black_bishops) +
        black_knights +
        black_pawns

    # king against king
    if number_of_white_pieces + number_of_black_pieces == 0
        return true
    end

    # king against king and bishop
    if (
        number_of_white_pieces == 0
        && number_of_black_pieces == 1
        && length(black_bishops) == 1
    )
        return true
    end
    if (
        number_of_black_pieces == 0
        && number_of_white_pieces == 1
        && length(white_bishops) == 1
    )
        return true
    end

    # king against king and knight
    if (
        number_of_white_pieces == 0
        && number_of_black_pieces == 1
        && black_knights == 1
    )
        return true
    end
    if (
        number_of_black_pieces == 0
        && number_of_white_pieces == 1
        && white_knights == 1
    )
        return true
    end

    # king and bishop against king and bishop, with both bishops on squares of the same color
    if (
        number_of_white_pieces == 1
        && number_of_black_pieces == 1
        && length(white_bishops) == 1
        && length(black_bishops) == 1
    )
        is_white_bishop_on_white_square = white_bishops[1] & 0xAA55_AA55_AA55_AA55 == 0
        is_black_bishop_on_white_square = black_bishops[1] & 0xAA55_AA55_AA55_AA55 == 0
        return is_white_bishop_on_white_square == is_black_bishop_on_white_square
    end

    return false
end


struct Game
    position::Position
    player::Bool
    last_move::Move
    possible_castles::Dict{Char, Bool}
    en_passant_square::UInt64
    position_counts::Dict{String, Int}
    move_counter::Int
    fifty_move_counter::Int
end

function new_game(
    position::Position,
    player::Bool,
    last_move::Move,
    possible_castles::Dict{Char, Bool},
    en_passant_square::UInt64,
    position_counts::Dict{String, Int},
    move_counter::Int,
    fifty_move_counter::Int,
)::Game
    return Game(
        position,
        player,
        last_move,
        possible_castles,
        en_passant_square,
        position_counts,
        move_counter,
        fifty_move_counter
    )
end

function id(game::Game)::String
    pieces = game.position.pieces
    return string(
        pieces['K'], "-", pieces['Q'], "-",
        pieces['R'], "-", pieces['B'], "-",
        pieces['N'], "-", pieces['P'], "-",
        pieces['k'], "-", pieces['q'], "-",
        pieces['r'], "-", pieces['b'], "-",
        pieces['n'], "-", pieces['p'], "-",
        game.player, "-",
        (game.possible_castles['K'] ? "K" : ""), "-",
        (game.possible_castles['Q'] ? "Q" : ""), "-",
        (game.possible_castles['k'] ? "k" : ""), "-",
        (game.possible_castles['q'] ? "q" : ""), "-",
        game.en_passant_square
    )
end

function do_move(game::Game, move::Move)::Game
    new_position, is_capturing = do_move(game.position, move)

    possible_castles = Dict(
        'K'=>game.possible_castles['K']
            && !(game.player && move.piece == 'K')
            && !(game.player && move.piece == 'R' && move.from_square == 0x0000_0000_0000_0001)
            && !(!game.player && is_capturing == 'R' && move.to_square == 0x0000_0000_0000_0001),
        'Q'=>game.possible_castles['Q']
            && !(game.player && move.piece == 'K')
            && !(game.player && move.piece == 'R' && move.from_square == 0x0000_0000_0000_0080)
            && !(!game.player && is_capturing == 'R' && move.to_square == 0x0000_0000_0000_0080),
        'k'=>game.possible_castles['k']
            && !(!game.player && move.piece == 'K')
            && !(!game.player && move.piece == 'R' && move.from_square == 0x0100_0000_0000_0000)
            && !(game.player && is_capturing == 'R' && move.to_square == 0x0100_0000_0000_0000),
        'q'=>game.possible_castles['q']
            && !(!game.player && move.piece == 'K')
            && !(!game.player && move.piece == 'R' && move.from_square == 0x8000_0000_0000_0000)
            && !(game.player && is_capturing == 'R' && move.to_square == 0x8000_0000_0000_0000),
    )

    if !(is_capturing != '0' || move.is_promoting_to != '0' || move.is_castling != '0')
        position_counts = Dict{String, Int}()
    else
        key = id(game)
        position_counts = copy(game.position_counts)
        position_counts[key] = get(game.position_counts, key, 0) + 1
    end

    return new_game(
        new_position,
        !game.player,
        move,
        possible_castles,
        move.en_passant_square,
        position_counts,
        game.move_counter + (game.player ? 0 : 1),
        (move.piece == 'P' || is_capturing != '0' || move.is_capturing_en_passant ? 0 : game.fifty_move_counter + 1)
    )
end


function legal_moves(game::Game)::Vector{Move}
    result = Vector{Move}()

    friendly_pieces = game.player ? game.position.white_pieces : game.position.black_pieces
    enemy_pieces = game.player ? game.position.black_pieces : game.position.white_pieces
    empty_squares = 0xFFFF_FFFF_FFFF_FFFF ⊻ game.position.all_pieces
    attacked = attacked_squares(game.position, !game.player, true)

    king = game.position.pieces[game.player ? 'K' : 'k']
    king_moves = KING_MOVES[king] & (0xFFFF_FFFF_FFFF_FFFF ⊻ attacked)
    king_moves ⊻= (king_moves & friendly_pieces)
    for to_square in split_bitboard(king_moves)
        push!(result, new_move(game.player, 'K', king, to_square))
    end

    attackers = checkers(game.position, !game.player, king)

    number_of_attackers = length(attackers)
    if number_of_attackers > 1
        # Multiple pieces are giving check, so the king has to move
        return result
    end

    capture_mask = 0xFFFF_FFFF_FFFF_FFFF
    push_mask = 0xFFFF_FFFF_FFFF_FFFF
    if number_of_attackers == 1
        attacker = attackers[1]
        capture_mask = attacker
        if (attacker & game.position.pieces[game.player ? 'n' : 'N'] != 0) || (attacker & game.position.pieces[game.player ? 'p' : 'P'] != 0)
            # checked by knight or pawn, this can't be blocked
            push_mask = 0
        else
            # checked by slider, this can be blocked
            push_mask = (
                get(NORTH_MOVES[king], attacker, 0x0000_0000_0000_0000)
                | get(SOUTH_MOVES[king], attacker, 0x0000_0000_0000_0000)
                | get(WEST_MOVES[king], attacker, 0x0000_0000_0000_0000)
                | get(EAST_MOVES[king], attacker, 0x0000_0000_0000_0000)
                | get(NORTH_WEST_MOVES[king], attacker, 0x0000_0000_0000_0000)
                | get(NORTH_EAST_MOVES[king], attacker, 0x0000_0000_0000_0000)
                | get(SOUTH_WEST_MOVES[king], attacker, 0x0000_0000_0000_0000)
                | get(SOUTH_EAST_MOVES[king], attacker, 0x0000_0000_0000_0000)
            )
        end
    end

    capture_or_push_mask = capture_mask | push_mask

    enemy_queens = game.position.pieces[game.player ? 'q' : 'Q']
    enemy_queens_and_rooks = enemy_queens | game.position.pieces[game.player ? 'r' : 'R']
    enemy_queens_and_bishops = enemy_queens | game.position.pieces[game.player ? 'b' : 'B']

    for from_square in split_bitboard(game.position.pieces[game.player ? 'Q' : 'q'])
        moveable_squares = (
            capture_or_push_mask
            & (
                get_rank_and_file_moves(game.position.all_pieces, enemy_pieces, from_square)
                | get_diagonal_moves(game.position.all_pieces, enemy_pieces, from_square)
            )
            & pinned_movement(
                game.position,
                from_square,
                king,
                enemy_queens_and_rooks,
                enemy_queens_and_bishops,
            )
        )
        for to_square in split_bitboard(moveable_squares)
            push!(result, new_move(game.player, 'Q', from_square, to_square))
        end
    end

    for from_square in split_bitboard(game.position.pieces[game.player ? 'R' : 'r'])
        moveable_squares = (
            capture_or_push_mask
            & get_rank_and_file_moves(game.position.all_pieces, enemy_pieces, from_square)
            & pinned_movement(
                game.position,
                from_square,
                king,
                enemy_queens_and_rooks,
                enemy_queens_and_bishops,
            )
        )
        for to_square in split_bitboard(moveable_squares)
            push!(result, new_move(game.player, 'R', from_square, to_square))
        end
    end

    for from_square in split_bitboard(game.position.pieces[game.player ? 'B' : 'b'])
        moveable_squares = (
            capture_or_push_mask
            & get_diagonal_moves(game.position.all_pieces, enemy_pieces, from_square)
            & pinned_movement(
                game.position,
                from_square,
                king,
                enemy_queens_and_rooks,
                enemy_queens_and_bishops,
            )
        )
        for to_square in split_bitboard(moveable_squares)
            push!(result, new_move(game.player, 'B', from_square, to_square))
        end
    end

    for from_square in split_bitboard(game.position.pieces[game.player ? 'N' : 'n'])
        moveable_squares = (
            capture_or_push_mask
            & KNIGHT_MOVES[from_square]
            & (KNIGHT_MOVES[from_square] ⊻ friendly_pieces)
            & pinned_movement(
                game.position,
                from_square,
                king,
                enemy_queens_and_rooks,
                enemy_queens_and_bishops,
            )
        )
        for to_square in split_bitboard(moveable_squares)
            push!(result, new_move(game.player, 'N', from_square, to_square))
        end
    end

    for from_square in split_bitboard(game.position.pieces[game.player ? 'P' : 'p'])
        pins = pinned_movement(
            game.position,
            from_square,
            king,
            enemy_queens_and_rooks,
            enemy_queens_and_bishops,
        )
        to_square = PAWN_SINGLE_MOVES[game.player][from_square] & empty_squares & pins & push_mask
        if to_square != 0
            if to_square & (game.player ? 0xFF00_0000_0000_0000 : 0x0000_0000_0000_00FF) != 0
                push!(result, new_move(game.player, 'P', from_square, to_square, is_promoting_to='Q'))
                push!(result, new_move(game.player, 'P', from_square, to_square, is_promoting_to='R'))
                push!(result, new_move(game.player, 'P', from_square, to_square, is_promoting_to='B'))
                push!(result, new_move(game.player, 'P', from_square, to_square, is_promoting_to='N'))
            else
                push!(result, new_move(game.player, 'P', from_square, to_square))
            end
        end

        pawn_attacks = Vector{UInt64}()
        for p in PAWN_ATTACK_MOVES[game.player][from_square]
            append!(pawn_attacks, p & enemy_pieces & pins & capture_mask)
        end
        for to_square in pawn_attacks
            if to_square == 0
                continue
            end
            if to_square & (game.player ? 0xFF00_0000_0000_0000 : 0x0000_0000_0000_00FF) != 0
                push!(result, new_move(game.player, 'P', from_square, to_square, is_promoting_to='Q'))
                push!(result, new_move(game.player, 'P', from_square, to_square, is_promoting_to='R'))
                push!(result, new_move(game.player, 'P', from_square, to_square, is_promoting_to='B'))
                push!(result, new_move(game.player, 'P', from_square, to_square, is_promoting_to='N'))
            else
                push!(result, new_move(game.player, 'P', from_square, to_square))
            end
        end

        to_square = (
            PAWN_DOUBLE_MOVES[game.player][from_square]
            & empty_squares
            & (game.player ? get_top_square(empty_squares) : get_bottom_square(empty_squares))
            & pins
            & push_mask
        )
        if to_square != 0
            push!(result, new_move(
                game.player,
                'P',
                from_square,
                to_square,
                en_passant_square=(game.player ? get_bottom_square(to_square) : get_top_square(to_square))
            ))
        end

        to_square = (
            PAWN_EN_PASSANT_CAPTURES[game.player][from_square]
            & game.en_passant_square
            & pins
            & (game.player ? get_top_square(capture_mask) : get_bottom_square(capture_mask))
        )
        if to_square != 0
            move = new_move(
                game.player,
                'P',
                from_square,
                to_square,
                is_capturing_en_passant=true
            )
            position = do_move(game.position, move)[1]
            if !is_check(position, game.player)
                push!(result, move)
            end
        end
    end

    can_castle_kingside = (
        game.possible_castles[game.player ? 'K' : 'k']
        && game.position.all_pieces & (game.player ? 0x0000_0000_0000_0006 : 0x0600_0000_0000_0000) == 0
        && attacked & (game.player ? 0x0000_0000_0000_000E : 0x0E00_0000_0000_0000) == 0
    )

    if can_castle_kingside
        push!(result, new_move(
            game.player,
            'K',
            game.player ? 0x0000_0000_0000_0008 : 0x0800_0000_0000_0000,
            game.player ? 0x0000_0000_0000_0002 : 0x0200_0000_0000_0000,
            is_castling=game.player ? 'K' : 'k'
        ))
    end

    can_castle_queenside = (
        game.possible_castles[game.player ? 'Q' : 'q']
        && game.position.all_pieces & (game.player ? 0x0000_0000_0000_0070 : 0x7000_0000_0000_0000) == 0
        && attacked & (game.player ? 0x0000_0000_0000_0038 : 0x3800_0000_0000_0000) == 0
    )

    if can_castle_queenside
        push!(result, new_move(
            game.player,
            'K',
            game.player ? 0x0000_0000_0000_0008 : 0x0800_0000_0000_0000,
            game.player ? 0x0000_0000_0000_0020 : 0x2000_0000_0000_0000,
            is_castling=game.player ? 'Q' : 'q',
        ))
    end

    return result
end

function count_legal_moves(game::Game; depth::Int=1)::Int
    if depth == 0
        return 1
    end

    sum = 0
    for move in legal_moves(game)
        next_game = do_move(game, move)
        add = count_legal_moves(next_game, depth=depth - 1)
        # if depth == 1:
        #     print(next_game.last_move.__str__() + ":", add)
        sum += add
    end

    return sum
end

function result(game::Game, legal_moves::Int)::String
    if legal_moves == 0
        if is_check(game.position, game.player)
            return game.player ? RESULT_BLACK : RESULT_WHITE
        end
        return RESULT_STALEMATE
    end

    if game.fifty_move_counter >= 100
        return RESULT_FIFTY_MOVE_RULE
    end

    for count in values(game.position_counts)
        if count >= 3
            return RESULT_REPITITION
        end
    end

    if is_dead(game.position)
        return RESULT_DEAD_POSITION
    end

    return ""
end

function game_from_fen(fen::String)::Game
    fen_parts = split(fen, " ")
    pieces = Dict(
        'K'=>0x0000_0000_0000_0000,
        'Q'=>0x0000_0000_0000_0000,
        'R'=>0x0000_0000_0000_0000,
        'B'=>0x0000_0000_0000_0000,
        'N'=>0x0000_0000_0000_0000,
        'P'=>0x0000_0000_0000_0000,
        'k'=>0x0000_0000_0000_0000,
        'q'=>0x0000_0000_0000_0000,
        'r'=>0x0000_0000_0000_0000,
        'b'=>0x0000_0000_0000_0000,
        'n'=>0x0000_0000_0000_0000,
        'p'=>0x0000_0000_0000_0000,
    )

    board = split(fen_parts[1], "/")
    for (rank_index, rank) in enumerate(board)
        file_index = 1
        while rank != ""
            piece = rank[1]
            if isdigit(piece)
                file_index += parse(Int, piece)
            else
                square = SQUARES[(rank_index - 1) * 8 + file_index]
                pieces[piece] = pieces[piece] | square
                file_index += 1
            end
            rank = rank[2:length(rank)]
        end
    end

    en_passant_square = 0x0000_0000_0000_0000
    for (square, human) in pairs(SQUARES_TO_HUMAN)
        if human == fen_parts[4]
            en_passant_square = square
        end
    end

    return new_game(
        new_position(
            pieces['K'], pieces['Q'], pieces['R'], pieces['B'], pieces['N'], pieces['P'],
            pieces['k'], pieces['q'], pieces['r'], pieces['b'], pieces['n'], pieces['p'],
        ),
        fen_parts[2] == "w",
        new_move(false, 'K', 0x0000_0000_0000_0000, 0x0000_0000_0000_0000),
        Dict(
            'K'=>occursin("K", fen_parts[3]),
            'Q'=>occursin("Q", fen_parts[3]),
            'k'=>occursin("k", fen_parts[3]),
            'q'=>occursin("q", fen_parts[3]),
        ),
        en_passant_square,
        Dict{String,Int}(),
        parse(Int, fen_parts[6]),
        parse(Int, fen_parts[5])
    )
end

function perft()
    game = game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    println(count_legal_moves(game, depth=4))
    # for i in 1:100
    #     game = game_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    #     moves = legal_moves(game)
    #     res = result(game, length(moves))
    #     while res == ""
    #         game = do_move(game, moves[rand(1:length(moves))])
    #         moves = legal_moves(game)
    #         res = result(game, length(moves))
    #     end
    #     println(i, " ", res)
    # end
end

@time perft()
@time perft()

end
