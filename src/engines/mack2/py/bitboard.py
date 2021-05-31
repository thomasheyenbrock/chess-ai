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


def get_moveable_sqares_to_left(
    all_pieces: int, enemy_pieces: int, observing_pieces: int
) -> int:
    change_mask = 0xFFFFFFFFFFFFFFFF
    to_left = observing_pieces
    moveable_squares = observing_pieces
    c = 0
    while to_left != 0 and change_mask != 0:
        if c > 100:
            raise Exception("infinite loop")
        c += 1

        to_left = get_left_square(to_left)
        change_mask = ((to_left & all_pieces) ^ to_left) & get_left_square(change_mask)
        moveable_squares = moveable_squares ^ change_mask
    return (
        observing_pieces
        | moveable_squares
        | get_left_square(get_right_square(enemy_pieces) & moveable_squares)
    )


def get_moveable_sqares_to_right(
    all_pieces: int,
    enemy_pieces: int,
    observing_pieces: int,
) -> int:
    change_mask = 0xFFFFFFFFFFFFFFFF
    to_right = observing_pieces
    moveable_squares = observing_pieces
    c = 0
    while to_right != 0 and change_mask != 0:
        if c > 100:
            raise Exception("infinite loop")
        c += 1

        to_right = get_right_square(to_right)
        change_mask = ((to_right & all_pieces) ^ to_right) & get_right_square(
            change_mask
        )
        moveable_squares = moveable_squares ^ change_mask
    return (
        observing_pieces
        | moveable_squares
        | get_right_square(get_left_square(enemy_pieces) & moveable_squares)
    )


def get_moveable_sqares_to_top(
    all_pieces: int, enemy_pieces: int, observing_pieces: int
) -> int:
    change_mask = 0xFFFFFFFFFFFFFFFF
    to_top = observing_pieces
    moveable_squares = observing_pieces
    c = 0
    while to_top != 0 and change_mask != 0:
        if c > 100:
            raise Exception("infinite loop")
        c += 1

        to_top = get_top_square(to_top)
        change_mask = ((to_top & all_pieces) ^ to_top) & get_top_square(change_mask)
        moveable_squares = moveable_squares ^ change_mask
    return (
        observing_pieces
        | moveable_squares
        | get_top_square(get_bottom_square(enemy_pieces) & moveable_squares)
    )


def get_moveable_sqares_to_bottom(
    all_pieces: int, enemy_pieces: int, observing_pieces: int
) -> int:
    change_mask = 0xFFFFFFFFFFFFFFFF
    to_bottom = observing_pieces
    moveable_squares = observing_pieces
    c = 0
    while to_bottom != 0 and change_mask != 0:
        if c > 100:
            raise Exception("infinite loop")
        c += 1

        to_bottom = get_bottom_square(to_bottom)
        change_mask = ((to_bottom & all_pieces) ^ to_bottom) & get_bottom_square(
            change_mask
        )
        moveable_squares = moveable_squares ^ change_mask
    return (
        observing_pieces
        | moveable_squares
        | get_bottom_square(get_top_square(enemy_pieces) & moveable_squares)
    )


def get_moveable_sqares_to_top_left(
    all_pieces: int, enemy_pieces: int, observing_pieces: int
) -> int:
    change_mask = 0xFFFFFFFFFFFFFFFF
    to_top_left = observing_pieces
    moveable_squares = observing_pieces
    c = 0
    while to_top_left != 0 and change_mask != 0:
        if c > 100:
            raise Exception("infinite loop")
        c += 1

        to_top_left = get_top_square(get_left_square(to_top_left))
        change_mask = ((to_top_left & all_pieces) ^ to_top_left) & get_top_square(
            get_left_square(change_mask)
        )
        moveable_squares = moveable_squares ^ change_mask
    return (
        observing_pieces
        | moveable_squares
        | get_top_square(
            get_left_square(
                get_bottom_square(get_right_square(enemy_pieces)) & moveable_squares
            )
        )
    )


def get_moveable_sqares_to_top_right(
    all_pieces: int, enemy_pieces: int, observing_pieces: int
) -> int:
    change_mask = 0xFFFFFFFFFFFFFFFF
    to_top_right = observing_pieces
    moveable_squares = observing_pieces
    c = 0
    while to_top_right != 0 and change_mask != 0:
        if c > 100:
            raise Exception("infinite loop")
        c += 1

        to_top_right = get_top_square(get_right_square(to_top_right))
        change_mask = ((to_top_right & all_pieces) ^ to_top_right) & get_top_square(
            get_right_square(change_mask)
        )
        moveable_squares = moveable_squares ^ change_mask
    return (
        observing_pieces
        | moveable_squares
        | get_top_square(
            get_right_square(
                get_bottom_square(get_left_square(enemy_pieces)) & moveable_squares
            )
        )
    )


def get_moveable_sqares_to_bottom_left(
    all_pieces: int, enemy_pieces: int, observing_pieces: int
) -> int:
    change_mask = 0xFFFFFFFFFFFFFFFF
    to_bottom_left = observing_pieces
    moveable_squares = observing_pieces
    c = 0
    while to_bottom_left != 0 and change_mask != 0:
        if c > 100:
            raise Exception("infinite loop")
        c += 1

        to_bottom_left = get_bottom_square(get_left_square(to_bottom_left))
        change_mask = (
            (to_bottom_left & all_pieces) ^ to_bottom_left
        ) & get_bottom_square(get_left_square(change_mask))
        moveable_squares = moveable_squares ^ change_mask
    return (
        observing_pieces
        | moveable_squares
        | get_bottom_square(
            get_left_square(
                get_top_square(get_right_square(enemy_pieces)) & moveable_squares
            )
        )
    )


def get_moveable_sqares_to_bottom_right(
    all_pieces: int, enemy_pieces: int, observing_pieces: int
) -> int:
    change_mask = 0xFFFFFFFFFFFFFFFF
    to_bottom_right = observing_pieces
    moveable_squares = observing_pieces
    c = 0
    while to_bottom_right != 0 and change_mask != 0:
        if c > 100:
            raise Exception("infinite loop")
        c += 1

        to_bottom_right = get_bottom_square(get_right_square(to_bottom_right))
        change_mask = (
            (to_bottom_right & all_pieces) ^ to_bottom_right
        ) & get_bottom_square(get_right_square(change_mask))
        moveable_squares = moveable_squares ^ change_mask
    return (
        observing_pieces
        | moveable_squares
        | get_bottom_square(
            get_right_square(
                get_top_square(get_left_square(enemy_pieces)) & moveable_squares
            )
        )
    )
