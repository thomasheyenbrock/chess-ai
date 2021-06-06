from typing import List

from py.game import Game


def get_activations_from_bitboard(bitboard: int) -> List[int]:
    return [
        0 if (bitboard & 0x8000_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x4000_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x2000_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x1000_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0800_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0400_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0200_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0100_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0080_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0040_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0020_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0010_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0008_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0004_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0002_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0001_0000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_8000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_4000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_2000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_1000_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0800_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0400_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0200_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0100_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0080_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0040_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0020_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0010_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0008_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0004_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0002_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0001_0000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_8000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_4000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_2000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_1000_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0800_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0400_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0200_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0100_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0080_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0040_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0020_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0010_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0008_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0004_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0002_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0001_0000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_8000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_4000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_2000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_1000) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0800) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0400) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0200) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0100) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0080) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0040) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0020) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0010) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0008) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0004) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0002) == 0 else 1,
        0 if (bitboard & 0x0000_0000_0000_0001) == 0 else 1,
    ]


def get_input_for_game(game: Game) -> List[int]:
    # player
    input = [
        1 if game.player else 0,
    ]
    # position
    input += get_activations_from_bitboard(game.position.pieces["K"])
    input += get_activations_from_bitboard(game.position.pieces["Q"])
    input += get_activations_from_bitboard(game.position.pieces["R"])
    input += get_activations_from_bitboard(game.position.pieces["B"])
    input += get_activations_from_bitboard(game.position.pieces["N"])
    input += get_activations_from_bitboard(game.position.pieces["P"])
    input += get_activations_from_bitboard(game.position.pieces["k"])
    input += get_activations_from_bitboard(game.position.pieces["q"])
    input += get_activations_from_bitboard(game.position.pieces["r"])
    input += get_activations_from_bitboard(game.position.pieces["b"])
    input += get_activations_from_bitboard(game.position.pieces["n"])
    input += get_activations_from_bitboard(game.position.pieces["p"])
    # castles
    input += [
        1 if game.possible_castles["K"] else 0,
        1 if game.possible_castles["Q"] else 0,
        1 if game.possible_castles["k"] else 0,
        1 if game.possible_castles["q"] else 0,
    ]
    # en passant
    input += get_activations_from_bitboard(game.en_passant_square)
    return input
