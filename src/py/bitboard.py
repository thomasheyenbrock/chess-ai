from typing import Iterable


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
    0x0000_0000_0000_0001,
]


def split(bitboard: int) -> Iterable[int]:
    c = 0
    while bitboard != 0:
        bit = bitboard & 1
        if bit:
            yield bit << c
        c += 1
        bitboard = bitboard >> 1


def get_left_square(bitboard: int) -> int:
    return (bitboard & 0x7F7F7F7F7F7F7F7F) << 1


def get_right_square(bitboard: int) -> int:
    return (bitboard & 0xFEFEFEFEFEFEFEFE) >> 1


def get_top_square(bitboard: int) -> int:
    return (bitboard << 8) & 0xFFFFFFFFFFFFFFFF


def get_bottom_square(bitboard: int) -> int:
    return bitboard >> 8