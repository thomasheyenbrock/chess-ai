module Bitboard

    function split_bitboard(bitboard::UInt64)::Vector{UInt64}
        if bitboard === UInt64(0)
            return Vector{UInt64}()
        end

        if (bitboard & (bitboard - 1)) === UInt64(0)
            # It's a power of two
            return fill(bitboard, 1)
        end

        result = Vector{UInt64}()
        c = 0
        while bitboard !== UInt64(0)
            bit = bitboard & 1
            if bit !== UInt64(0)
                append!(result, bit << c)
            end
            c += 1
            bitboard = bitboard >>> 1
        end
        return result
    end

    function get_left_square(bitboard::UInt64)::UInt64
        return (bitboard & 0x7F7F_7F7F_7F7F_7F7F) << 1
    end

    function get_right_square(bitboard::UInt64)::UInt64
        return (bitboard & 0xFEFE_FEFE_FEFE_FEFE) >>> 1
    end

    function get_top_square(bitboard::UInt64)::UInt64
        return (bitboard << 8) & 0xFFFF_FFFF_FFFF_FFFF
    end

    function get_bottom_square(bitboard::UInt64)::UInt64
        return bitboard >> 8
    end

end
