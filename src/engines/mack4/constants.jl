module Constants

include("./bitboard.jl")

using .Bitboard: get_top_square, get_bottom_square, get_left_square, get_right_square

export NORTH_RAY,
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

NORTH_RAY = Dict{UInt64, UInt64}()
SOUTH_RAY = Dict{UInt64, UInt64}()
WEST_RAY = Dict{UInt64, UInt64}()
EAST_RAY = Dict{UInt64, UInt64}()
NORTH_WEST_RAY = Dict{UInt64, UInt64}()
NORTH_EAST_RAY = Dict{UInt64, UInt64}()
SOUTH_WEST_RAY = Dict{UInt64, UInt64}()
SOUTH_EAST_RAY = Dict{UInt64, UInt64}()

NORTH_MOVES = Dict{UInt64, Dict{UInt64, UInt64}}()
SOUTH_MOVES = Dict{UInt64, Dict{UInt64, UInt64}}()
WEST_MOVES = Dict{UInt64, Dict{UInt64, UInt64}}()
EAST_MOVES = Dict{UInt64, Dict{UInt64, UInt64}}()
NORTH_WEST_MOVES = Dict{UInt64, Dict{UInt64, UInt64}}()
NORTH_EAST_MOVES = Dict{UInt64, Dict{UInt64, UInt64}}()
SOUTH_WEST_MOVES = Dict{UInt64, Dict{UInt64, UInt64}}()
SOUTH_EAST_MOVES = Dict{UInt64, Dict{UInt64, UInt64}}()

NORTH_ATTACKS = Dict{UInt64, Dict{UInt64, UInt64}}()
SOUTH_ATTACKS = Dict{UInt64, Dict{UInt64, UInt64}}()
WEST_ATTACKS = Dict{UInt64, Dict{UInt64, UInt64}}()
EAST_ATTACKS = Dict{UInt64, Dict{UInt64, UInt64}}()
NORTH_WEST_ATTACKS = Dict{UInt64, Dict{UInt64, UInt64}}()
NORTH_EAST_ATTACKS = Dict{UInt64, Dict{UInt64, UInt64}}()
SOUTH_WEST_ATTACKS = Dict{UInt64, Dict{UInt64, UInt64}}()
SOUTH_EAST_ATTACKS = Dict{UInt64, Dict{UInt64, UInt64}}()

KING_MOVES = Dict{UInt64, UInt64}()
KNIGHT_MOVES = Dict{UInt64, UInt64}()
PAWN_ATTACKS = Dict{Bool, Dict{UInt64, UInt64}}(
    true=>Dict{UInt64, UInt64}(),
    false=>Dict{UInt64, UInt64}()
)
PAWN_SINGLE_MOVES = Dict{Bool, Dict{UInt64, UInt64}}(
    true=>Dict{UInt64, UInt64}(),
    false=>Dict{UInt64, UInt64}()
)
PAWN_DOUBLE_MOVES = Dict{Bool, Dict{UInt64, UInt64}}(
    true=>Dict{UInt64, UInt64}(),
    false=>Dict{UInt64, UInt64}()
)
PAWN_ATTACK_MOVES = Dict{Bool, Dict{UInt64, Vector{UInt64}}}(
    true=>Dict{UInt64, Vector{UInt64}}(),
    false=>Dict{UInt64, Vector{UInt64}}()
)
PAWN_EN_PASSANT_CAPTURES = Dict{Bool, Dict{UInt64, UInt64}}(
    true=>Dict{UInt64, UInt64}(),
    false=>Dict{UInt64, UInt64}()
)

function generate_possibilities(current::UInt64, direction)::Vector{UInt64}
    forward = direction(current)
    possibilities = fill(UInt64(0), 1)
    if forward != 0
        append!(possibilities, forward)
    end

    while forward != 0
        forward = direction(forward)
        new_possibilities = Vector{UInt64}()
        for p in possibilities
            if forward != 0
                append!(new_possibilities, p | forward)
            end
        end
        possibilities = cat(possibilities, new_possibilities, dims=(1))
    end
    return possibilities
end

function get_top_left_square(bitboard::UInt64)::UInt64
    return get_top_square(get_left_square(bitboard))
end

