from typing import List


def split(bitboard: int) -> List[int]:
    single_bits: List[int] = []
    c = 0
    current = bitboard
    while current != 0:
        bit = current & 0x00000001
        if bit:
            single_bits += [bit << c]
        c += 1
        current = current >> 1

    return single_bits


def get_left_square(bitboard: int) -> int:
    return (bitboard & 0x7F7F7F7F7F7F7F7F) << 1


def get_right_square(bitboard: int) -> int:
    return (bitboard & 0xFEFEFEFEFEFEFEFE) >> 1


def get_top_square(bitboard: int) -> int:
    return (bitboard << 8) & 0xFFFFFFFFFFFFFFFF


def get_bottom_square(bitboard: int) -> int:
    return bitboard >> 8
