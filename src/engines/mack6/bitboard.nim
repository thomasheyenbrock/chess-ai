iterator split*(bitboard: uint64): uint64 =
    if bitboard != 0:
        if (bitboard and (bitboard - 1)) == 0:
            # It's a power of two
            yield bitboard
        else:
            var c = 0
            var bb = bitboard
            while bb != 0:
                let bit = bb and 0x0000_0000_0000_0001'u64
                if bit != 0:
                    yield bit shl c
                c += 1
                bb = bb shr 1


iterator get_activations*(bitboard: uint64): float32 =
    var c = 0x8000_0000_0000_0000'u64
    for i in 0..63:
        yield float32(bitboard and c)
        c = c shr 1


proc get_left_square*(bitboard: uint64): uint64 =
    return (bitboard and 0x7F7F_7F7F_7F7F_7F7F'u64) shl 1


proc get_right_square*(bitboard: uint64): uint64 =
    return (bitboard and 0xFEFE_FEFE_FEFE_FEFE'u64) shr 1


proc get_top_square*(bitboard: uint64): uint64 =
    return (bitboard shl 8) and 0xFFFF_FFFF_FFFF_FFFF'u64


proc get_bottom_square*(bitboard: uint64): uint64 =
    return bitboard shr 8