function get_top_right_square(bitboard::UInt64)::UInt64
    return get_top_square(get_right_square(bitboard))
end

function get_bottom_left_square(bitboard::UInt64)::UInt64
    return get_bottom_square(get_left_square(bitboard))
end

function get_bottom_right_square(bitboard::UInt64)::UInt64
    return get_bottom_square(get_right_square(bitboard))
end

for rank in 0:7
    for file in 0:7
        square = UInt64(2 ^ (8 * rank + file) - 1) + 1

        top = get_top_square(square)
        bottom = get_bottom_square(square)
        left = get_left_square(square)
        right = get_right_square(square)
        top_left = get_left_square(top)
        top_right = get_right_square(top)
        bottom_left = get_left_square(bottom)
        bottom_right = get_right_square(bottom)

        NORTH_RAY[square] = 0x0000_0000_0000_0000
        NORTH_MOVES[square] = Dict{UInt64, UInt64}()
        NORTH_ATTACKS[square] = Dict(0x0000_0000_0000_0000=>0x0000_0000_0000_0000)
        current = top
        carry = 0x0000_0000_0000_0000
        while current != 0
            NORTH_RAY[square] |= current
            for p in generate_possibilities(current, get_top_square)
                NORTH_MOVES[square][p | current] = carry
                NORTH_ATTACKS[square][p | current] = current
            end
            carry |= current
            current = get_top_square(current)
        end
        NORTH_MOVES[square][0] = carry

        SOUTH_RAY[square] = 0x0000_0000_0000_0000
        SOUTH_MOVES[square] = Dict{UInt64, UInt64}()
        SOUTH_ATTACKS[square] = Dict(0x0000_0000_0000_0000=>0x0000_0000_0000_0000)
        current = bottom
        carry = 0x0000_0000_0000_0000
        while current != 0
            SOUTH_RAY[square] |= current
            for p in generate_possibilities(current, get_bottom_square)
                SOUTH_MOVES[square][p | current] = carry
                SOUTH_ATTACKS[square][p | current] = current
            end
            carry |= current
            current = get_bottom_square(current)
        end
        SOUTH_MOVES[square][0] = carry

        WEST_RAY[square] = 0x0000_0000_0000_0000
        WEST_MOVES[square] = Dict{UInt64, UInt64}()
        WEST_ATTACKS[square] = Dict(0x0000_0000_0000_0000=>0x0000_0000_0000_0000)
        current = left
        carry = 0x0000_0000_0000_0000
        while current != 0
            WEST_RAY[square] |= current
            for p in generate_possibilities(current, get_left_square)
                WEST_MOVES[square][p | current] = carry
                WEST_ATTACKS[square][p | current] = current
            end
            carry |= current
            current = get_left_square(current)
        end
        WEST_MOVES[square][0] = carry

        EAST_RAY[square] = 0x0000_0000_0000_0000
        EAST_MOVES[square] = Dict{UInt64, UInt64}()
        EAST_ATTACKS[square] = Dict(0x0000_0000_0000_0000=>0x0000_0000_0000_0000)
        current = right
        carry = 0x0000_0000_0000_0000
        while current != 0
            EAST_RAY[square] |= current
            for p in generate_possibilities(current, get_right_square)
                EAST_MOVES[square][p | current] = carry
                EAST_ATTACKS[square][p | current] = current
            end
            carry |= current
            current = get_right_square(current)
        end
        EAST_MOVES[square][0] = carry

        NORTH_WEST_RAY[square] = 0x0000_0000_0000_0000
        NORTH_WEST_MOVES[square] = Dict{UInt64, UInt64}()
        NORTH_WEST_ATTACKS[square] = Dict(0x0000_0000_0000_0000=>0x0000_0000_0000_0000)
        current = top_left
        carry = 0x0000_0000_0000_0000
        while current != 0
            NORTH_WEST_RAY[square] |= current
            for p in generate_possibilities(current, get_top_left_square)
                NORTH_WEST_MOVES[square][p | current] = carry
                NORTH_WEST_ATTACKS[square][p | current] = current
            end
            carry |= current
            current = get_top_square(get_left_square(current))
        end
        NORTH_WEST_MOVES[square][0] = carry

        NORTH_EAST_RAY[square] = 0x0000_0000_0000_0000
        NORTH_EAST_MOVES[square] = Dict{UInt64, UInt64}()
        NORTH_EAST_ATTACKS[square] = Dict(0x0000_0000_0000_0000=>0x0000_0000_0000_0000)
        current = top_right
        carry = 0x0000_0000_0000_0000
        while current != 0
            NORTH_EAST_RAY[square] |= current
            for p in generate_possibilities(current, get_top_right_square)
                NORTH_EAST_MOVES[square][p | current] = carry
                NORTH_EAST_ATTACKS[square][p | current] = current
            end
            carry |= current
            current = get_top_square(get_right_square(current))
        end
        NORTH_EAST_MOVES[square][0] = carry

        SOUTH_WEST_RAY[square] = 0x0000_0000_0000_0000
        SOUTH_WEST_MOVES[square] = Dict{UInt64, UInt64}()
        SOUTH_WEST_ATTACKS[square] = Dict(0x0000_0000_0000_0000=>0x0000_0000_0000_0000)
        current = bottom_left
        carry = 0x0000_0000_0000_0000
        while current != 0
            SOUTH_WEST_RAY[square] |= current
            for p in generate_possibilities(current, get_bottom_left_square)
                SOUTH_WEST_MOVES[square][p | current] = carry
                SOUTH_WEST_ATTACKS[square][p | current] = current
            end
            carry |= current
            current = get_bottom_square(get_left_square(current))
        end
        SOUTH_WEST_MOVES[square][0] = carry

        SOUTH_EAST_RAY[square] = 0x0000_0000_0000_0000
        SOUTH_EAST_MOVES[square] = Dict{UInt64, UInt64}()
        SOUTH_EAST_ATTACKS[square] = Dict(0x0000_0000_0000_0000=>0x0000_0000_0000_0000)
        current = bottom_right
        carry = 0x0000_0000_0000_0000
        while current != 0
            SOUTH_EAST_RAY[square] |= current
            for p in generate_possibilities(current, get_bottom_right_square)
                SOUTH_EAST_MOVES[square][p | current] = carry
                SOUTH_EAST_ATTACKS[square][p | current] = current
            end
            carry |= current
            current = get_bottom_square(get_right_square(current))
        end
        SOUTH_EAST_MOVES[square][0] = carry

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

        PAWN_ATTACKS[true][square] = bottom_left | bottom_right
        PAWN_ATTACKS[false][square] = top_left | top_right

        PAWN_SINGLE_MOVES[true][square] = top
        PAWN_SINGLE_MOVES[false][square] = bottom

        if square & 0x0000_0000_0000_FF00 == 0
            PAWN_DOUBLE_MOVES[true][square] = 0
        else
            PAWN_DOUBLE_MOVES[true][square] = top2
        end

        if square & 0x00FF_0000_0000_0000 == 0
            PAWN_DOUBLE_MOVES[false][square] = 0
        else
            PAWN_DOUBLE_MOVES[false][square] = bottom2
        end

        PAWN_ATTACK_MOVES[true][square] = Vector{UInt64}()
        if top_left != 0
            append!(PAWN_ATTACK_MOVES[true][square], top_left)
        end
        if top_right != 0
            append!(PAWN_ATTACK_MOVES[true][square], top_right)
        end

        PAWN_ATTACK_MOVES[false][square] = Vector{UInt64}()
        if bottom_left != 0
            append!(PAWN_ATTACK_MOVES[false][square], bottom_left)
        end
        if bottom_right != 0
            append!(PAWN_ATTACK_MOVES[false][square], bottom_right)
        end

        if square & 0x0000_00FF_0000_0000 == 0
            PAWN_EN_PASSANT_CAPTURES[true][square] = 0x0000_0000_0000_0000
        else
            PAWN_EN_PASSANT_CAPTURES[true][square] = top_left | top_right
        end

        if square & 0x0000_0000_FF00_0000 == 0
            PAWN_EN_PASSANT_CAPTURES[false][square] = 0x0000_0000_0000_0000
        else
            PAWN_EN_PASSANT_CAPTURES[false][square] = bottom_left | bottom_right
        end
    end
end

end
